use rmcp::handler::server::wrapper::Parameters;
use rmcp::tool_handler;
use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::router::tool::ToolRouter, model::*,
    schemars, tool, tool_router,
};
use serde::Deserialize;

use crate::db::models::Memories;
use crate::db::{
    cleanup_expired_sessions, delete_session, generate_reflection, get_connection,
    get_reflection_by_session, get_session_context, list_sessions, search_memories, start_session,
    store_memory,
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
    #[tool(description = "Guarda una nueva observación (memoria) en la sesión actual")]
    pub async fn memory_store(
        &self,
        Parameters(p): Parameters<MemoryStoreParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let memory = Memories {
            memory_id: uuid::Uuid::new_v4().to_string(),
            session_id: p.session_id,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            title: p.title,
            r#type: p.r#type,
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

    #[tool(description = "Busca memorias por texto completo")]
    pub async fn memory_search(
        &self,
        Parameters(p): Parameters<MemorySearchParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

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

    #[tool(description = "Inicia una nueva sesión o reanuda una existente")]
    pub async fn memory_session_start(
        &self,
        Parameters(p): Parameters<SessionStartParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

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

    #[tool(description = "Lista todas las sesiones de un proyecto")]
    pub async fn memory_list_sessions(
        &self,
        Parameters(p): Parameters<ProjectParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let sessions = list_sessions(&conn, &p.project)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let count = sessions.len();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "sessions": sessions, "count": count }).to_string(),
        )]))
    }

    #[tool(description = "Obtiene el contexto reciente de una sesión")]
    pub async fn memory_context(
        &self,
        Parameters(p): Parameters<SessionContextParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let memories = get_session_context(&conn, &p.session_id, p.limit.unwrap_or(5))
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let count = memories.len();
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "context": memories, "count": count }).to_string(),
        )]))
    }

    #[tool(description = "Genera una reflexión consolidada de una sesión")]
    pub async fn memory_reflect(
        &self,
        Parameters(p): Parameters<SessionIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let reflection = generate_reflection(&conn, &p.session_id)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "success": true,
                "reflection_id": reflection.reflection_id,
                "content": reflection.content,
                "type": reflection.r#type,
                "importance": reflection.importance,
                "level": reflection.level
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Obtiene la reflexión de una sesión")]
    pub async fn memory_get_reflection(
        &self,
        Parameters(p): Parameters<SessionIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

        match get_reflection_by_session(&conn, &p.session_id)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?
        {
            Some(reflection) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::json!({ "exists": true, "reflection": reflection }).to_string(),
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

    #[tool(description = "Elimina una sesión")]
    pub async fn memory_delete_session(
        &self,
        Parameters(p): Parameters<DeleteSessionParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let deleted = delete_session(&conn, &p.session_id, p.permanent.unwrap_or(false))
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "success": deleted, "session_id": p.session_id }).to_string(),
        )]))
    }

    #[tool(description = "Limpia sesiones expiradas de un proyecto")]
    pub async fn memory_cleanup(
        &self,
        Parameters(p): Parameters<CleanupParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = get_connection().map_err(|e| McpError::internal_error(e.to_string(), None))?;

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
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Servidor de memoria persistente para sesiones de trabajo con IA.".to_string(),
            ),
        }
    }
}
