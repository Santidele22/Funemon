use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

static DB_CONNECTION: OnceLock<Arc<Mutex<Connection>>> = OnceLock::new();

fn get_db_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("No home dir");
    path.push(".local/share/mimir/mimir.db");
    path
}

fn init_database_inner(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            session_id TEXT PRIMARY KEY,
            project TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            last_active INTEGER NOT NULL,
            deleted_at INTEGER,
            ended_at INTEGER
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS memories (
            memory_id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            type TEXT,
            title TEXT NOT NULL,
            what TEXT,
            where_field TEXT,
            why TEXT,
            learned TEXT,
            created_at INTEGER NOT NULL,
            deleted_at INTEGER,
            FOREIGN KEY (session_id) REFERENCES sessions(session_id)
        )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reflections (
            reflection_id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            content TEXT NOT NULL,
            type TEXT NOT NULL,
            importance REAL NOT NULL,
            level TEXT NOT NULL,
            source_summary TEXT,
            created_at INTEGER NOT NULL,
            deleted_at INTEGER,
            FOREIGN KEY (session_id) REFERENCES sessions(session_id)
        )",
        (),
    )?;

    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5(
            title, what, why, where_field, learned,
            content='memories',
            content_rowid='rowid'
        )",
        (),
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_ai AFTER INSERT ON memories BEGIN
            INSERT INTO memories_fts(rowid, title, what, why, where_field, learned)
            VALUES (NEW.rowid, NEW.title, NEW.what, NEW.why, NEW.where_field, NEW.learned);
        END",
        (),
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_ad AFTER DELETE ON memories BEGIN
            INSERT INTO memories_fts(memories_fts, rowid)
            VALUES ('delete', OLD.rowid);
        END",
        (),
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_au AFTER UPDATE ON memories BEGIN
            INSERT INTO memories_fts(memories_fts, rowid)
            VALUES ('delete', OLD.rowid);
            INSERT INTO memories_fts(rowid, title, what, why, where_field, learned)
            VALUES (NEW.rowid, NEW.title, NEW.what, NEW.why, NEW.where_field, NEW.learned);
        END",
        (),
    )?;

    Ok(())
}

pub fn init_database() -> Result<()> {
    let db_path = get_db_path();
    std::fs::create_dir_all(db_path.parent().unwrap()).expect("Failed to create db directory");

    let conn = Connection::open(&db_path)?;

    // Configuración óptima para SQLite
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "busy_timeout", "5000")?;

    init_database_inner(&conn)?;

    DB_CONNECTION.set(Arc::new(Mutex::new(conn))).unwrap();

    eprintln!("✅ Mimir DB initialized at {:?}", db_path);
    Ok(())
}
pub fn get_connection() -> Result<Arc<Mutex<Connection>>> {
    if DB_CONNECTION.get().is_none() {
        init_database()?;
    }

    DB_CONNECTION.get().cloned().ok_or_else(|| {
        rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(1),
            Some("Database not initialized".to_string()),
        )
    })
}
