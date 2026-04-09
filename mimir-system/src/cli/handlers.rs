use crate::cli::commands::{Cli, Commands, MemoryCommands, ReflectionCommands, SessionCommands};
use crate::db::models::{Memories, MemoryType};
use crate::tui::run_tui;
use rusqlite::Row;
use crate::db::{
    cleanup_expired_sessions, delete_memory, delete_reflection, delete_session,
    generate_reflection, get_connection, get_reflection_by_session, get_session_context,
    init_database, list_sessions, search_memories, start_session, store_memory,
};

pub async fn run_cli(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Some(Commands::Mcp) => {
            crate::mcp::run_server().await?;
        }
        Some(Commands::Init) => {
            init_database()?;
            eprintln!("✅ Base de datos inicializada correctamente");
        }

        Some(Commands::Session(cmd)) => handle_session_command(cmd)?,
        Some(Commands::Memories(cmd)) => handle_memory_command(cmd)?,
        Some(Commands::Reflection(cmd)) => handle_reflection_command(cmd)?,

        Some(Commands::Stats) => {
            let conn = get_connection()?;
            let conn = conn.lock().unwrap();

            let session_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM sessions WHERE deleted_at IS NULL",
                [],
                |row: &rusqlite::Row| row.get(0),
            )?;

            let memory_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM memories WHERE deleted_at IS NULL",
                [],
                |row: &rusqlite::Row| row.get(0),
            )?;

            let reflection_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM reflections WHERE deleted_at IS NULL",
                [],
                |row: &rusqlite::Row| row.get(0),
            )?;

            println!("📊 Estadísticas de Mimir:");
            println!("   Sesiones activas: {}", session_count);
            println!("   Memorias guardadas: {}", memory_count);
            println!("   Reflexiones generadas: {}", reflection_count);
        }

        Some(Commands::Tui) => {
            run_tui()?;
        }
        None => {
            println!("🧠 Mimir CLI");
            println!("Usá alguno de los siguientes comandos:");
            println!("  mimir mcp                → Inicia el servidor MCP");
            println!("  mimir init               → Inicializa la base de datos");
            println!("  mimir session ...        → Gestión de sesiones");
            println!("  mimir memories ...       → Gestión de memorias");
            println!("  mimir reflection ...     → Gestión de reflexiones");
            println!("  mimir stats              → Estadísticas");
        }
    }

    Ok(())
}

fn handle_session_command(cmd: SessionCommands) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection()?;

    match cmd {
        SessionCommands::Start {
            project,
            session_id,
        } => {
            let session = start_session(&conn, &project, session_id.as_deref())?;
            println!("✅ Sesión iniciada/reanudada:");
            println!("   ID: {}", session.session_id);
            println!("   Proyecto: {}", session.project);
            println!("   Creada: {}", timestamp_to_string(session.created_at));
            println!(
                "   Última actividad: {}",
                timestamp_to_string(session.last_active)
            );
        }

        SessionCommands::List { project } => {
            let sessions = list_sessions(&conn, &project)?;

            if sessions.is_empty() {
                println!("📭 No hay sesiones para el proyecto '{}'", project);
            } else {
                println!("📋 Sesiones del proyecto '{}':", project);
                for session in sessions {
                    let status = if session.ended_at.is_some() {
                        "🔚"
                    } else {
                        "🟢"
                    };
                    println!(
                        "   {} {} - Última actividad: {}",
                        status,
                        session.session_id,
                        timestamp_to_string(session.last_active)
                    );
                }
            }
        }

        SessionCommands::Delete {
            session_id,
            permanent,
        } => {
            let deleted = delete_session(&conn, &session_id, permanent)?;
            if deleted {
                let delete_type = if permanent {
                    "eliminada permanentemente"
                } else {
                    "marcada como eliminada"
                };
                println!("✅ Sesión {} {}", session_id, delete_type);
            } else {
                println!("❌ Sesión {} no encontrada", session_id);
            }
        }

        SessionCommands::Cleanup { project, days } => {
            let count = cleanup_expired_sessions(&conn, &project, days)?;
            println!("✅ {} sesiones expiradas eliminadas", count);
        }
    }

    Ok(())
}

fn handle_memory_command(cmd: MemoryCommands) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection()?;

    match cmd {
        MemoryCommands::Store {
            session_id,
            title,
            r#type,
            what,
            why,
            where_field,
            learned,
        } => {
            let memory = Memories {
                memory_id: uuid::Uuid::new_v4().to_string(),
                session_id,
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                title,
                r#type: match r#type.as_deref() {
                    Some("observation") => Some(MemoryType::OBSERVATION),
                    Some("error") => Some(MemoryType::ERROR),
                    Some("plan") => Some(MemoryType::PLAN),
                    Some("preference") | Some("preferences") => Some(MemoryType::PREFERENCES),
                    _ => None,
                },
                what,
                why,
                where_field,
                learned,
                deleted_at: None,
            };

            let id = store_memory(&conn, &memory)?;
            println!("✅ Memoria guardada con ID: {}", id);
        }

        MemoryCommands::Search {
            query,
            session_id,
            limit,
        } => {
            let results = search_memories(&conn, &query, session_id.as_deref(), limit)?;

            if results.is_empty() {
                println!("🔍 No se encontraron resultados para '{}'", query);
            } else {
                println!(
                    "🔍 Resultados para '{}' ({} encontrados):",
                    query,
                    results.len()
                );
                for memory in results {
                    println!("   📝 [{}] {}", memory.memory_id, memory.title);
                    if let Some(learned) = memory.learned {
                        println!("      Aprendido: {}", learned);
                    }
                }
            }
        }

        MemoryCommands::List { session_id, limit } => {
            let memories = get_session_context(&conn, &session_id, limit)?;

            if memories.is_empty() {
                println!("📭 No hay memorias en la sesión {}", session_id);
            } else {
                println!(
                    "📝 Últimas {} memorias de la sesión {}:",
                    memories.len(),
                    session_id
                );
                for memory in memories {
                    println!(
                        "   [{}] {} - {}",
                        timestamp_to_string(memory.created_at),
                        memory.memory_id,
                        memory.title
                    );
                }
            }
        }

        MemoryCommands::Delete {
            memory_id,
            permanent,
        } => {
            let deleted = delete_memory(&conn, &memory_id, permanent)?;
            if deleted {
                let delete_type = if permanent {
                    "eliminada permanentemente"
                } else {
                    "marcada como eliminada"
                };
                println!("✅ Memoria {} {}", memory_id, delete_type);
            } else {
                println!("❌ Memoria {} no encontrada", memory_id);
            }
        }
    }

    Ok(())
}

fn handle_reflection_command(cmd: ReflectionCommands) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection()?;

    match cmd {
        ReflectionCommands::Generate { session_id } => {
            println!("🔄 Generando reflexión para la sesión {}...", session_id);
            let reflection = generate_reflection(&conn, &session_id)?;
            println!("✅ Reflexión generada:");
            println!("   ID: {}", reflection.reflection_id);
            println!("   Tipo: {}", reflection.r#type);
            println!("   Importancia: {:.2}", reflection.importance);
            println!("   Nivel: {}", reflection.level);
            println!("   Contenido: {}", reflection.content);
        }

        ReflectionCommands::Get { session_id } => {
            match get_reflection_by_session(&conn, &session_id)? {
                Some(reflection) => {
                    println!("📋 Reflexión de la sesión {}:", session_id);
                    println!("   ID: {}", reflection.reflection_id);
                    println!("   Tipo: {}", reflection.r#type);
                    println!("   Importancia: {:.2}", reflection.importance);
                    println!("   Nivel: {}", reflection.level);
                    println!("   Contenido: {}", reflection.content);
                    if let Some(summary) = reflection.source_summary {
                        println!("   Fuente: {}", summary);
                    }
                }
                None => {
                    println!("📭 No hay reflexión para la sesión {}", session_id);
                    println!(
                        "   Ejecuta 'mimir reflection generate --session-id {}' para generar una",
                        session_id
                    );
                }
            }
        }

        ReflectionCommands::Delete { reflection_id } => {
            let deleted = delete_reflection(&conn, &reflection_id)?;
            if deleted {
                println!("✅ Reflexión {} eliminada", reflection_id);
            } else {
                println!("❌ Reflexión {} no encontrada", reflection_id);
            }
        }
    }

    Ok(())
}

fn timestamp_to_string(timestamp: i64) -> String {
    use chrono::{DateTime, Utc};
    let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
