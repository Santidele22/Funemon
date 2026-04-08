use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mimir")]
#[command(about = "Sistema de memoria persistente para agentes de programación", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Mcp,
    /// Inicializa la base de datos (crea tablas y triggers)
    Init,
    /// Comandos para gestionar sesiones
    #[command(subcommand)]
    Session(SessionCommands),

    /// Comandos para gestionar memorias
    #[command(subcommand)]
    Memories(MemoryCommands),

    /// Comandos para gestionar reflexiones
    #[command(subcommand)]
    Reflection(ReflectionCommands),

    /// Muestra estadísticas del sistema
    Stats,
}

#[derive(Subcommand)]
pub enum SessionCommands {
    /// Inicia una nueva sesión o reanuda una existente
    Start {
        /// Nombre del proyecto
        #[arg(short, long)]
        project: String,

        /// ID de sesión existente (opcional, para reanudar)
        #[arg(short, long)]
        session_id: Option<String>,
    },

    /// Lista todas las sesiones de un proyecto
    List {
        /// Nombre del proyecto
        #[arg(short, long)]
        project: String,
    },

    /// Elimina una sesión
    Delete {
        /// ID de la sesión
        #[arg(short, long)]
        session_id: String,

        /// Eliminación permanente (por defecto soft delete)
        #[arg(short, long)]
        permanent: bool,
    },

    /// Limpia sesiones expiradas (inactivas por más de N días)
    Cleanup {
        /// Nombre del proyecto
        #[arg(short, long)]
        project: String,

        /// Días de inactividad (default: 5)
        #[arg(short, long, default_value = "5")]
        days: i64,
    },
}

#[derive(Subcommand)]
pub enum MemoryCommands {
    /// Guarda una nueva memoria
    Store {
        /// ID de la sesión
        #[arg(short, long)]
        session_id: String,

        /// Título de la memoria
        #[arg(short, long)]
        title: String,

        /// Tipo de memoria (observation, error, plan, preference)
        #[arg(short, long)]
        r#type: Option<String>,

        /// Qué ocurrió
        #[arg(short, long)]
        what: Option<String>,

        /// Por qué ocurrió
        #[arg(short, long)]
        why: Option<String>,

        /// Dónde ocurrió
        #[arg(short, long)]
        where_field: Option<String>,

        /// Lección aprendida
        #[arg(short, long)]
        learned: Option<String>,
    },

    /// Busca memorias por texto
    Search {
        /// Texto a buscar
        query: String,

        /// ID de la sesión (opcional)
        #[arg(short, long)]
        session_id: Option<String>,

        /// Límite de resultados (default: 10)
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Lista las memorias de una sesión
    List {
        /// ID de la sesión
        #[arg(short, long)]
        session_id: String,

        /// Límite de resultados (default: 20)
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    /// Elimina una memoria
    Delete {
        /// ID de la memoria
        #[arg(short, long)]
        memory_id: String,

        /// Eliminación permanente (por defecto soft delete)
        #[arg(short, long)]
        permanent: bool,
    },
}

#[derive(Subcommand)]
pub enum ReflectionCommands {
    /// Genera una reflexión para una sesión
    Generate {
        /// ID de la sesión
        #[arg(short, long)]
        session_id: String,
    },

    /// Obtiene la reflexión de una sesión
    Get {
        /// ID de la sesión
        #[arg(short, long)]
        session_id: String,
    },

    /// Elimina una reflexión
    Delete {
        /// ID de la reflexión
        #[arg(short, long)]
        reflection_id: String,
    },
}
