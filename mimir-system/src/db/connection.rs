use rustqlite::{Connection, Result};
mod memory_ops;
mod reflection_ops;
mod session_ops;


fn connection() ->Result<()> {
    let conn = Connection::open("mimir.db")?;
    //Crea las tablas
    conn.execute()?;
    Ok(()) 
}
