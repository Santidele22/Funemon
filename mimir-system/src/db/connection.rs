use rustqlite::{Connection, Result};
mod memory_ops;
mod reflection_ops;
mod session_ops;


fn connection() ->Result<()> {
    let conn = Connection::open("mimir.db")?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            session_id TEXT PRIMARY KEY,
            project TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            last_active INTEGER NOT NULL,
            deleted_at INTEGER,
            ended_at INTEGER
        )",
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
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS reflections (
            project TEXT NOT NULL,
            reflection_id TEXT PRIMARY KEY,
            session_id TEXT NOT NULL,
            content TEXT NOT NULL,
            type TEXT NOT NULL,
            importance INTEGER NOT NULL,
            level TEXT NOT NULL,
            source_summary TEXT,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (session_id) REFERENCES sessions(session_id)
        )",
    )?;

    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5(
            title, what, why, where_field, learned,
            content='memories',
            content_rowid='rowid'
        )",
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_ai AFTER INSERT ON memories BEGIN
            INSERT INTO memories_fts(rowid, title, what, why, where_field, learned)
            VALUES (NEW.rowid, NEW.title, NEW.what, NEW.why, NEW.where_field, NEW.learned);
        END",
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_ad AFTER DELETE ON memories BEGIN
            INSERT INTO memories_fts(memories_fts, rowid, title, what, why, where_field, learned)
            VALUES ('delete', OLD.rowid, OLD.title, OLD.what, OLD.why, OLD.where_field, OLD.learned);
        END",
    )?;

    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS memories_au AFTER UPDATE ON memories BEGIN
            INSERT INTO memories_fts(memories_fts, rowid, title, what, why, where_field, learned)
            VALUES ('delete', OLD.rowid, OLD.title, OLD.what, OLD.why, OLD.where_field, OLD.learned);
            INSERT INTO memories_fts(rowid, title, what, why, where_field, learned)
            VALUES (NEW.rowid, NEW.title, NEW.what, NEW.why, NEW.where_field, NEW.learned);
        END",
    )?;

    Ok(()) 
}
