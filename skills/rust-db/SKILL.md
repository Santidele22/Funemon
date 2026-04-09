---
name: rust-db
description: Rust database patterns con rusqlite. Connections, queries, migrations, FTS. Guide para SQLite en Rust.
license: Apache-2.0
compatibility: opencode
metadata:
  audience: agents
  workflow: rust-db
---

## ¿Qué soy?

Soy el guide de base de datos en Rust. Te ayudo a usar SQLite con rusqlite efectivamente.

## Estructura Básica

### Connection

```rust
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex, OnceLock};

static DB: OnceLock<Arc<Mutex<Connection>>> = OnceLock::new();

fn get_connection() -> Result<Arc<Mutex<Connection>>> {
    if DB.get().is_none() {
        let conn = Connection::open("db.sqlite")?;
        DB.set(Arc::new(Mutex::new(conn))).unwrap();
    }
    DB.get().cloned().ok_or(...)
}
```

### Connection con Path

```rust
use std::path::PathBuf;

fn get_db_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap();
    path.push("mimir/mimir.db");
    path
}

fn init_db() -> Result<()> {
    let conn = Connection::open(get_db_path())?;
    // Configuracion
    conn.pragma_update(None, "journal_mode", "WAL")?;
    Ok(())
}
```

## Queries

### SELECT Básico

```rust
fn get_users(conn: &Connection) -> Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT id, name FROM users")?;
    let rows = stmt.query([])?;
    
    let mut users = Vec::new();
    while let Some(row) = rows.next()? {
        users.push(User {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }
    Ok(users)
}
```

### SELECT con Parametros

```rust
fn get_user_by_id(conn: &Connection, id: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare("SELECT id, name FROM users WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;
    
    if let Some(row) = rows.next()? {
        Ok(Some(User::from_row(row)?))
    } else {
        Ok(None)
    }
}
```

### INSERT

```rust
fn insert_user(conn: &Connection, user: &User) -> Result<()> {
    conn.execute(
        "INSERT INTO users (id, name) VALUES (?1, ?2)",
        params![user.id, user.name],
    )?;
    Ok(())
}
```

### UPDATE

```rust
fn update_user(conn: &Connection, user: &User) -> Result<()> {
    conn.execute(
        "UPDATE users SET name = ?1 WHERE id = ?2",
        params![user.name, user.id],
    )?;
    Ok(())
}
```

### DELETE

```rust
fn delete_user(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
    Ok(())
}
```

## Transactions

```rust
fn do_transaction(conn: &Connection) -> Result<()> {
    let tx = conn.unchecked_transaction()?;
    
    tx.execute("INSERT...", params![...])?;
    tx.execute("UPDATE...", params![...])?;
    
    tx.commit()?;  // O rollback si error
    Ok(())
}
```

## Migrations

### Schema

```rust
fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        [],
    )?;
    
    Ok(())
}
```

### Migration Runner

```rust
const MIGRATIONS: &[&str] = &[
    "CREATE TABLE IF NOT EXISTS users (...)",
    "CREATE TABLE IF NOT EXISTS sessions (...)",
    "ALTER TABLE users ADD COLUMN email TEXT",
];

fn run_migrations(conn: &Connection) -> Result<()> {
    for (i, sql) in MIGRATIONS.iter().enumerate() {
        conn.execute(sql, [])?;
    }
    Ok(())
}
```

## Full-Text Search (FTS5)

### Create FTS Table

```rust
fn init_fts(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS posts_fts USING fts5(
            title,
            content,
            content='posts',
            content_rowid='rowid'
        )",
        [],
    )?;
    Ok(())
}
```

### Search

```rust
fn search_posts(conn: &Connection, query: &str) -> Result<Vec<Post>> {
    let mut stmt = conn.prepare(
        "SELECT p.* FROM posts p
         JOIN posts_fts f ON p.rowid = f.rowid
         WHERE posts_fts MATCH ?1"
    )?;
    
    let rows = stmt.query(params![query])?;
    // ...
}
```

## Connection Pooling

### Simple Pool

```rust
use std::sync::{Arc, Mutex};

struct Pool {
    conn: Arc<Mutex<Connection>>,
}

impl Pool {
    fn new() -> Result<Self> {
        Ok(Pool {
            conn: Arc::new(Mutex::new(Connection::open("db.sqlite")?)),
        })
    }
    
    fn with_connection<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let conn = self.conn.lock().unwrap();
        f(&conn)
    }
}
```

## Best Practices

### ✅ SIEMPRE

- Usar parameterized queries (`?1`, `?2`)
- Usar transactions para multi-statement
- Usar migrations para schema changes
- Close connection on shutdown
- Handle Result explicitamente

### ✅ NUNCA

- No concatenar strings en queries (SQL injection!)
- No usar raw strings en SQL
- No olvidar transaction rollback
- No ignore errors with `ok()`

### Errores Comunes

| Error | Causa | Solucion |
|-------|------|----------|
| `locked` | Transaction activa | Commit/Rollback |
| `no such table` | Schema no creado | run migrations |
| `UNIQUE constraint` | Duplicate key | Usar INSERT OR IGNORE |

## Checklist

- [ ] parameterized queries
- [ ] migrations system
- [ ] connection management
- [ ] error handling
- [ ] FTS para search
- [ ] transactions donde needed

## Triggers

Este skill se activa cuando:
- Working con SQLite/rusqlite
- Creating queries
- migrations
- FTS

---

## Regla de Hierro

**"Parameterized queries siempre. SQL injection es para siempre."**