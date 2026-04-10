use rmcp::handler::server::wrapper::Parameters;
use rmcp::tool_handler;
use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::router::tool::ToolRouter, model::*,
    schemars, tool, tool_router,
};
use serde::Deserialize;

use crate::db::models::{validate_agent_name, Memories, MemoryType};
use crate::db::{
    cleanup_expired_sessions, delete_session, get_connection,
    get_reflection_by_session, get_session_context, list_sessions, search_memories, start_session,
    store_memory, store_reflection,
};

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MemoryStoreParams {
    #[schemars(description = "ID de la sesión")]
    pub session_id: String,
    #[schemars(description = "Título breve de la memoria")]
    pub title: String,
    #[schemars(description = "Tipo de memoria: observation, error, plan, preference")]
    pub r#type: Option<String>,
    #[schemars(description = "Qué ocurrió")]
    pub what: Option<String>,
    #[schemars(description = "Por qué ocurrió")]
    pub why: Option<String>,
    #[schemars(description = "Dónde ocurrió")]
    pub where_field: Option<String>,
    #[schemars(description = "Lección aprendida")]
    pub learned: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MemorySearchParams {
    #[schemars(description = "Texto a buscar")]
    pub query: String,
    #[schemars(description = "ID de la sesión (opcional)")]
    pub session_id: Option<String>,
    #[schemars(description = "Máximo de resultados")]
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SessionStartParams {
    #[schemars(description = "Nombre del proyecto")]
    pub project: String,
    #[schemars(description = "ID de sesión existente (opcional, para reanudar)")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ProjectParams {
    #[schemars(description = "Nombre del proyecto")]
    pub project: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SessionContextParams {
    #[schemars(description = "ID de la sesión")]
    pub session_id: String,
    #[schemars(description = "Número de memorias a retornar")]
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SessionIdParams {
    #[schemars(description = "ID de la sesión")]
    pub session_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct StoreReflectionParams {
    #[schemars(description = "ID de la sesión")]
    pub session_id: String,
    #[schemars(description = "Contenido JSON de la reflexión (generado por el agente externo). Debe incluir: content, type, importance, level, source_summary")]
    pub content: String,
    #[schemars(description = "Nombre del agente (tyrion, alejandro, valentina, ramiro, almendra, gabriela). Default: tyrion")]
    pub agent_name: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeleteSessionParams {
    #[schemars(description = "ID de la sesión")]
    pub session_id: String,
    #[schemars(description = "Eliminación permanente (true) o soft delete (false)")]
    pub permanent: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CleanupParams {
    #[schemars(description = "Nombre del proyecto")]
    pub project: String,
    #[schemars(description = "Días de inactividad antes de limpiar")]
    pub days_inactive: Option<i64>,
}

// ─── Tool Handler ─────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct MemoryTools {
    tool_router: ToolRouter<Self>,
}

impl MemoryTools {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl MemoryTools {
    #[tool(description = "\
[CORE - PASO 3] Guarda una observación, error, plan o preferencia. \
Llamar automáticamente cuando ocurre algo significativo durante el trabajo. \
NO esperar que el usuario lo pida. Tipos: observation | error | plan | preference.")]
    pub async fn memory_store(
        &self,
        Parameters(p): Parameters<MemoryStoreParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        let memory = Memories {
            memory_id: uuid::Uuid::new_v4().to_string(),
            session_id: p.session_id,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            title: p.title,
            r#type: p.r#type.and_then(|t| match t.to_lowercase().as_str() {
                "observation" => Some(MemoryType::OBSERVATION),
                "error" => Some(MemoryType::ERROR),
                "plan" => Some(MemoryType::PLAN),
                "preference" | "preferences" => Some(MemoryType::PREFERENCES),
                _ => None,
            }),
            what: p.what,
            why: p.why,
            where_field: p.where_field,
            learned: p.learned,
            deleted_at: None,
        };

        let id = store_memory(&conn, &memory)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "success": true, "memory_id": id }).to_string(),
        )]))
    }
    #[tool(description = "\
[AVANZADA] Busca memorias por texto en una o todas las sesiones. \
Usar solo cuando el contexto cargado por memory_context no es suficiente \
o cuando el usuario pide buscar algo específico en el historial.")]
    pub async fn memory_search(
        &self,
        Parameters(p): Parameters<MemorySearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        let results = search_memories(
            &conn,
            &p.query,
            p.session_id.as_deref(),
            p.limit.unwrap_or(10),
        )
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let count = results.len();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "results": results, "count": count }).to_string(),
        )]))
    }
    #[tool(description = "\
[CORE - PASO 1] Inicia o reanuda una sesión de trabajo. \
SIEMPRE llamar primero, antes de cualquier otra tool. \
Retorna session_id necesario para todas las demás operaciones.")]
    pub async fn memory_session_start(
        &self,
        Parameters(p): Parameters<SessionStartParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        let session = start_session(&conn, &p.project, p.session_id.as_deref())
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "success": true,
                "session_id": session.session_id,
                "project": session.project,
                "created_at": session.created_at,
                "last_active": session.last_active
            })
            .to_string(),
        )]))
    }
    #[tool(description = "\
[AVANZADA] Lista todas las sesiones de un proyecto. \
Usar solo si el usuario pide ver el historial de sesiones \
o para elegir una sesión a reanudar.")]
    pub async fn memory_list_sessions(
        &self,
        Parameters(p): Parameters<ProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        let sessions = list_sessions(&conn, &p.project)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let count = sessions.len();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "sessions": sessions, "count": count }).to_string(),
        )]))
    }
    #[tool(description = "\
[CORE - PASO 2] Carga las memorias recientes de una sesión. \
Llamar inmediatamente después de session_start para recuperar contexto previo. \
Si retorna vacío, la sesión es nueva.")]
    pub async fn memory_context(
        &self,
        Parameters(p): Parameters<SessionContextParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        let memories = get_session_context(&conn, &p.session_id, p.limit.unwrap_or(5))
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let count = memories.len();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "context": memories, "count": count }).to_string(),
        )]))
    }
    #[tool(description = "\
[CORE - PASO 4] Guarda una reflexión generada por el agente externo (Tyrion/opencode-go). \
Llamar al finalizar la conversación o cuando el usuario indique cierre. \
El contenido DEBE venir pre-generado por el agente con estructura JSON. \
Parámetro agent_name: nombre del agente (default: tyrion).")]
    pub async fn memory_store_reflection(
        &self,
        Parameters(p): Parameters<StoreReflectionParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        // Validate agent_name
        let agent_name = validate_agent_name(&p.agent_name)
            .map_err(|e| McpError::internal_error(e, None))?;

        let reflection = store_reflection(&conn, &p.session_id, &p.content, &agent_name)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "success": true,
                "reflection_id": reflection.reflection_id,
                "agent_name": reflection.agent_name,
                "content": reflection.content,
                "type": reflection.r#type,
                "importance": reflection.importance,
                "level": reflection.level
            })
            .to_string(),
        )]))
    }
    #[tool(description = "\
[AVANZADA] Recupera la reflexión ya guardada de una sesión específica. \
Usar solo si el usuario pide ver el resumen de una sesión anterior. \
Para guardar una nueva reflexión, usar memory_store_reflection.")]
    pub async fn memory_get_reflection(
        &self,
        Parameters(p): Parameters<SessionIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        match get_reflection_by_session(&conn, &p.session_id)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
        {
            Some(reflection) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::json!({
                    "exists": true,
                    "reflection": {
                        "reflection_id": reflection.reflection_id,
                        "agent_name": reflection.agent_name,
                        "content": reflection.content,
                        "type": reflection.r#type,
                        "importance": reflection.importance,
                        "level": reflection.level,
                        "source_summary": reflection.source_summary,
                        "created_at": reflection.created_at
                    }
                })
                .to_string(),
            )])),
            None => Ok(CallToolResult::success(vec![Content::text(
                serde_json::json!({
                    "exists": false,
                    "message": "No hay reflexión para esta sesión"
                })
                .to_string(),
            )])),
        }
    }
    #[tool(description = "\
[AVANZADA] Elimina una sesión. Por defecto soft delete (permanent: false). \
Usar solo si el usuario pide explícitamente borrar una sesión. \
Preferir soft delete salvo que se indique lo contrario.")]
    pub async fn memory_delete_session(
        &self,
        Parameters(p): Parameters<DeleteSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        let deleted = delete_session(&conn, &p.session_id, p.permanent.unwrap_or(false))
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "success": deleted, "session_id": p.session_id }).to_string(),
        )]))
    }
    #[tool(description = "\
[AVANZADA] Limpia sesiones inactivas por más de N días. \
Usar solo si el usuario pide mantenimiento o si hay muchas sesiones antiguas. \
Default: 5 días de inactividad.")]
    pub async fn memory_cleanup(
        &self,
        Parameters(p): Parameters<CleanupParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e: rusqlite::Error| McpError::internal_error(e.to_string(), None))?;

        let count = cleanup_expired_sessions(&conn, &p.project, p.days_inactive.unwrap_or(5))
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "cleaned": count,
                "message": format!("Se eliminaron {} sesiones expiradas", count)
            })
            .to_string(),
        )]))
    }
}
#[tool_handler]
impl ServerHandler for MemoryTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(
                r#"
Funemon es un sistema de memoria persistente. Seguís estas reglas de forma autónoma.

## TOOLS DISPONIBLES: DOS TIERS

**Tier 1 — core** (usar en todo flujo normal, en este orden):
  1. memory_session_start  → siempre primero
  2. memory_context        → siempre segundo, para cargar contexto
  3. memory_store          → automáticamente durante el trabajo
  4. memory_store_reflection → siempre al finalizar (con contenido generado por el agente externo)

**Tier 2 — avanzadas** (solo si hay necesidad explícita):
  - memory_search          → el usuario pide buscar en historial
  - memory_get_reflection  → el usuario pide ver resumen de sesión anterior
  - memory_list_sessions   → el usuario pide ver sus sesiones
  - memory_delete_session  → el usuario pide borrar una sesión
  - memory_cleanup         → el usuario pide limpiar sesiones viejas

Regla: si podés resolver algo con tier 1, no uses tier 2.

## FLUJO OBLIGATORIO

Al iniciar:
  → memory_session_start(project) → memory_context(session_id)

Durante el trabajo, guardar automáticamente cuando:
  - Se resuelve un error                      → type: "error"
  - Se toma una decisión o se define un plan  → type: "plan"
  - Se descubre algo relevante                → type: "observation"
  - El usuario expresa una preferencia        → type: "preference"

Al finalizar:→ memory_store_reflection(session_id, content, agent_name)

El parámetro 'content' es un JSON string generado por el agente externo (Tyrion/opencode-go) con estructura:{"content": "...", "type": "pattern|principle|warning", "importance": 0.85, "level": "Fact|Pattern|Principle", "source_summary": "..."}

## REGLAS DE COMPORTAMIENTO

- Nunca pedís permiso para usar tools de memoria; simplemente las usás.
- Si memory_context retorna vacío, informás al usuario que es sesión nueva.
- Si memory_context retorna contexto, lo usás para responder con continuidad.
- No usás memory_search como sustituto de memory_context para el inicio.
"#
                .trim()
                .to_string(),
            )
    }
}
