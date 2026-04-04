use super::models::Memory;
use rusqlite::{Connection, Result, params};
//Consultas sql para la sessiones
const CREATE_SESION: &str = "";
const GET_SESSION: &str = "";
const KILL_SESSION: &str

fn createSession() -> Result<String> {
    // Verificar si ya existe una session con ese id
    // Si existe retornar error
    // Crear session
}
fn searchAllSession() -> Result<String> {
    //Existe la session?
}
fn searchSessionById() -> Result<String> {}
fn deleteSessin() -> Result<String> {
    //existe la session?
}
