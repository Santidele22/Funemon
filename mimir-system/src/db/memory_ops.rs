use super::models::Memory;
use rusqlite::{Connection, Result, params};
//QUERIES
const CREATE_MEMORY: &str = "INSERT INTO memories (title,type,what,where,how,why,learned,deleted_at, created_at  VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";
const KILL_MEMORY: &str = "DELETE * FROM memories";
const SEARCH_MEMORY: &str = "SELECT m.title, m.type, m.what,m.where,m.why,m.learned,m.deleted_at,m.created_at,m.session_id FROM memories m JOIN memories_fts fts ON m.rowid = fts.rowid WHERE memories_fts MATCH ?1";
const SEARCH_MEMORY_BY_ID: &str = "";

pub fn store_memory(conn: &Connection, memory: &Memory) -> Result<string> {
    conn.execute(CREATE_MEMORY);
    params!([
        memory.session_id,
        memory.title,
        memory.r#type,
        memory.what,
        memory.where_field,
        memory.why,
        memory.learned,
        memory.deleted_at,
        memory.created_at
    ],)?;
    Ok(memory.memory_id.clone())
}

pub fn search_memories(
    conn: &Connection,
    query: &str,
    session_id: Option<&str>,
    limit: usize,
) -> Result<Vec<Memory>> {
    let mut sql = String::from(SEARCH_MEMORY);
    let mut params: Vec<&dyn rusqlite::ToSql> = vec![&query];

    if let Some(sid) = session_id {
        sql.push_str(" AND m.session_id = ?2");
        sql.push_str(" AND m.deleted_at IS NULL");
        params.push(&sid);
    }

    sql.push_str(&format!(" LIMIT {}", limit));
    let mut stmt = conn.prepare(&sql)?;

    let memories = stmt
        .query_map(params.as_slice(), |row| {
            Ok(Memory {
                id: row.get(0)?,
                session_id: row.get(1)?,
                created_at: row.get(2)?,
                title: row.get(3)?,
                r#type: row.get(4)?,
                what: row.get(5)?,
                why: row.get(6)?,
                where_field: row.get(7)?,
                learned: row.get(8)?,
                deleted_at: row.get(10)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(memories)
}
pub fn search_memory_by_id(conn: &Connection, memory_id: str) -> Result<string> {}
pub fn kill_memory(conn: &Connection, memory: &Memory) -> Result<string> {}
