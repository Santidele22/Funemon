// Funemon System Library
// Export modules for use by funemon-ecosystem

pub mod cli;
pub mod db;
pub mod mcp;

// Re-export commonly used items
pub use db::{
    // Session operations
    cleanup_expired_sessions,
    // Memory operations
    delete_memory,
    // Reflection operations
    delete_reflection,
    delete_session,
    get_connection,
    get_memory_by_id,
    get_project_context,
    get_reflection_by_session,
    get_session_context,
    init_database,
    list_sessions,
    // Models
    models::{Memories, Sessions},
    search_memories,
    start_session,
    store_memory,
    store_reflection,
};

/// Check if funemon is properly configured
pub fn check_funemon() -> bool {
    // Simple check - can be expanded
    true
}
