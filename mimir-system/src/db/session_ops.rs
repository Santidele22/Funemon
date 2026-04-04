use crate::models::Session;
use rusqlite::{Connection, Result, params};
use uuid::Uuid;

//Consultas sql para la sessiones
const CREATE_SESION: &str = "INSERT INTO sessions (session_id, project, created_at,last_active, deleted_at,ended_at) VALUES (?1,?2,?3,?4,?5,?6)";
const GET_SESSION_BY_ID: &str = "
    SELECT session_id, project, created_at, last_active, deleted_at ,ended_at
    FROM sessions
    WHERE session_id = ?1 AND deleted_at IS NULL
";
const UPDATE_LAST_ACTIVE: &str = "UPDATE sessions SET last_active = ?1 WHERE session_id = ?2";
const GET_ACTIVE_SESSION: &str = "
    SELECT session_id, project, created_at, last_active, deleted_at, ended_at   FROM sessions
    WHERE project = ?1 
    AND deleted_at IS NULL 
    AND ended_at IS NULL
    ORDER BY last_active DESC
    LIMIT 1
";
const LIST_SESSIONS: &str = "
    SELECT session_id, project, created_at, last_active, deleted_at,ended_at     FROM sessions
    WHERE project = ?1 AND deleted_at IS NULL
    ORDER BY last_active DESC
";
const CLEANUP_EXPIRED: &str = "
    UPDATE sessions 
    SET deleted_at = ?1 
    WHERE project = ?2 
    AND deleted_at IS NULL 
    AND ended_at IS NULL
    AND last_active < ?3
";

fn unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
fn session_from_row(row: &rusqlite::Row) -> Result<Session> {
    Ok(Session {
        session_id: row.get(0)?,
        project: row.get(1)?,
        created_at: row.get(2)?,
        last_active: row.get(3)?,
        deleted_at: row.get(5)?,
        ended_at: row.get(4)?,
    })
}

pub fn start_session(
    conn: &Connection,
    project: &str,
    existing_session_id: Option<&str>,
) -> Result<String> {
    // Verificar si ya existe una session con ese id
    if let Some(sid) = existing_session_id {
        let now = unix_timestamp();
        // Si existe actualizar el last_active y seguir con esa session
        conn.execute(UPDATE_LAST_ACTIVE, params![now, sid])?;
        if let Some(session) = get_session_by(conn, sid)? {
            return OK(session);
        }
    }
    // sino existe crear session
    let session_uuid = Uuid::new_v4().to_string();
    let now = unix_timestamp();
    conn.execute(
        CREATE_SESION,
        params![session_uuid, project, now, now, Option<i64>, Option<i64>],
    )?;

    OK(Session {
        last_active: now,
        deleted_at: None,
        ended_at: None,
    })
}
fn get_session_by(conn: &Connection, session_id: Uuid) -> Result<String> {
    let mut smtmt = conn.prepare(GET_SESSION)?;
    let session = smtmt.query(params![session_id])?;
    if let Some(row) = rows.next()? {
        OK(Some(session_from_row(row)?))
    } else {
        OK(None)
    }
}
pub fn list_sessions(conn: &Connection, project: &str) -> Result<Vec<Session>> {
    let mut stmt = conn.prepare(LIST_SESSIONS)?;
    let mut rows = stmt.query(params![project])?;

    let mut sessions = Vec::new();
    while let Some(row) = rows.next()? {
        sessions.push(session_from_row(row)?);
    }
    Ok(sessions)
}
pub fn get_active_session(conn: &Connection, project: &str) -> Result<Option<Session>> {
    let mut stmt = conn.prepare(GET_ACTIVE_SESSION)?;
    let mut rows = stmt.query(params![project])?;

    if let Some(row) = rows.next()? {
        Ok(Some(session_from_row(row)?))
    } else {
        Ok(None)
    }
}
pub fn cleanup_expired_sessions(
    conn: &Connection,
    project: &str,
    days_inactive: i64,
) -> Result<usize> {
    let now = unix_timestamp();
    let expire_time = now - (days_inactive * 86400); // 86400 segundos por día

    let affected = conn.execute(CLEANUP_EXPIRED, params![now, project, expire_time])?;

    Ok(affected)
}
