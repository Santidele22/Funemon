// Integration tests for get_project_context functionality
//
// Test cases covered:
// 1. Basic functionality
//    - Project with multiple sessions returns memories from all sessions
//    - Project with no sessions returns empty vec
//    - Project with memories respects limit parameter
//
// 2. Edge cases
//    - Invalid project name (empty string)
//    - Limit = 0 edge case
//    - Limit larger than available memories
//
// 3. Ordering
//    - Memories returned in DESC order by created_at
//    - Most recent memory first
//
// 4. Integration
//    - Works with existing session system
//    - Deleted sessions don't contribute memories
//    - Deleted memories are not returned
//    - Multiple projects are isolated

use funemon::{get_project_context, get_session_context, start_session, store_memory, Memories};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

/// Helper to create an in-memory test database
fn setup_test_db() -> Arc<Mutex<Connection>> {
    // Create in-memory database
    let conn = Connection::open_in_memory().expect("Failed to create in-memory db");

    // Initialize schema
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
    )
    .expect("Failed to create sessions table");

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
    )
    .expect("Failed to create memories table");

    Arc::new(Mutex::new(conn))
}

/// Helper to create a test session
fn create_test_session(conn: &Arc<Mutex<Connection>>, project: &str) -> String {
    let session = start_session(conn, project, None).expect("Failed to create session");
    session.session_id
}

/// Helper to create a test memory
fn create_test_memory(
    conn: &Arc<Mutex<Connection>>,
    session_id: &str,
    title: &str,
    created_at: i64,
) -> String {
    let memory = Memories {
        memory_id: uuid::Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        created_at,
        title: title.to_string(),
        r#type: None,
        what: None,
        why: None,
        where_field: None,
        learned: None,
        deleted_at: None,
    };

    store_memory(conn, &memory).expect("Failed to store memory")
}

// ═══════════════════════════════════════════════════════════════════════════
// BASIC FUNCTIONALITY TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_project_context_basic_multiple_sessions() {
    // Test: Project with multiple sessions → returns memories from all sessions

    let conn = setup_test_db();

    // Create project with 2 sessions
    let session1 = create_test_session(&conn, "test-project-1");
    let session2 = create_test_session(&conn, "test-project-1");

    // Add memories to both sessions
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    create_test_memory(&conn, &session1, "Memory from session 1", now);
    create_test_memory(&conn, &session2, "Memory from session 2", now);

    // Get project context
    let memories =
        get_project_context(&conn, "test-project-1", 10).expect("Failed to get project context");

    // Assert: Returns memories from all sessions
    assert_eq!(memories.len(), 2);
    let titles: Vec<&str> = memories.iter().map(|m| m.title.as_str()).collect();
    assert!(titles.contains(&"Memory from session 1"));
    assert!(titles.contains(&"Memory from session 2"));
}

#[test]
fn test_project_context_empty_project() {
    // Test: Project with no sessions → returns empty vec

    let conn = setup_test_db();

    // Get context for non-existent project
    let memories = get_project_context(&conn, "non-existent-project", 10)
        .expect("Failed to get project context");

    // Assert: Returns empty vec
    assert_eq!(memories.len(), 0);
}

#[test]
fn test_project_context_limit_parameter() {
    // Test: Project with memories → respects limit parameter

    let conn = setup_test_db();

    // Create session with multiple memories
    let session = create_test_session(&conn, "test-project-2");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Create 10 memories
    for i in 0..10 {
        create_test_memory(&conn, &session, &format!("Memory {}", i), now + i);
    }

    // Get project context with limit 5
    let memories =
        get_project_context(&conn, "test-project-2", 5).expect("Failed to get project context");

    // Assert: Returns only 5 memories
    assert_eq!(memories.len(), 5);
}

// ═══════════════════════════════════════════════════════════════════════════
// EDGE CASE TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_project_context_empty_project_name() {
    // Test: Invalid project name (empty string) → should handle gracefully

    let conn = setup_test_db();

    // Get context with empty project name
    let result = get_project_context(&conn, "", 10);

    // Should succeed but return empty vec (no sessions match empty project)
    assert!(result.is_ok());
    let memories = result.unwrap();
    assert_eq!(memories.len(), 0);
}

#[test]
fn test_project_context_limit_zero() {
    // Test: Limit = 0 → edge case behavior

    let conn = setup_test_db();

    // Create session with memories
    let session = create_test_session(&conn, "test-project-3");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    create_test_memory(&conn, &session, "Test memory", now);

    // Get context with limit 0
    let memories =
        get_project_context(&conn, "test-project-3", 0).expect("Failed to get project context");

    // SQLite LIMIT 0 returns empty result set
    assert_eq!(memories.len(), 0);
}

#[test]
fn test_project_context_limit_larger_than_available() {
    // Test: Limit larger than available memories → returns all available

    let conn = setup_test_db();

    // Create session with 2 memories
    let session = create_test_session(&conn, "test-project-4");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    create_test_memory(&conn, &session, "Memory 1", now);
    create_test_memory(&conn, &session, "Memory 2", now + 1);

    // Get context with limit 100 (larger than available)
    let memories =
        get_project_context(&conn, "test-project-4", 100).expect("Failed to get project context");

    // Assert: Returns all available memories (only 2)
    assert_eq!(memories.len(), 2);
}

#[test]
fn test_project_context_special_characters_in_name() {
    // Test: Project name with special characters → should handle gracefully

    let conn = setup_test_db();

    // Try with special characters
    let session = create_test_session(&conn, "test-project-特殊字符-émoji-🎉");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    create_test_memory(&conn, &session, "Special memory", now);

    // Get context
    let memories = get_project_context(&conn, "test-project-特殊字符-émoji-🎉", 10)
        .expect("Failed to get project context");

    // Assert: Should return the memory
    assert_eq!(memories.len(), 1);
    assert_eq!(memories[0].title, "Special memory");
}

// ═══════════════════════════════════════════════════════════════════════════
// ORDERING TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_project_context_ordering_desc() {
    // Test: Memories returned in DESC order by created_at

    let conn = setup_test_db();

    // Create session
    let session = create_test_session(&conn, "test-project-5");
    let base_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Create memories with different timestamps
    create_test_memory(&conn, &session, "Oldest memory", base_time);
    create_test_memory(&conn, &session, "Middle memory", base_time + 100);
    create_test_memory(&conn, &session, "Newest memory", base_time + 200);

    // Get context
    let memories =
        get_project_context(&conn, "test-project-5", 10).expect("Failed to get project context");

    // Assert:Ordered DESC by created_at
    assert_eq!(memories.len(), 3);
    assert_eq!(memories[0].title, "Newest memory");
    assert_eq!(memories[1].title, "Middle memory");
    assert_eq!(memories[2].title, "Oldest memory");
}

#[test]
fn test_project_context_most_recent_first() {
    // Test: Most recent memory first

    let conn = setup_test_db();

    // Create session
    let session = create_test_session(&conn, "test-project-6");
    let base_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Create multiple memories
    for i in 0..5 {
        create_test_memory(&conn, &session, &format!("Memory {}", i), base_time + i);
    }

    // Get context
    let memories =
        get_project_context(&conn, "test-project-6", 3).expect("Failed to get project context");

    // Assert: First memory is the most recent
    assert_eq!(memories.len(), 3);
    assert_eq!(memories[0].title, "Memory 4"); // Most recent

    // Verify timestamps are descending
    for i in 1..memories.len() {
        assert!(memories[i - 1].created_at >= memories[i].created_at);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_project_context_deleted_sessions() {
    // Test: Deleted sessions don't contribute memories

    let conn = setup_test_db();

    // Create session
    let session = create_test_session(&conn, "test-project-7");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    create_test_memory(&conn, &session, "Active memory", now);

    // Create second session and then delete it
    let deleted_session = create_test_session(&conn, "test-project-7");
    create_test_memory(&conn, &deleted_session, "Deleted session memory", now);

    // Soft delete the session
    let delete_time = now + 1000;
    {
        let conn_guard = conn.lock().unwrap();
        conn_guard
            .execute(
                "UPDATE sessions SET deleted_at = ?1 WHERE session_id = ?2",
                rusqlite::params![delete_time, deleted_session],
            )
            .expect("Failed to delete session");
    }

    // Get project context
    let memories =
        get_project_context(&conn, "test-project-7", 10).expect("Failed to get project context");

    // Assert: Only memories from active session
    assert_eq!(memories.len(), 1);
    assert_eq!(memories[0].title, "Active memory");
}

#[test]
fn test_project_context_deleted_memories() {
    // Test: Deleted memories are not returned

    let conn = setup_test_db();

    // Create session
    let session = create_test_session(&conn, "test-project-8");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let _mem1 = create_test_memory(&conn, &session, "Active memory", now);
    let mem2 = create_test_memory(&conn, &session, "Deleted memory", now + 1);

    // Soft delete one memory
    let delete_time = now + 2000;
    {
        let conn_guard = conn.lock().unwrap();
        conn_guard
            .execute(
                "UPDATE memories SET deleted_at = ?1 WHERE memory_id = ?2",
                rusqlite::params![delete_time, mem2],
            )
            .expect("Failed to delete memory");
    }

    // Get project context
    let memories =
        get_project_context(&conn, "test-project-8", 10).expect("Failed to get project context");

    // Assert: Only non-deleted memories
    assert_eq!(memories.len(), 1);
    assert_eq!(memories[0].title, "Active memory");
}

#[test]
fn test_project_context_multiple_projects_isolation() {
    // Test: Multiple projects are isolated

    let conn = setup_test_db();

    // Create sessions for different projects
    let session_proj1 = create_test_session(&conn, "project-alpha");
    let session_proj2 = create_test_session(&conn, "project-beta");

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    create_test_memory(&conn, &session_proj1, "Alpha memory 1", now);
    create_test_memory(&conn, &session_proj1, "Alpha memory 2", now + 1);
    create_test_memory(&conn, &session_proj2, "Beta memory 1", now);

    // Get context for project-alpha
    let memories_alpha =
        get_project_context(&conn, "project-alpha", 10).expect("Failed to get project context");

    // Get context for project-beta
    let memories_beta =
        get_project_context(&conn, "project-beta", 10).expect("Failed to get project context");

    // Assert: Each project only returns its own memories
    assert_eq!(memories_alpha.len(), 2);
    assert_eq!(memories_beta.len(), 1);

    let alpha_titles: Vec<&str> = memories_alpha.iter().map(|m| m.title.as_str()).collect();
    assert!(alpha_titles.contains(&"Alpha memory 1"));
    assert!(alpha_titles.contains(&"Alpha memory 2"));
    assert!(!alpha_titles.contains(&"Beta memory 1"));

    assert_eq!(memories_beta[0].title, "Beta memory 1");
}

#[test]
fn test_project_context_vs_session_context() {
    // Test: Works with existing session system
    // Verify that both get_session_context and get_project_context work independently

    let conn = setup_test_db();

    // Create two sessions in the same project
    let session1 = create_test_session(&conn, "test-project-9");
    let session2 = create_test_session(&conn, "test-project-9");

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    create_test_memory(&conn, &session1, "Session 1 memory", now);
    create_test_memory(&conn, &session2, "Session 2 memory", now + 1);

    // Get session context (only memories from session 1)
    let session_memories =
        get_session_context(&conn, &session1, 10).expect("Failed to get session context");

    // Get project context (memories from all sessions in project)
    let project_memories =
        get_project_context(&conn, "test-project-9", 10).expect("Failed to get project context");

    // Assert: Session context returns only session memories
    assert_eq!(session_memories.len(), 1);
    assert_eq!(session_memories[0].title, "Session 1 memory");

    // Assert: Project context returns all project memories
    assert_eq!(project_memories.len(), 2);
}

#[test]
fn test_project_context_cross_project_isolation() {
    // Test: Verify cross-project isolation with simultaneous requests

    let conn = setup_test_db();

    // Setup multiple projects with distinct data
    let projects = vec!["project-A", "project-B", "project-C"];
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    for (i, project) in projects.iter().enumerate() {
        let session = create_test_session(&conn, project);
        create_test_memory(
            &conn,
            &session,
            &format!("Memory for {}", project),
            now + i as i64,
        );
    }

    // Test each project in isolation using assertions
    let memories_a = get_project_context(&conn, "project-A", 10).unwrap();
    assert_eq!(memories_a.len(), 1);
    assert!(memories_a[0].title.contains("project-A"));

    let memories_b = get_project_context(&conn, "project-B", 10).unwrap();
    assert_eq!(memories_b.len(), 1);
    assert!(memories_b[0].title.contains("project-B"));

    let memories_c = get_project_context(&conn, "project-C", 10).unwrap();
    assert_eq!(memories_c.len(), 1);
    assert!(memories_c[0].title.contains("project-C"));
}

// ═══════════════════════════════════════════════════════════════════════════
// ADDITIONAL EDGE CASES
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_project_context_with_limit_one() {
    // Test: Limit = 1 returns only the most recent memory

    let conn = setup_test_db();

    let session = create_test_session(&conn, "test-project-10");
    let base_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    create_test_memory(&conn, &session, "Old memory", base_time);
    create_test_memory(&conn, &session, "New memory", base_time + 100);

    let memories =
        get_project_context(&conn, "test-project-10", 1).expect("Failed to get project context");

    assert_eq!(memories.len(), 1);
    assert_eq!(memories[0].title, "New memory");
}

#[test]
fn test_project_context_timestamp_ordering_across_sessions() {
    // Test: Timestamp ordering works correctly across different sessions

    let conn = setup_test_db();

    let session1 = create_test_session(&conn, "test-project-11");
    let session2 = create_test_session(&conn, "test-project-11");

    let base_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Create memories in different sessions with different timestamps
    create_test_memory(&conn, &session1, "Session1 - Time 0", base_time);
    create_test_memory(&conn, &session2, "Session2 - Time 100", base_time + 100);
    create_test_memory(&conn, &session1, "Session1 - Time 200", base_time + 200);
    create_test_memory(&conn, &session2, "Session2 - Time 50", base_time + 50);

    let memories =
        get_project_context(&conn, "test-project-11", 10).expect("Failed to get project context");

    // Verify DESC ordering
    assert_eq!(memories.len(), 4);
    assert_eq!(memories[0].title, "Session1 - Time 200");
    assert_eq!(memories[1].title, "Session2 - Time 100");
    assert_eq!(memories[2].title, "Session2 - Time 50");
    assert_eq!(memories[3].title, "Session1 - Time 0");
}
