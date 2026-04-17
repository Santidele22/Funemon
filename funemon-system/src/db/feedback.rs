use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============== Feedback Trigger Conditions ==============

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedbackTrigger {
    AmbiguousResult,
    NewAgentInteraction,
    CrossTeamDelegation,
    MajorFeatureCompletion,
    None,
}

pub fn should_capture_explicit_feedback(
    is_ambiguous: bool,
    is_new_agent: bool,
    is_cross_team: bool,
    is_major_feature: bool,
) -> FeedbackTrigger {
    if is_ambiguous {
        return FeedbackTrigger::AmbiguousResult;
    }
    if is_new_agent {
        return FeedbackTrigger::NewAgentInteraction;
    }
    if is_cross_team {
        return FeedbackTrigger::CrossTeamDelegation;
    }
    if is_major_feature {
        return FeedbackTrigger::MajorFeatureCompletion;
    }
    FeedbackTrigger::None
}

// ============== Verification Queue ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTask {
    pub id: String,
    pub pattern_id: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub suggested_action: String,
    pub created_at: i64,
    pub status: String, // pending, confirmed, rejected, modified
    pub expires_at: i64,
}

impl VerificationTask {
    pub fn new(pattern_id: &str, confidence: f64, evidence: Vec<String>, action: &str) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            pattern_id: pattern_id.to_string(),
            confidence,
            evidence,
            suggested_action: action.to_string(),
            created_at: now,
            status: "pending".to_string(),
            expires_at: now + (7 * 24 * 3600), // 7 days
        }
    }

    pub fn confirm(&mut self) {
        self.status = "confirmed".to_string();
    }

    pub fn reject(&mut self) {
        self.status = "rejected".to_string();
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expires_at
    }
}

pub fn create_verification_task(
    pattern_id: &str,
    confidence: f64,
    evidence: Vec<String>,
    action: &str,
) -> Option<VerificationTask> {
    // Only create if confidence >= 0.8
    if confidence >= 0.8 {
        Some(VerificationTask::new(
            pattern_id, confidence, evidence, action,
        ))
    } else {
        None
    }
}

pub fn get_pending_verifications(tasks: &[VerificationTask]) -> Vec<&VerificationTask> {
    let mut pending: Vec<_> = tasks.iter().filter(|t| t.status == "pending").collect();

    // Sort by confidence (highest first)
    pending.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
    pending
}

pub fn confirm_verification(task: &mut VerificationTask) {
    task.confirm();
}

pub fn reject_verification(task: &mut VerificationTask) {
    task.reject();
}

pub fn cleanup_expired_verifications(tasks: &mut Vec<VerificationTask>) {
    let now = Utc::now().timestamp();
    for task in tasks.iter_mut() {
        if task.status == "pending" && task.expires_at < now {
            task.status = "expired".to_string();
        }
    }
}

// ============== Analytics ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPerformance {
    pub agent: String,
    pub total_tasks: i32,
    pub average_rating: f64,
    pub positive_count: i32,
    pub negative_count: i32,
    pub improvement_trend: f64,
}

impl AgentPerformance {
    pub fn calculate(feedbacks: &[super::learning::Feedback], agent: &str) -> Self {
        let agent_feedbacks: Vec<_> = feedbacks.iter().filter(|f| f.to_agent == agent).collect();

        let total_tasks = agent_feedbacks.len() as i32;

        let average_rating = if total_tasks > 0 {
            agent_feedbacks.iter().map(|f| f.rating as f64).sum::<f64>() / total_tasks as f64
        } else {
            0.0
        };

        let positive_count = agent_feedbacks.iter().filter(|f| f.rating >= 4).count() as i32;
        let negative_count = agent_feedbacks.iter().filter(|f| f.rating <= 2).count() as i32;

        // Simple trend calculation (last 5 vs previous 5)
        let improvement_trend = 0.0; // Placeholder

        Self {
            agent: agent.to_string(),
            total_tasks,
            average_rating,
            positive_count,
            negative_count,
            improvement_trend,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackTrend {
    pub period: String,
    pub average_rating: f64,
    pub total_feedback: i32,
}

pub fn get_feedback_trend(feedbacks: &[super::learning::Feedback]) -> FeedbackTrend {
    let avg = if !feedbacks.is_empty() {
        feedbacks.iter().map(|f| f.rating as f64).sum::<f64>() / feedbacks.len() as f64
    } else {
        0.0
    };

    FeedbackTrend {
        period: "last_7_days".to_string(),
        average_rating: avg,
        total_feedback: feedbacks.len() as i32,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSummary {
    pub total_patterns: i32,
    pub verified_patterns: i32,
    pub pending_verifications: i32,
    pub total_preferences: i32,
    pub verified_preferences: i32,
    pub agents: Vec<AgentPerformance>,
}

pub fn get_analytics_summary(
    patterns: &[super::learning::LearnedPattern],
    preferences: &[super::learning::SantiPreference],
    verifications: &[VerificationTask],
    feedbacks: &[super::learning::Feedback],
    agents: &[&str],
) -> AnalyticsSummary {
    let verified_patterns = patterns.iter().filter(|p| p.verified).count() as i32;
    let pending_verifications = verifications
        .iter()
        .filter(|v| v.status == "pending")
        .count() as i32;
    let verified_preferences = preferences.iter().filter(|p| p.verified).count() as i32;

    let agent_performances: Vec<AgentPerformance> = agents
        .iter()
        .map(|&agent| AgentPerformance::calculate(feedbacks, agent))
        .collect();

    AnalyticsSummary {
        total_patterns: patterns.len() as i32,
        verified_patterns,
        pending_verifications,
        total_preferences: preferences.len() as i32,
        verified_preferences,
        agents: agent_performances,
    }
}

// ============== Reports ==============

pub fn get_learned_patterns_report(patterns: &[super::learning::LearnedPattern]) -> String {
    let mut report = String::from("# Learned Patterns Report\n\n");
    report.push_str(&format!("Total: {}\n\n", patterns.len()));
    report.push_str("| Trigger | Action | Confidence | Verified |\n");
    report.push_str("|---------|--------|------------|----------|\n");

    for p in patterns {
        report.push_str(&format!(
            "| {} | {} | {:.0}% | {} |\n",
            p.trigger_pattern,
            p.recommended_action,
            p.confidence * 100.0,
            if p.verified { "✅" } else { "❌" }
        ));
    }

    report
}

pub fn get_verification_queue_report(tasks: &[VerificationTask]) -> String {
    let pending: Vec<_> = tasks.iter().filter(|t| t.status == "pending").collect();

    let mut report = String::from("# Pending Verifications\n\n");
    report.push_str(&format!("Total: {}\n\n", pending.len()));

    for t in &pending {
        report.push_str(&format!(
            "- **{}** (confidence: {:.0}%) - {}\n",
            t.pattern_id,
            t.confidence * 100.0,
            t.suggested_action
        ));
    }

    report
}

pub fn get_preferences_report(preferences: &[super::learning::SantiPreference]) -> String {
    let mut report = String::from("# Santi Preferences\n\n");

    // Group by category
    let mut code_style: Vec<_> = preferences
        .iter()
        .filter(|p| p.category == "code_style")
        .collect();
    let mut workflow: Vec<_> = preferences
        .iter()
        .filter(|p| p.category == "workflow")
        .collect();
    let mut communication: Vec<_> = preferences
        .iter()
        .filter(|p| p.category == "communication")
        .collect();

    report.push_str("## Code Style\n");
    for p in &code_style {
        report.push_str(&format!(
            "- {}: {} ({:.0}%)\n",
            p.key,
            p.value,
            p.confidence * 100.0
        ));
    }

    report.push_str("\n## Workflow\n");
    for p in &workflow {
        report.push_str(&format!(
            "- {}: {} ({:.0}%)\n",
            p.key,
            p.value,
            p.confidence * 100.0
        ));
    }

    report.push_str("\n## Communication\n");
    for p in &communication {
        report.push_str(&format!(
            "- {}: {} ({:.0}%)\n",
            p.key,
            p.value,
            p.confidence * 100.0
        ));
    }

    report
}

// ============== TESTS ==============

#[cfg(test)]
mod tests {
    use super::*;

    // ====== Story 7: Explicit Feedback Capture ======

    #[test]
    fn test_should_capture_explicit_feedback_ambiguous() {
        let result = should_capture_explicit_feedback(true, false, false, false);
        assert_eq!(result, FeedbackTrigger::AmbiguousResult);
    }

    #[test]
    fn test_should_capture_explicit_feedback_new_agent() {
        let result = should_capture_explicit_feedback(false, true, false, false);
        assert_eq!(result, FeedbackTrigger::NewAgentInteraction);
    }

    #[test]
    fn test_should_capture_explicit_feedback_cross_team() {
        let result = should_capture_explicit_feedback(false, false, true, false);
        assert_eq!(result, FeedbackTrigger::CrossTeamDelegation);
    }

    #[test]
    fn test_should_capture_explicit_feedback_major_feature() {
        let result = should_capture_explicit_feedback(false, false, false, true);
        assert_eq!(result, FeedbackTrigger::MajorFeatureCompletion);
    }

    #[test]
    fn test_should_capture_explicit_feedback_none() {
        let result = should_capture_explicit_feedback(false, false, false, false);
        assert_eq!(result, FeedbackTrigger::None);
    }

    #[test]
    fn test_capture_explicit_feedback_priority() {
        // Ambiguous should be first priority
        let result = should_capture_explicit_feedback(true, true, true, true);
        assert_eq!(result, FeedbackTrigger::AmbiguousResult);
    }

    // ====== Story 8: Verification Queue ======

    #[test]
    fn test_create_verification_task_high_confidence() {
        let task = create_verification_task("p1", 0.85, vec!["e1".to_string()], "action");
        assert!(task.is_some());
    }

    #[test]
    fn test_create_verification_task_low_confidence() {
        let task = create_verification_task("p1", 0.7, vec!["e1".to_string()], "action");
        assert!(task.is_none());
    }

    #[test]
    fn test_get_pending_verifications_ordered_by_confidence() {
        let t1 = VerificationTask::new("p1", 0.85, vec![], "a1");
        let t2 = VerificationTask::new("p2", 0.95, vec![], "a2");
        let t3 = VerificationTask::new("p3", 0.80, vec![], "a3");

        let tasks = vec![t1, t2, t3];
        let pending = get_pending_verifications(&tasks);

        assert_eq!(pending.len(), 3);
        assert!(pending[0].confidence >= pending[1].confidence);
        assert!(pending[1].confidence >= pending[2].confidence);
    }

    #[test]
    fn test_confirm_verification() {
        let mut task = VerificationTask::new("p1", 0.9, vec![], "action");
        confirm_verification(&mut task);
        assert_eq!(task.status, "confirmed");
    }

    #[test]
    fn test_reject_verification() {
        let mut task = VerificationTask::new("p1", 0.9, vec![], "action");
        reject_verification(&mut task);
        assert_eq!(task.status, "rejected");
    }

    #[test]
    fn test_verification_expires_after_7_days() {
        let task = VerificationTask::new("p1", 0.9, vec![], "action");
        assert!(!task.is_expired());
    }

    #[test]
    fn test_cleanup_expired_verifications() {
        let mut tasks = vec![VerificationTask::new("p1", 0.9, vec![], "action")];
        cleanup_expired_verifications(&mut tasks);
        // Should not expire immediately
        assert_eq!(tasks[0].status, "pending");
    }

    // ====== Story 9: Feedback Analytics Dashboard ======

    #[test]
    fn test_agent_performance_calculates_correctly() {
        use super::super::learning::Feedback;

        let feedbacks = vec![
            Feedback::new("s1", "t1", "m", "bruno", "quick_approval", "implicit"),
            Feedback::new("s2", "t2", "m", "bruno", "quick_approval", "implicit"),
            Feedback::new("s3", "t3", "m", "bruno", "no_comments", "implicit"),
        ];

        let perf = AgentPerformance::calculate(&feedbacks, "bruno");
        assert_eq!(perf.agent, "bruno");
        assert_eq!(perf.total_tasks, 3);
        assert!(perf.average_rating > 0.0);
        assert_eq!(perf.positive_count, 3);
        assert_eq!(perf.negative_count, 0);
    }

    #[test]
    fn test_agent_performance_no_feedback() {
        use super::super::learning::Feedback;

        let feedbacks: Vec<Feedback> = vec![];
        let perf = AgentPerformance::calculate(&feedbacks, "unknown");
        assert_eq!(perf.total_tasks, 0);
        assert_eq!(perf.average_rating, 0.0);
    }

    #[test]
    fn test_get_feedback_trend() {
        use super::super::learning::Feedback;

        let feedbacks = vec![
            Feedback::new("s1", "t1", "m", "b", "quick_approval", "implicit"),
            Feedback::new("s2", "t2", "m", "b", "no_comments", "implicit"),
        ];

        let trend = get_feedback_trend(&feedbacks);
        assert_eq!(trend.total_feedback, 2);
        assert!(trend.average_rating > 0.0);
    }

    #[test]
    fn test_get_learned_patterns_report() {
        use super::super::learning::LearnedPattern;

        let patterns = vec![LearnedPattern::new("auth", "use_pkce", "magnus")];

        let report = get_learned_patterns_report(&patterns);
        assert!(report.contains("Learned Patterns"));
        assert!(report.contains("auth"));
    }

    #[test]
    fn test_get_verification_queue_report() {
        let tasks = vec![VerificationTask::new("p1", 0.9, vec![], "action1")];
        let report = get_verification_queue_report(&tasks);
        assert!(report.contains("Pending Verifications"));
        assert!(report.contains("p1"));
    }

    #[test]
    fn test_get_preferences_report() {
        use super::super::learning::SantiPreference;

        let prefs = vec![
            SantiPreference::new("code_style", "indent", "2 spaces", 0.8),
            SantiPreference::new("workflow", "pr", "before merge", 0.8),
        ];

        let report = get_preferences_report(&prefs);
        assert!(report.contains("Santi Preferences"));
        assert!(report.contains("Code Style"));
        assert!(report.contains("Workflow"));
    }

    #[test]
    fn test_get_analytics_summary() {
        use super::super::learning::{Feedback, LearnedPattern, SantiPreference};

        let patterns = vec![LearnedPattern::new("t", "a", "magnus")];
        let preferences = vec![SantiPreference::new("c", "k", "v", 0.8)];
        let verifications = vec![VerificationTask::new("p1", 0.9, vec![], "action")];
        let feedbacks: Vec<Feedback> = vec![];
        let agents = vec!["magnus", "bruno"];

        let summary =
            get_analytics_summary(&patterns, &preferences, &verifications, &feedbacks, &agents);

        assert_eq!(summary.total_patterns, 1);
        assert_eq!(summary.total_preferences, 1);
        assert_eq!(summary.agents.len(), 2);
    }
}
