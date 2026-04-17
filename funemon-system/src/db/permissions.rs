use chrono::Utc;
use uuid::Uuid;

// ============== Permission Result ==============

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionResult {
    Approved,
    RequiresCheckpoint,
    Denied(String), // Contains suggestion
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionScope {
    Team,
    Project,
    Temporary,
}

// ============== Permission ==============

#[derive(Debug, Clone)]
pub struct Permission {
    pub id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub team_id: Option<String>,
    pub granted_at: i64,
    pub expires_at: Option<i64>,
    pub scope: String,
}

impl Permission {
    pub fn new(from_agent: &str, to_agent: &str, team_id: Option<&str>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from_agent: from_agent.to_string(),
            to_agent: to_agent.to_string(),
            team_id: team_id.map(|s| s.to_string()),
            granted_at: Utc::now().timestamp(),
            expires_at: None,
            scope: "temporary".to_string(),
        }
    }
}

// ============== Delegation ==============

#[derive(Debug, Clone, PartialEq)]
pub enum DelegationStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    P0, // Critical
    P1, // High
    P2, // Medium
    P3, // Low
}

#[derive(Debug, Clone)]
pub struct Delegation {
    pub id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub task_description: String,
    pub priority: String,
    pub status: String,
    pub created_at: i64,
    pub completed_at: Option<i64>,
    pub team_id: Option<String>,
}

impl Delegation {
    pub fn new(from_agent: &str, to_agent: &str, task: &str, team_id: Option<&str>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from_agent: from_agent.to_string(),
            to_agent: to_agent.to_string(),
            task_description: task.to_string(),
            priority: "normal".to_string(),
            status: "pending".to_string(),
            created_at: Utc::now().timestamp(),
            completed_at: None,
            team_id: team_id.map(|s| s.to_string()),
        }
    }

    pub fn transition_to(&mut self, new_status: &str) -> bool {
        // Valid transitions
        let valid = match (self.status.as_str(), new_status) {
            ("pending", "in_progress") => true,
            ("pending", "cancelled") => true,
            ("in_progress", "completed") => true,
            _ => false,
        };

        if valid {
            self.status = new_status.to_string();
            if new_status == "completed" {
                self.completed_at = Some(Utc::now().timestamp());
            }
        }

        valid
    }
}

// ============== Team Detection ==============

fn get_agent_team(agent: &str) -> Option<&'static str> {
    match agent {
        // Magnus Team
        "magnus" | "bruno" | "almendra" | "gabriela" => Some("magnus"),
        // Aurora Team
        "aurora" | "iris" => Some("aurora"),
        // ATLAS Team
        "atlas" => Some("atlas"),
        _ => None,
    }
}

fn is_same_team(agent1: &str, agent2: &str) -> bool {
    match (get_agent_team(agent1), get_agent_team(agent2)) {
        (Some(t1), Some(t2)) => t1 == t2,
        _ => false,
    }
}

fn is_cross_team(from: &str, to: &str) -> bool {
    match (get_agent_team(from), get_agent_team(to)) {
        (Some(t1), Some(t2)) => t1 != t2,
        _ => false,
    }
}

// ============== Permission Check ==============

pub fn check_permission(from_agent: &str, to_agent: &str) -> PermissionResult {
    // Same team = auto-approved
    if is_same_team(from_agent, to_agent) {
        return PermissionResult::Approved;
    }

    // ATLAS can only delegate internally
    if from_agent == "atlas" {
        return PermissionResult::Denied("ATLAS can only delegate internally".to_string());
    }

    // Cross-team = requires checkpoint
    if is_cross_team(from_agent, to_agent) {
        return PermissionResult::RequiresCheckpoint;
    }

    PermissionResult::Approved
}

pub fn approve_checkpoint(permission: &mut Permission) {
    permission.granted_at = Utc::now().timestamp();
    // Temporary permission expires in 1 hour
    permission.expires_at = Some(Utc::now().timestamp() + 3600);
    permission.scope = "temporary".to_string();
}

pub fn deny_checkpoint(from: &str, to: &str) -> PermissionResult {
    // Return suggestion
    if get_agent_team(from) == Some("magnus") {
        if to == "aurora" || to == "iris" {
            return PermissionResult::Denied(
                "Permission denied. Suggest: Use Aurora Team instead".to_string(),
            );
        }
    }
    if get_agent_team(from) == Some("aurora") {
        if to == "magnus" || to == "gabriela" {
            return PermissionResult::Denied(
                "Permission denied. Suggest: Use Magnus Team instead".to_string(),
            );
        }
    }
    PermissionResult::Denied("Permission denied".to_string())
}

// ============== Result-Only Communication ==============

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    FinalCompletion,
    BlockingError,
    Cancellation,
    ProgressUpdate,
    IntermediateStatus,
}

pub fn classify_message(message: &str) -> MessageType {
    let lower = message.to_lowercase();

    if lower.contains("completed") && lower.contains("pr ready") {
        return MessageType::FinalCompletion;
    }
    if lower.contains("blocking") && lower.contains("error") {
        return MessageType::BlockingError;
    }
    if lower.contains("cancelled") && lower.contains("reason") {
        return MessageType::Cancellation;
    }
    if lower.contains("progress") || lower.contains("update") {
        return MessageType::ProgressUpdate;
    }
    if lower.contains("should i")
        || lower.contains("need docs")
        || lower.contains("is this correct")
    {
        return MessageType::IntermediateStatus;
    }

    MessageType::IntermediateStatus
}

pub fn should_forward_to_tyrion(message: &str) -> bool {
    matches!(
        classify_message(message),
        MessageType::FinalCompletion | MessageType::BlockingError | MessageType::Cancellation
    )
}

// ============== Priority ==============

pub fn parse_priority(input: &str) -> String {
    match input.to_lowercase().as_str() {
        "p0" | "critical" | "urgent" => "P0".to_string(),
        "p1" | "high" => "P1".to_string(),
        "p2" | "medium" => "P2".to_string(),
        "p3" | "low" => "P3".to_string(),
        _ => "normal".to_string(),
    }
}

// ============== TESTS ==============

#[cfg(test)]
mod tests {
    use super::*;

    // ====== Story 5: Permission Matrix ======

    #[test]
    fn test_same_team_delegation_auto_approved_magnus_to_bruno() {
        let result = check_permission("magnus", "bruno");
        assert_eq!(result, PermissionResult::Approved);
    }

    #[test]
    fn test_same_team_delegation_auto_approved_aurora_to_iris() {
        let result = check_permission("aurora", "iris");
        assert_eq!(result, PermissionResult::Approved);
    }

    #[test]
    fn test_cross_team_delegation_requires_checkpoint() {
        let result = check_permission("magnus", "aurora");
        assert_eq!(result, PermissionResult::RequiresCheckpoint);
    }

    #[test]
    fn test_atlas_team_can_only_delegate_internally() {
        // ATLAS only has itself
        let result = check_permission("atlas", "atlas");
        assert_eq!(result, PermissionResult::Approved);
    }

    #[test]
    fn test_atlas_team_cannot_delegate_to_magnus() {
        let result = check_permission("atlas", "magnus");
        assert!(matches!(result, PermissionResult::Denied(_)));
    }

    #[test]
    fn test_magnus_team_cannot_auto_delegate_to_aurora_team() {
        let result = check_permission("magnus", "aurora");
        assert_eq!(result, PermissionResult::RequiresCheckpoint);
    }

    #[test]
    fn test_magnus_team_cannot_auto_delegate_to_iris() {
        let result = check_permission("gabriela", "iris");
        assert_eq!(result, PermissionResult::RequiresCheckpoint);
    }

    #[test]
    fn test_aurora_team_cannot_auto_delegate_to_magnus_team() {
        let result = check_permission("aurora", "magnus");
        assert_eq!(result, PermissionResult::RequiresCheckpoint);
    }

    #[test]
    fn test_aurora_team_cannot_auto_delegate_to_gabriela() {
        let result = check_permission("iris", "gabriela");
        assert_eq!(result, PermissionResult::RequiresCheckpoint);
    }

    // ====== Story 6: Permission Checkpoint Logic ======

    #[test]
    fn test_check_permission_same_team_returns_approved() {
        let result = check_permission("magnus", "almendra");
        assert_eq!(result, PermissionResult::Approved);
    }

    #[test]
    fn test_check_permission_cross_team_returns_requires_checkpoint() {
        let result = check_permission("bruno", "aurora");
        assert_eq!(result, PermissionResult::RequiresCheckpoint);
    }

    #[test]
    fn test_checkpoint_can_be_approved() {
        let mut permission = Permission::new("magnus", "aurora", Some("magnus"));
        approve_checkpoint(&mut permission);
        assert!(permission.expires_at.is_some());
    }

    #[test]
    fn test_checkpoint_can_be_denied() {
        let result = deny_checkpoint("magnus", "aurora");
        assert!(matches!(result, PermissionResult::Denied(_)));
    }

    #[test]
    fn test_approved_checkpoint_creates_temporary_permission() {
        let mut permission = Permission::new("magnus", "aurora", Some("magnus"));
        approve_checkpoint(&mut permission);
        assert_eq!(permission.scope, "temporary");
    }

    #[test]
    fn test_denied_checkpoint_returns_error_with_suggestion() {
        let result = deny_checkpoint("magnus", "aurora");
        if let PermissionResult::Denied(msg) = result {
            assert!(msg.contains("Aurora Team"));
        } else {
            panic!("Expected Denied variant");
        }
    }

    // ====== Story 7: Delegation State Machine ======

    #[test]
    fn test_delegation_starts_in_pending_state() {
        let d = Delegation::new("magnus", "bruno", "test task", None);
        assert_eq!(d.status, "pending");
    }

    #[test]
    fn test_pending_can_transition_to_in_progress() {
        let mut d = Delegation::new("magnus", "bruno", "test", None);
        let result = d.transition_to("in_progress");
        assert!(result);
        assert_eq!(d.status, "in_progress");
    }

    #[test]
    fn test_pending_can_transition_to_cancelled() {
        let mut d = Delegation::new("magnus", "bruno", "test", None);
        let result = d.transition_to("cancelled");
        assert!(result);
        assert_eq!(d.status, "cancelled");
    }

    #[test]
    fn test_in_progress_can_transition_to_completed() {
        let mut d = Delegation::new("magnus", "bruno", "test", None);
        d.transition_to("in_progress");
        let result = d.transition_to("completed");
        assert!(result);
        assert_eq!(d.status, "completed");
        assert!(d.completed_at.is_some());
    }

    #[test]
    fn test_pending_cannot_transition_directly_to_completed() {
        let mut d = Delegation::new("magnus", "bruno", "test", None);
        let result = d.transition_to("completed");
        assert!(!result);
        assert_eq!(d.status, "pending");
    }

    #[test]
    fn test_completed_cannot_transition() {
        let mut d = Delegation::new("magnus", "bruno", "test", None);
        d.transition_to("in_progress");
        d.transition_to("completed");
        let result = d.transition_to("cancelled");
        assert!(!result);
        assert_eq!(d.status, "completed");
    }

    #[test]
    fn test_cancelled_cannot_transition() {
        let mut d = Delegation::new("magnus", "bruno", "test", None);
        d.transition_to("cancelled");
        let result = d.transition_to("in_progress");
        assert!(!result);
        assert_eq!(d.status, "cancelled");
    }

    #[test]
    fn test_valid_state_machine_transitions() {
        let mut d = Delegation::new("magnus", "bruno", "test", None);

        // Valid: pending → in_progress
        assert!(d.transition_to("in_progress"));

        // Valid: in_progress → completed
        assert!(d.transition_to("completed"));
    }

    // ====== Story 8: Result-Only Communication ======

    #[test]
    fn test_system_filters_non_final_messages() {
        assert!(!should_forward_to_tyrion("Should I write tests now?"));
        assert!(!should_forward_to_tyrion("Need docs?"));
    }

    #[test]
    fn test_only_final_results_reach_tyrion() {
        assert!(should_forward_to_tyrion("Feature X completed, PR ready"));
    }

    #[test]
    fn test_blocking_errors_allowed_through() {
        assert!(should_forward_to_tyrion("Blocking error, need decision"));
    }

    #[test]
    fn test_progress_updates_are_filtered() {
        assert!(!should_forward_to_tyrion("Progress update: 50%"));
    }

    #[test]
    fn test_cancellation_reaches_tyrion() {
        assert!(should_forward_to_tyrion("Feature X cancelled: reason"));
    }

    #[test]
    fn test_intermediate_status_filtered() {
        assert!(!should_forward_to_tyrion("Is this correct?"));
        assert!(!should_forward_to_tyrion("Should I continue?"));
    }
}
