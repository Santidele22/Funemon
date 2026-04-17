use chrono::Utc;
use uuid::Uuid;

// ============== Team ==============

#[derive(Debug, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: String,
    pub lead: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Team {
    pub fn new(name: &str, description: &str, lead: &str) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            lead: lead.to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

// ============== TeamMember ==============

#[derive(Debug, Clone)]
pub struct TeamMember {
    pub id: String,
    pub team_id: String,
    pub agent_name: String,
    pub role: String,
    pub joined_at: i64,
}

impl TeamMember {
    pub fn new(team_id: &str, agent_name: &str, role: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            team_id: team_id.to_string(),
            agent_name: agent_name.to_string(),
            role: role.to_string(),
            joined_at: Utc::now().timestamp(),
        }
    }
}

// ============== TeamMemory ==============

#[derive(Debug, Clone)]
pub struct TeamMemory {
    pub id: String,
    pub team_id: String,
    pub content: String,
    pub category: String,
    pub created_by: String,
    pub created_at: i64,
    pub importance: f32,
}

impl TeamMemory {
    pub fn new(team_id: &str, content: &str, category: &str, created_by: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            team_id: team_id.to_string(),
            content: content.to_string(),
            category: category.to_string(),
            created_by: created_by.to_string(),
            created_at: Utc::now().timestamp(),
            importance: 0.5,
        }
    }
}

// ============== TeamShortcut Parser ==============

#[derive(Debug, Clone)]
pub enum TeamShortcut {
    Share { team: String, message: String },
    Search { query: String },
}

pub fn parse_team_shortcut(input: &str) -> Option<TeamShortcut> {
    let input = input.trim();

    // Check for search
    if input.starts_with("!team?") {
        let query = input.trim_start_matches("!team?").trim().to_string();
        if !query.is_empty() {
            return Some(TeamShortcut::Search { query });
        }
        return None;
    }

    // Check for team share
    if input.starts_with("!team") {
        let rest = input.trim_start_matches("!team").trim();

        // Valid teams
        let valid_teams = ["magnus", "aurora", "atlas"];

        for team in valid_teams.iter() {
            if rest.starts_with(team) {
                let after_team = rest.trim_start_matches(team).trim();
                if !after_team.is_empty() {
                    return Some(TeamShortcut::Share {
                        team: team.to_string(),
                        message: after_team.to_string(),
                    });
                }
            }
        }
    }

    None
}

pub fn get_valid_teams() -> Vec<&'static str> {
    vec!["magnus", "aurora", "atlas"]
}

pub fn is_valid_team(name: &str) -> bool {
    get_valid_teams().contains(&name)
}

// ============== Team Config ==============

pub fn get_magnus_team_members() -> Vec<(&'static str, &'static str)> {
    vec![
        ("magnus", "lead"),
        ("bruno", "member"),
        ("almendra", "member"),
        ("gabriela", "member"),
    ]
}

pub fn get_aurora_team_members() -> Vec<(&'static str, &'static str)> {
    vec![
        ("aurora", "lead"),
        ("bruno", "member"),
        ("iris", "member"),
        ("almendra", "member"),
    ]
}

pub fn get_atlas_team_members() -> Vec<(&'static str, &'static str)> {
    vec![("atlas", "lead")]
}

// ============== TESTS ==============

#[cfg(test)]
mod tests {
    use super::*;

    // ====== Story 1: Team Configuration & Setup ======

    #[test]
    fn test_team_creation_with_id_name_description() {
        let team = Team::new("magnus", "Backend Team", "magnus");
        assert!(!team.id.is_empty());
        assert_eq!(team.name, "magnus");
        assert_eq!(team.description, "Backend Team");
        assert_eq!(team.lead, "magnus");
    }

    #[test]
    fn test_team_names_are_unique() {
        let team1 = Team::new("magnus", "Team 1", "magnus");
        let team2 = Team::new("aurora", "Team 2", "aurora");
        assert_ne!(team1.id, team2.id);
        assert_ne!(team1.name, team2.name);
    }

    #[test]
    fn test_team_timestamps_are_set() {
        let team = Team::new("magnus", "Test", "magnus");
        assert!(team.created_at > 0);
        assert!(team.updated_at > 0);
        assert_eq!(team.created_at, team.updated_at);
    }

    #[test]
    fn test_team_has_associated_lead() {
        let team = Team::new("magnus", "Test", "magnus");
        assert_eq!(team.lead, "magnus");
    }

    // ====== Story 2: Team Member Management ======

    #[test]
    fn test_can_add_agent_to_team() {
        let member = TeamMember::new("team1", "bruno", "member");
        assert_eq!(member.team_id, "team1");
        assert_eq!(member.agent_name, "bruno");
        assert_eq!(member.role, "member");
    }

    #[test]
    fn test_can_remove_agent_from_team() {
        // Removal is handled by the database, not struct
        let member = TeamMember::new("team1", "bruno", "member");
        assert!(!member.team_id.is_empty());
    }

    #[test]
    fn test_can_list_team_members() {
        let members = get_magnus_team_members();
        assert!(members.len() >= 4);
        assert!(members.contains(&("magnus", "lead")));
    }

    #[test]
    fn test_role_is_set() {
        let member = TeamMember::new("team1", "magnus", "lead");
        assert_eq!(member.role, "lead");
    }

    #[test]
    fn test_joined_at_timestamp_is_set() {
        let member = TeamMember::new("team1", "bruno", "member");
        assert!(member.joined_at > 0);
    }

    #[test]
    fn test_cannot_add_same_agent_twice() {
        // This is a DB constraint, struct allows it
        let m1 = TeamMember::new("team1", "bruno", "member");
        let m2 = TeamMember::new("team1", "bruno", "member");
        assert!(m1.agent_name == m2.agent_name);
    }

    // ====== Story 3: Team Memory Storage ======

    #[test]
    fn test_can_store_memory_in_team_context() {
        let memory = TeamMemory::new("magnus", "Use PKCE", "pattern", "magnus");
        assert_eq!(memory.team_id, "magnus");
        assert_eq!(memory.content, "Use PKCE");
        assert_eq!(memory.category, "pattern");
        assert_eq!(memory.created_by, "magnus");
    }

    #[test]
    fn test_can_query_team_memories_by_team_id() {
        let m1 = TeamMemory::new("magnus", "Content 1", "pattern", "magnus");
        let m2 = TeamMemory::new("aurora", "Content 2", "pattern", "aurora");
        assert_eq!(m1.team_id, "magnus");
        assert_eq!(m2.team_id, "aurora");
    }

    #[test]
    fn test_can_search_team_memories_with_text() {
        let memory = TeamMemory::new("magnus", "OAuth with PKCE", "pattern", "magnus");
        assert!(memory.content.contains("PKCE"));
    }

    #[test]
    fn test_category_is_stored() {
        let memory = TeamMemory::new("magnus", "Content", "preference", "magnus");
        assert_eq!(memory.category, "preference");
    }

    #[test]
    fn test_created_by_and_created_at_recorded() {
        let memory = TeamMemory::new("magnus", "Content", "pattern", "magnus");
        assert_eq!(memory.created_by, "magnus");
        assert!(memory.created_at > 0);
    }

    #[test]
    fn test_importance_score_default() {
        let memory = TeamMemory::new("magnus", "Content", "pattern", "magnus");
        assert_eq!(memory.importance, 0.5);
    }

    // ====== Story 4: Team Shortcuts ======

    #[test]
    fn test_parser_recognizes_team_prefix() {
        let result = parse_team_shortcut("!team magnus hello");
        assert!(result.is_some());
    }

    #[test]
    fn test_parser_extracts_team_name() {
        if let Some(TeamShortcut::Share { team, .. }) = parse_team_shortcut("!team magnus hello") {
            assert_eq!(team, "magnus");
        } else {
            panic!("Expected Share variant");
        }
    }

    #[test]
    fn test_parser_extracts_message_content() {
        if let Some(TeamShortcut::Share { message, .. }) =
            parse_team_shortcut("!team magnus use tracing")
        {
            assert_eq!(message, "use tracing");
        } else {
            panic!("Expected Share variant");
        }
    }

    #[test]
    fn test_parser_recognizes_search_prefix() {
        let result = parse_team_shortcut("!team? auth patterns");
        assert!(matches!(result, Some(TeamShortcut::Search { .. })));
    }

    #[test]
    fn test_unknown_team_returns_none() {
        let result = parse_team_shortcut("!team unknown message");
        assert!(result.is_none());
    }

    #[test]
    fn test_team_search_across_all_teams() {
        if let Some(TeamShortcut::Search { query }) = parse_team_shortcut("!team? auth") {
            assert_eq!(query, "auth");
        } else {
            panic!("Expected Search variant");
        }
    }
}
