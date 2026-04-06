use crate::models::Memory;
use rusqlite::{Connection, Result, params};

// ============================================
// QUERYS SQL
// ============================================
const CREATE_MEMORY: &str = "
    INSERT INTO memories (memory_id, session_id, title, type, what, why, where_field, learned, tags, created_at, deleted_at)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
";

const GET_MEMORY_BY_ID: &str = "
    SELECT memory_id, session_id, created_at, title, type, what, why, where_field, learned, tags, deleted_at
    FROM memories
    WHERE memory_id = ?1 AND deleted_at IS NULL
";

const SEARCH_MEMORIES: &str = "
    SELECT m.memory_id, m.session_id, m.created_at, m.title, m.type, 
           m.what, m.why, m.where_field, m.learned, m.tags, m.deleted_at
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
           what, why, where_field, learned, tags, deleted_at
    FROM memories
    WHERE session_id = ?1 AND deleted_at IS NULL
    ORDER BY created_at DESC
    LIMIT ?2
";

// ============================================
// FUNCIONES AUXILIARES
// ============================================

fn unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn memory_from_row(row: &rusqlite::Row) -> Result<Memory> {
    Ok(Memory {
        memory_id: row.get(0)?,
        session_id: row.get(1)?,
        created_at: row.get(2)?,
        title: row.get(3)?,
        r#type: row.get(4)?,
        what: row.get(5)?,
        why: row.get(6)?,
        where_field: row.get(7)?,
        learned: row.get(8)?,
        tags: row.get(9)?,
        deleted_at: row.get(10)?,
    })
}

// ============================================
// FUNCIONES PÚBLICAS
// ============================================

pub fn store_memory(conn: &Connection, memory: &Memory) -> Result<String> {
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
            memory.tags,
            memory.created_at,
            memory.deleted_at,
        ],
    )?;
    Ok(memory.memory_id.clone())
}

pub fn get_memory_by_id(conn: &Connection, memory_id: &str) -> Result<Option<Memory>> {
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
) -> Result<Vec<Memory>> {
    let mut sql = String::from(SEARCH_MEMORIES);
    let mut params: Vec<&dyn rusqlite::ToSql> = vec![&query];

    if let Some(sid) = session_id {
        sql.push_str(" AND m.session_id = ?2");
        params.push(&sid);
    }

    sql.push_str(" AND m.deleted_at IS NULL");
    sql.push_str(" ORDER BY fts.score DESC");
    sql.push_str(&format!(" LIMIT {}", limit));

    let mut stmt = conn.prepare(&sql)?;

    let memories = stmt
        .query_map(params.as_slice(), |row| {
            Ok(Memory {
                memory_id: row.get(0)?,
                session_id: row.get(1)?,
                created_at: row.get(2)?,
                title: row.get(3)?,
                r#type: row.get(4)?,
                what: row.get(5)?,
                why: row.get(6)?,
                where_field: row.get(7)?,
                learned: row.get(8)?,
                tags: row.get(9)?,
                deleted_at: row.get(10)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(memories)
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
) -> Result<Vec<Memory>> {
    let mut stmt = conn.prepare(GET_SESSION_MEMORIES)?;
    let memories = stmt
        .query_map(params![session_id, limit], |row| {
            Ok(Memory {
                memory_id: row.get(0)?,
                session_id: row.get(1)?,
                created_at: row.get(2)?,
                title: row.get(3)?,
                r#type: row.get(4)?,
                what: row.get(5)?,
                why: row.get(6)?,
                where_field: row.get(7)?,
                learned: row.get(8)?,
                tags: row.get(9)?,
                deleted_at: row.get(10)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(memories)
}
