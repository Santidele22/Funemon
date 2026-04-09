use std::sync::{Arc, Mutex};

use rusqlite::params;

use crate::db::models::Sessions;

const CREATE_SESSION: &str = "INSERT INTO sessions (session_id, project, created_at, last_active, deleted_at, ended_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
const GET_SESSION_BY_ID: &str = "
    SELECT session_id, project, created_at, last_active, deleted_at, ended_at
    FROM sessions
    WHERE session_id = ?1 AND deleted_at IS NULL
";
const UPDATE_LAST_ACTIVE: &str = "UPDATE sessions SET last_active = ?1 WHERE session_id = ?2";
const GET_ACTIVE_SESSION: &str = "
    SELECT session_id, project, created_at, last_active, deleted_at, ended_at
    FROM sessions
    WHERE project = ?1 
    AND deleted_at IS NULL 
    AND ended_at IS NULL
    ORDER BY last_active DESC
    LIMIT 1
";
const LIST_SESSIONS: &str = "
    SELECT session_id, project, created_at, last_active, deleted_at, ended_at
    FROM sessions
    WHERE deleted_at IS NULL
    AND ( ?1 = '' OR project = ?1 )
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
const SOFT_DELETE_SESSION: &str = "
    UPDATE sessions SET deleted_at = ?1 WHERE session_id = ?2
";

const HARD_DELETE_SESSION: &str = "
    DELETE FROM sessions WHERE session_id = ?1
";
const END_SESSION_SQL: &str = "
    UPDATE sessions SET ended_at = ?1, last_active = ?1
    WHERE session_id = ?2 AND ended_at IS NULL
";

fn unix_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn session_from_row(row: &rusqlite::Row) -> Result<Sessions, rusqlite::Error> {
    Ok(Sessions {
        session_id: row.get(0)?,
        project: row.get(1)?,
        created_at: row.get(2)?,
        last_active: row.get(3)?,
        deleted_at: row.get(4)?,
        ended_at: row.get(5)?,
    })
}

pub fn start_session(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    project: &str,
    existing_session_id: Option<&str>,
) -> Result<Sessions, rusqlite::Error> {
    let mut conn = conn.lock().unwrap();

    if let Some(sid) = existing_session_id {
        let now = unix_timestamp();
        conn.execute(UPDATE_LAST_ACTIVE, params![now, sid])?;
        let mut stmt = conn.prepare(GET_SESSION_BY_ID)?;
        let mut rows = stmt.query(params![sid])?;
        if let Some(row) = rows.next()? {
            return session_from_row(row);
        }
    }

    let session_uuid = uuid::Uuid::new_v4().to_string();
    let now = unix_timestamp();
    conn.execute(
        CREATE_SESSION,
        params![
            session_uuid,
            project,
            now,
            now,
            Option::<i64>::None,
            Option::<i64>::None
        ],
    )?;

    Ok(Sessions {
        session_id: session_uuid,
        project: project.to_string(),
        created_at: now,
        last_active: now,
        deleted_at: None,
        ended_at: None,
    })
}

pub fn get_session_by_id(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    session_id: &str,
) -> Result<Option<Sessions>, rusqlite::Error> {
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(GET_SESSION_BY_ID)?;
    let mut rows = stmt.query(params![session_id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(session_from_row(row)?))
    } else {
        Ok(None)
    }
}

pub fn list_sessions(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    project: &str,
) -> Result<Vec<Sessions>, rusqlite::Error> {
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(LIST_SESSIONS)?;
    let mut rows = stmt.query(params![project])?;

    let mut sessions = Vec::new();
    while let Some(row) = rows.next()? {
        sessions.push(session_from_row(row)?);
    }
    Ok(sessions)
}

pub fn get_active_session(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    project: &str,
) -> Result<Option<Sessions>, rusqlite::Error> {
    let conn = conn.lock().unwrap();
    let mut stmt = conn.prepare(GET_ACTIVE_SESSION)?;
    let mut rows = stmt.query(params![project])?;

    if let Some(row) = rows.next()? {
        Ok(Some(session_from_row(row)?))
    } else {
        Ok(None)
    }
}

pub fn cleanup_expired_sessions(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    project: &str,
    days_inactive: i64,
) -> Result<usize, rusqlite::Error> {
    let conn = conn.lock().unwrap();
    let now = unix_timestamp();
    let expire_time = now - (days_inactive * 86400);

    let affected = conn.execute(CLEANUP_EXPIRED, params![now, project, expire_time])?;

    Ok(affected)
}

pub fn end_session(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    session_id: &str,
) -> Result<Sessions, rusqlite::Error> {
    let conn = conn.lock().unwrap();
    let now = unix_timestamp();
    conn.execute(END_SESSION_SQL, params![now, session_id])?;

    let mut stmt = conn.prepare(GET_SESSION_BY_ID)?;
    let mut rows = stmt.query(params![session_id])?;

    rows.next()?
        .map(session_from_row)
        .transpose()?
        .ok_or(rusqlite::Error::QueryReturnedNoRows)
}

pub fn delete_session(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    session_id: &str,
    permanent: bool,
) -> Result<bool, rusqlite::Error> {
    let conn = conn.lock().unwrap();
    let affected = if permanent {
        conn.execute(HARD_DELETE_SESSION, params![session_id])?
    } else {
        let now = unix_timestamp();
        conn.execute(SOFT_DELETE_SESSION, params![now, session_id])?
    };
    Ok(affected > 0)
}
