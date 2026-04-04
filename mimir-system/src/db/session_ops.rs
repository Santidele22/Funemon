use super::models::Memory;
use rusqlite::{Connection, Result, params};
//Consultas sql para la sessiones
const CREATE_SESION: &str = "";
const GET_SESSION: &str = "";
const KILL_SESSION: &str = "";

fn create_session() -> Result<String> {
    // Verificar si ya existe una session con ese id
    // Si existe retornar error
    // Crear session
}
fn search_all_Session() -> Result<String> {
    //Existe la session?
}
fn search_session_by_id() -> Result<String> {}
fn deleteSessin() -> Result<String> {
    //existe la session?
}
