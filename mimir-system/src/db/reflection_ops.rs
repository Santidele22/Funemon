use super::memory_ops::get_session_memories;
use crate::models::{Memory, Reflection};
use crate::reflection::call_ollana;
use rusqlite::{Connection, Result, params};

const CREATE_REFLECTION: &str = "
    INSERT INTO reflections (reflection_id, session_id, content, type, importance, level, source_summary, created_at, deleted_at)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
";
const GET_REFLECTION_BY_SESSION: &str = "
    SELECT reflection_id, session_id, content, type, importance, level, source_summary, created_at, deleted_at
    FROM reflections
    WHERE session_id = ?1 AND deleted_at IS NULL
";

const DELETE_REFLECTION: &str = "
    UPDATE reflections SET deleted_at = ?1 WHERE reflection_id = ?2
";

#[derive(Debug)]
pub struct ParsedReflection {
    pub content: String,
    pub r#type: String, // "pattern", "principle", "warning"
    pub importance: f32,
    pub level: String, // "Fact", "Pattern", "Principle"
    pub source_summary: String,
}

fn clean_json_string(s: &str) -> String {
    let trimmed = s.trim();

    if trimmed.starts_with("```json") {
        let without_start = trimmed.trim_start_matches("```json");
        if let Some(end_pos) = without_start.rfind("```") {
            return without_start[..end_pos].trim().to_string();
        }
        return without_start.trim().to_string();
    }

    if trimmed.starts_with("```") {
        let without_start = trimmed.trim_start_matches("```");
        if let Some(end_pos) = without_start.rfind("```") {
            return without_start[..end_pos].trim().to_string();
        }
        return without_start.trim().to_string();
    }

    trimmed.to_string()
}
pub fn parse_response(response: &str) -> Result<ParsedReflection> {
    let cleaned = clean_json_string(response);

    let json: Value = serde_json::from_str(&cleaned)
        .map_err(|e| RusqliteError::Other(format!("Error parseando JSON: {}", e)))?;

    let content = json
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RusqliteError::Other("Falta campo 'content'".to_string()))?
        .to_string();

    let r#type = json
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RusqliteError::Other("Falta campo 'type'".to_string()))?
        .to_string();

    match r#type.as_str() {
        "pattern" | "principle" | "warning" => {}
        _ => return Err(RusqliteError::Other(format!("Tipo inválido: {}", r#type))),
    }

    let importance = json
        .get("importance")
        .and_then(|v| v.as_f64())
        .ok_or_else(|| RusqliteError::Other("Falta campo 'importance'".to_string()))?
        as f32;

    if importance < 0.0 || importance > 1.0 {
        return Err(RusqliteError::Other(format!(
            "Importance debe estar entre 0 y 1: {}",
            importance
        )));
    }

    let level = json
        .get("level")
        .and_then(|v| v.as_str())
        .ok_or_else(|| RusqliteError::Other("Falta campo 'level'".to_string()))?
        .to_string();

    match level.as_str() {
        "Fact" | "Pattern" | "Principle" => {}
        _ => return Err(RusqliteError::Other(format!("Level inválido: {}", level))),
    }

    let source_summary = json
        .get("source_summary")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    Ok(ParsedReflection {
        content,
        r#type,
        importance,
        level,
        source_summary,
    })
}

fn unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn build_prompt(memories: &[Memory]) -> String {
    let mut prompt = String::new();

    //  Instrucciones del sistema
    prompt.push_str("Eres un asistente que analiza memorias de programación y extrae insights consolidados.\n\n");
    prompt.push_str("Tus tareas:\n");
    prompt.push_str("1. Analiza todas las memorias proporcionadas\n");
    prompt.push_str(
        "2. Identifica patrones recurrentes, principios generales o advertencias importantes\n",
    );
    prompt.push_str("3. Genera un insight consolidado que sea útil para sesiones futuras\n\n");

    //  Formato de salida requerido
    prompt.push_str(
        "RESPONDE EXCLUSIVAMENTE CON UN JSON EN ESTE FORMATO (sin markdown, solo el JSON):\n",
    );
    prompt.push_str(
        r#"{
    "content": "insight consolidado en texto claro",
    "type": "pattern|principle|warning",
    "importance": 0.85,
    "level": "Fact|Pattern|Principle",
    "source_summary": "Resumen breve de qué memorias originaron este insight"
}
"#,
    );
    prompt.push_str("\nDonde:\n");
    prompt.push_str("- type: pattern (patrón recurrente), principle (principio general), warning (advertencia)\n");
    prompt.push_str("- importance: número entre 0.0 y 1.0 (1.0 = muy importante)\n");
    prompt.push_str(
        "- level: Fact (hecho concreto), Pattern (patrón), Principle (principio abstracto)\n\n",
    );

    //  Listado de memorias
    prompt.push_str("MEMORIAS DE LA SESIÓN:\n");
    prompt.push_str("=====================\n\n");

    for (i, memory) in memories.iter().enumerate() {
        prompt.push_str(&format!("[MEMORIA {}]\n", i + 1));
        prompt.push_str(&format!("  Título: {}\n", memory.title));

        if let Some(ref t) = memory.r#type {
            prompt.push_str(&format!("  Tipo: {:?}\n", t));
        }

        if let Some(ref what) = memory.what {
            prompt.push_str(&format!("  Qué: {}\n", what));
        }

        if let Some(ref why) = memory.why {
            prompt.push_str(&format!("  Por qué: {}\n", why));
        }

        if let Some(ref where_field) = memory.where_field {
            prompt.push_str(&format!("  Dónde: {}\n", where_field));
        }

        if let Some(ref learned) = memory.learned {
            prompt.push_str(&format!("  Aprendido: {}\n", learned));
        }

        prompt.push_str("\n");
    }

    prompt.push_str("INSTRUCCIONES FINALES:\n");
    prompt.push_str("- Analiza TODAS las memorias anteriores\n");
    prompt.push_str("- Extrae un insight ÚNICO y consolidado\n");
    prompt.push_str("- Responde SOLO con el JSON, sin texto adicional\n");
    prompt.push_str("- No uses markdown (```json ```), solo el JSON puro\n");

    prompt
}

pub fn generate_reflection(conn: &Connection, session_id: &str) -> Result<Reflection> {
    //Buscar memorias en la session
    let memories = get_memory_by_id(conn, session_id, 100)?;
    //Si no hay memorias retornar error o null
    if memorias.is_empty() {
        return Err(RusqliteError::Other(
            "No hay memorias para reflexionar".to_string(),
        ));
    }
    let prompt = build_prompt(&memories);
    let llm_response = call_ollana(&prompt);
    let parsed = parse_response(&llm_response);
    //guardar en bd
    let reflection_id = uuid::Uuid::new_v4().to_string();
    let now = unix_timestamp();

    conn.execute(
        CREATE_REFLECTION,
        params![
            reflection_id,
            session_id,
            parsed.content,
            parsed.r#type,
            parsed.importance,
            parsed.level,
            parsed.source_summary,
            now,
            Option::<i64>::None,
        ],
    )?;
    Ok(Reflection {
        reflection_id,
        session_id: session_id.to_string(),
        content: parsed.content,
        r#type: parsed.r#type,
        importance: parsed.importance,
        level: parsed.level,
        source_summary: Some(parsed.source_summary),
        created_at: now,
        deleted_at: None,
    })
}
pub fn get_reflection_by_session() -> String {}
pub fn kill_reflection() -> String {}
