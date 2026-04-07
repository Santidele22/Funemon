use crate::db::models::Memories;
use rusqlite::{Connection, Result, Row, params};

// --- Constantes SQL ---
const CREATE_MEMORY: &str = "
    INSERT INTO memories (memory_id, session_id, title, type, what, why, where_field, learned, created_at, deleted_at)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
";

const GET_MEMORY_BY_ID: &str = "
    SELECT memory_id, session_id, created_at, title, type, what, why, where_field, learned, deleted_at
    FROM memories
    WHERE memory_id = ?1 AND deleted_at IS NULL
";

const SEARCH_MEMORIES_BASE: &str = "
    SELECT m.memory_id, m.session_id, m.created_at, m.title, m.type,
           m.what, m.why, m.where_field, m.learned, m.deleted_at
    FROM memories m
    JOIN memories_fts fts ON m.rowid = fts.rowid
    WHERE fts MATCH ?1
";

const SOFT_DELETE_MEMORY: &str = "
    UPDATE memories SET deleted_at = ?1 WHERE memory_id = ?2
";

const HARD_DELETE_MEMORY: &str = "
    DELETE FROM memories WHERE memory_id = ?1
";

const GET_SESSION_MEMORIES: &str = "
    SELECT memory_id, session_id, created_at, title, type,
           what, why, where_field, learned, deleted_at
    FROM memories
    WHERE session_id = ?1 AND deleted_at IS NULL
    ORDER BY created_at DESC
    LIMIT ?2
";

// --- Funciones Auxiliares ---

fn unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

fn memory_from_row(row: &Row) -> Result<Memories> {
    Ok(Memories {
        memory_id: row.get(0)?,
        session_id: row.get(1)?,
        created_at: row.get(2)?,
        title: row.get(3)?,
        r#type: row.get(4)?,
        what: row.get(5)?,
        why: row.get(6)?,
        where_field: row.get(7)?,
        learned: row.get(8)?,
        deleted_at: row.get(9)?,
    })
}

pub fn store_memory(conn: &Connection, memory: &Memories) -> Result<String> {
    conn.execute(
        CREATE_MEMORY,
        params![
            memory.memory_id,
            memory.session_id,
            memory.title,
            memory.r#type,
            memory.what,
            memory.why,
            memory.where_field,
            memory.learned,
            memory.created_at,
            memory.deleted_at,
        ],
    )?;
    Ok(memory.memory_id.clone())
}

pub fn get_memory_by_id(conn: &Connection, memory_id: &str) -> Result<Option<Memories>> {
    let mut stmt = conn.prepare(GET_MEMORY_BY_ID)?;
    let mut rows = stmt.query(params![memory_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(memory_from_row(row)?))
    } else {
        Ok(None)
    }
}

pub fn search_memories(
    conn: &Connection,
    query: &str,
    session_id: Option<&str>,
    limit: usize,
) -> Result<Vec<Memories>> {
    let mut sql = String::from(SEARCH_MEMORIES_BASE);

    if session_id.is_some() {
        sql.push_str(" AND m.session_id = ?2");
    }

    sql.push_str(" AND m.deleted_at IS NULL");
    sql.push_str(" ORDER BY fts.score DESC");
    sql.push_str(" LIMIT ?3"); // Cambiado a parámetro para mayor seguridad

    let mut stmt = conn.prepare(&sql)?;

    // Ejecución basada en si hay session_id o no
    let memories_iter = match session_id {
        Some(sid) => stmt.query_map(params![query, sid, limit as i64], memory_from_row)?,
        None => {
            // El tercer parámetro (?3) se vuelve el segundo en la posición si no hay SID
            // pero para mantener consistencia con el SQL dinámico, ajustamos los params:
            stmt.query_map(
                params![query, rusqlite::types::Null, limit as i64],
                memory_from_row,
            )?
        }
    };

    memories_iter.collect()
}

pub fn delete_memory(conn: &Connection, memory_id: &str, permanent: bool) -> Result<bool> {
    let affected = if permanent {
        conn.execute(HARD_DELETE_MEMORY, params![memory_id])?
    } else {
        let now = unix_timestamp();
        conn.execute(SOFT_DELETE_MEMORY, params![now, memory_id])?
    };
    Ok(affected > 0)
}

pub fn get_session_context(
    conn: &Connection,
    session_id: &str,
    limit: usize,
) -> Result<Vec<Memories>> {
    let mut stmt = conn.prepare(GET_SESSION_MEMORIES)?;

    let memories = stmt
        .query_map(params![session_id, limit as i64], memory_from_row)?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(memories)
}
