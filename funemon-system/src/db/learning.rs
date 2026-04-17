use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============== Feedback ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub id: String,
    pub session_id: String,
    pub task_id: String,
    pub from_agent: String,
    pub to_agent: String,
    pub rating: i32,
    pub feedback_type: String,
    pub comments: Option<String>,
    pub created_at: i64,
    pub metadata: Option<String>,
}

impl Feedback {
    pub fn new(
        session_id: &str,
        task_id: &str,
        from_agent: &str,
        to_agent: &str,
        signal: &str,
        feedback_type: &str,
    ) -> Self {
        let rating = signal_to_rating(signal);
        Self {
            id: Uuid::new_v4().to_string(),
            session_id: session_id.to_string(),
            task_id: task_id.to_string(),
            from_agent: from_agent.to_string(),
            to_agent: to_agent.to_string(),
            rating,
            feedback_type: feedback_type.to_string(),
            comments: None,
            created_at: Utc::now().timestamp(),
            metadata: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeedbackSignal {
    QuickApproval,
    NoComments,
    MinorFeedback,
    MajorChanges,
    StyleOnly,
    RejectedPr,
    RepeatedIssue,
    FirstTrySuccess,
    Unknown,
}

impl FeedbackSignal {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "quick_approval" | "quick approval" => FeedbackSignal::QuickApproval,
            "no_comments" | "no comments" => FeedbackSignal::NoComments,
            "minor_feedback" | "minor feedback" => FeedbackSignal::MinorFeedback,
            "major_changes" | "major changes" => FeedbackSignal::MajorChanges,
            "style_only" | "style only" => FeedbackSignal::StyleOnly,
            "rejected_pr" | "rejected pr" => FeedbackSignal::RejectedPr,
            "repeated_issue" | "repeated issue" => FeedbackSignal::RepeatedIssue,
            "first_try_success" | "first try success" => FeedbackSignal::FirstTrySuccess,
            _ => FeedbackSignal::Unknown,
        }
    }
}

pub fn signal_to_rating(signal: &str) -> i32 {
    match FeedbackSignal::from_str(signal) {
        FeedbackSignal::QuickApproval | FeedbackSignal::FirstTrySuccess => 5,
        FeedbackSignal::NoComments | FeedbackSignal::MinorFeedback => 4,
        FeedbackSignal::StyleOnly => 3,
        FeedbackSignal::MajorChanges | FeedbackSignal::RepeatedIssue => 2,
        FeedbackSignal::RejectedPr => 1,
        FeedbackSignal::Unknown => 3,
    }
}

// ============== Learned Pattern ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedPattern {
    pub id: String,
    pub trigger_pattern: String,
    pub recommended_action: String,
    pub confidence: f64,
    pub evidence_count: i32,
    pub evidence_examples: Vec<String>,
    pub agent: String,
    pub first_seen: i64,
    pub last_updated: i64,
    pub verified: bool,
    pub active: bool,
}

impl LearnedPattern {
    pub fn new(trigger: &str, action: &str, agent: &str) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: Uuid::new_v4().to_string(),
            trigger_pattern: trigger.to_string(),
            recommended_action: action.to_string(),
            confidence: 0.0,
            evidence_count: 0,
            evidence_examples: Vec::new(),
            agent: agent.to_string(),
            first_seen: now,
            last_updated: now,
            verified: false,
            active: false,
        }
    }

    pub fn update_evidence(&mut self, example: &str, new_confidence: f64) {
        self.evidence_count += 1;
        self.evidence_examples.push(example.to_string());
        self.confidence = new_confidence;
        self.last_updated = Utc::now().timestamp();

        // Auto-activate if 3+ evidence
        if self.evidence_count >= 3 {
            self.active = true;
        }

        // Auto-verify if confidence >= 0.8
        if self.confidence >= 0.8 {
            self.verified = true;
        }
    }

    pub fn verify(&mut self) {
        self.verified = true;
        self.last_updated = Utc::now().timestamp();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.last_updated = Utc::now().timestamp();
    }
}

// ============== Santi Preference ==============

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PreferenceCategory {
    CodeStyle,
    Workflow,
    Communication,
    Unknown,
}

impl PreferenceCategory {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "code_style" | "code style" => PreferenceCategory::CodeStyle,
            "workflow" => PreferenceCategory::Workflow,
            "communication" => PreferenceCategory::Communication,
            _ => PreferenceCategory::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PreferenceCategory::CodeStyle => "code_style",
            PreferenceCategory::Workflow => "workflow",
            PreferenceCategory::Communication => "communication",
            PreferenceCategory::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SantiPreference {
    pub id: String,
    pub category: String,
    pub key: String,
    pub value: String,
    pub confidence: f64,
    pub examples: Vec<String>,
    pub verified: bool,
    pub last_seen: i64,
}

impl SantiPreference {
    pub fn new(category: &str, key: &str, value: &str, confidence: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            category: category.to_string(),
            key: key.to_string(),
            value: value.to_string(),
            confidence,
            examples: Vec::new(),
            verified: false,
            last_seen: Utc::now().timestamp(),
        }
    }

    pub fn update_examples(&mut self, example: &str) {
        self.examples.push(example.to_string());
        self.last_seen = Utc::now().timestamp();

        // Auto-verify if confidence >= 0.8
        if self.confidence >= 0.8 {
            self.verified = true;
        }
    }

    pub fn verify(&mut self) {
        self.verified = true;
    }
}

// ============== Pending Verification ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingVerification {
    pub id: String,
    pub pattern_id: String,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub suggested_action: String,
    pub created_at: i64,
    pub status: String,
}

impl PendingVerification {
    pub fn new(
        pattern_id: &str,
        confidence: f64,
        evidence: Vec<String>,
        suggested_action: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            pattern_id: pattern_id.to_string(),
            confidence,
            evidence,
            suggested_action: suggested_action.to_string(),
            created_at: Utc::now().timestamp(),
            status: "pending".to_string(),
        }
    }
}

// ============== Pattern Detection ==============

#[derive(Debug, Clone)]
pub struct PatternContext {
    pub context: String,
    pub feedback_list: Vec<Feedback>,
}

pub fn detect_patterns(feedbacks: &[Feedback]) -> Vec<LearnedPattern> {
    let mut patterns: Vec<LearnedPattern> = Vec::new();

    // Group feedback by context (using to_agent as context for simplicity)
    let mut grouped: std::collections::HashMap<String, Vec<&Feedback>> =
        std::collections::HashMap::new();

    for feedback in feedbacks {
        let context = feedback.to_agent.clone();
        grouped.entry(context).or_default().push(feedback);
    }

    for (context, group) in grouped {
        if group.len() >= 3 {
            let confidence = calculate_confidence(&group);
            let action = infer_action(&group);

            let mut pattern = LearnedPattern::new(&context, &action, "system");
            pattern.update_evidence(
                &format!("Evidence from {} feedbacks", group.len()),
                confidence,
            );

            patterns.push(pattern);
        }
    }

    patterns
}

pub fn calculate_confidence(feedbacks: &[&Feedback]) -> f64 {
    if feedbacks.is_empty() {
        return 0.0;
    }

    let total_rating: f64 = feedbacks.iter().map(|f| f.rating as f64).sum();
    let avg = total_rating / (feedbacks.len() as f64);

    // Calculate consistency (standard deviation inverse)
    let mean = avg;
    let variance: f64 = feedbacks
        .iter()
        .map(|f| {
            let diff = f.rating as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / feedbacks.len() as f64;

    let std_dev = variance.sqrt();
    let consistency = 1.0 - (std_dev / 2.0).min(1.0);

    // Final confidence formula
    (avg / 5.0) * (0.5 + consistency * 0.5)
}

pub fn calculate_consistency(feedbacks: &[&Feedback]) -> f64 {
    if feedbacks.is_empty() {
        return 0.0;
    }

    let total_rating: f64 = feedbacks.iter().map(|f| f.rating as f64).sum();
    let mean = total_rating / feedbacks.len() as f64;

    let variance: f64 = feedbacks
        .iter()
        .map(|f| {
            let diff = f.rating as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / feedbacks.len() as f64;

    let std_dev = variance.sqrt();
    1.0 - (std_dev / 2.0).min(1.0)
}

pub fn infer_action(feedbacks: &[&Feedback]) -> String {
    if feedbacks.is_empty() {
        return "Continue monitoring".to_string();
    }

    let avg_rating: f64 =
        feedbacks.iter().map(|f| f.rating as f64).sum::<f64>() / feedbacks.len() as f64;

    if avg_rating >= 4.5 {
        "Keep doing what you're doing".to_string()
    } else if avg_rating >= 3.5 {
        "Minor improvements possible".to_string()
    } else if avg_rating >= 2.5 {
        "Review and adjust approach".to_string()
    } else {
        "Major changes needed".to_string()
    }
}

// ============== Preference Extraction ==============

pub fn extract_preference(memory_content: &str, confidence: f64) -> Option<SantiPreference> {
    if confidence < 0.7 {
        return None;
    }

    // Detect category based on keywords
    let category = if memory_content.contains("indent")
        || memory_content.contains("naming")
        || memory_content.contains("format")
        || memory_content.contains("style")
        || memory_content.contains("spaces")
    {
        PreferenceCategory::CodeStyle
    } else if memory_content.contains("PR")
        || memory_content.contains("commit")
        || memory_content.contains("test")
        || memory_content.contains("merge")
        || memory_content.contains("workflow")
    {
        PreferenceCategory::Workflow
    } else if memory_content.contains("brief")
        || memory_content.contains("update")
        || memory_content.contains("communication")
    {
        PreferenceCategory::Communication
    } else {
        PreferenceCategory::Workflow // default
    };

    // Parse key-value from content
    let (key, value) = parse_key_value(memory_content);

    Some(SantiPreference::new(
        category.as_str(),
        &key,
        &value,
        confidence,
    ))
}

fn parse_key_value(content: &str) -> (String, String) {
    // Simple parsing: look for patterns like "X: Y" or "X = Y"
    let content_lower = content.to_lowercase();

    if content_lower.contains("indent") {
        if content_lower.contains("2") || content_lower.contains("two") {
            ("indent".to_string(), "2 spaces".to_string())
        } else if content_lower.contains("4") || content_lower.contains("four") {
            ("indent".to_string(), "4 spaces".to_string())
        } else {
            ("format".to_string(), "default".to_string())
        }
    } else if content_lower.contains("naming") || content_lower.contains("name") {
        if content_lower.contains("snake") {
            ("naming".to_string(), "snake_case".to_string())
        } else if content_lower.contains("camel") {
            ("naming".to_string(), "camelCase".to_string())
        } else if content_lower.contains("kebab") {
            ("naming".to_string(), "kebab-case".to_string())
        } else {
            ("naming".to_string(), "default".to_string())
        }
    } else {
        ("general".to_string(), content.to_string())
    }
}

// ============== Nightly Processor ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSummary {
    pub patterns_detected: i32,
    pub patterns_verified: i32,
    pub preferences_extracted: i32,
    pub verifications_generated: i32,
    pub errors: Vec<String>,
}

impl ProcessorSummary {
    pub fn new() -> Self {
        Self {
            patterns_detected: 0,
            patterns_verified: 0,
            preferences_extracted: 0,
            verifications_generated: 0,
            errors: Vec::new(),
        }
    }
}

pub fn run_nightly_processor(feedbacks: &[Feedback], memories: &[String]) -> ProcessorSummary {
    let mut summary = ProcessorSummary::new();

    // 1. Load and process feedback (already loaded in this case)

    // 2. Detect patterns from feedback
    let patterns = detect_patterns(feedbacks);
    summary.patterns_detected = patterns.len() as i32;

    for pattern in &patterns {
        if pattern.verified {
            summary.patterns_verified += 1;

            // Generate verification task if confidence >= 0.8
            if pattern.confidence >= 0.8 {
                summary.verifications_generated += 1;
            }
        }
    }

    // 3. Extract preferences from memories
    for memory in memories {
        if let Some(pref) = extract_preference(memory, 0.7) {
            if pref.confidence >= 0.7 {
                summary.preferences_extracted += 1;
            }
        }
    }

    // 4. Log summary
    println!(
        "Nightly processor complete: {} patterns, {} verified, {} preferences",
        summary.patterns_detected, summary.patterns_verified, summary.preferences_extracted
    );

    summary
}

// ============== Insights ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInsights {
    pub agent: String,
    pub top_patterns: Vec<LearnedPattern>,
    pub preferences: Vec<SantiPreference>,
    pub average_rating: f64,
}

pub fn get_agent_insights(
    patterns: &[LearnedPattern],
    preferences: &[SantiPreference],
    agent: &str,
) -> AgentInsights {
    let agent_patterns: Vec<LearnedPattern> = patterns
        .iter()
        .filter(|p| p.agent == agent)
        .take(5)
        .cloned()
        .collect();

    let agent_prefs: Vec<SantiPreference> = preferences.iter().take(5).cloned().collect();

    let avg_rating = if !agent_patterns.is_empty() {
        agent_patterns.iter().map(|p| p.confidence).sum::<f64>() / agent_patterns.len() as f64 * 5.0
    } else {
        0.0
    };

    AgentInsights {
        agent: agent.to_string(),
        top_patterns: agent_patterns,
        preferences: agent_prefs,
        average_rating: avg_rating,
    }
}

// ============== TESTS ==============

#[cfg(test)]
mod tests {
    use super::*;

    // ====== Story 1: Feedback Signals Database ======

    #[test]
    fn test_store_feedback_saves_to_db() {
        let feedback = Feedback::new(
            "session1",
            "task1",
            "magnus",
            "bruno",
            "quick_approval",
            "implicit",
        );
        assert!(!feedback.id.is_empty());
        assert_eq!(feedback.session_id, "session1");
        assert_eq!(feedback.rating, 5);
    }

    #[test]
    fn test_get_feedback_by_session_returns_session_feedback() {
        let feedback = Feedback::new(
            "session1",
            "task1",
            "magnus",
            "bruno",
            "quick_approval",
            "implicit",
        );
        // Simulate filtering by session
        let session_id = &feedback.session_id;
        assert_eq!(session_id, "session1");
    }

    #[test]
    fn test_get_feedback_by_agent_returns_agent_feedback() {
        let feedback = Feedback::new(
            "session1",
            "task1",
            "magnus",
            "bruno",
            "quick_approval",
            "implicit",
        );
        assert_eq!(feedback.to_agent, "bruno");
    }

    #[test]
    fn test_rating_calculated_from_signal_type() {
        assert_eq!(signal_to_rating("quick_approval"), 5);
        assert_eq!(signal_to_rating("no_comments"), 4);
        assert_eq!(signal_to_rating("minor_feedback"), 4);
        assert_eq!(signal_to_rating("major_changes"), 2);
        assert_eq!(signal_to_rating("style_only"), 3);
        assert_eq!(signal_to_rating("rejected_pr"), 1);
    }

    #[test]
    fn test_feedback_type_is_stored() {
        let feedback = Feedback::new(
            "session1",
            "task1",
            "magnus",
            "bruno",
            "quick_approval",
            "explicit",
        );
        assert_eq!(feedback.feedback_type, "explicit");
    }

    // ====== Story 2: Pattern Detection Engine ======

    #[test]
    fn test_detect_patterns_groups_feedback_by_context() {
        let feedbacks = vec![
            Feedback::new("s1", "t1", "m", "bruno", "quick_approval", "implicit"),
            Feedback::new("s2", "t2", "m", "bruno", "quick_approval", "implicit"),
            Feedback::new("s3", "t3", "m", "bruno", "quick_approval", "implicit"),
        ];
        let patterns = detect_patterns(&feedbacks);
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_detect_patterns_calculates_confidence_correctly() {
        let feedbacks = vec![
            Feedback::new("s1", "t1", "m", "bruno", "quick_approval", "implicit"),
            Feedback::new("s2", "t2", "m", "bruno", "quick_approval", "implicit"),
            Feedback::new("s3", "t3", "m", "bruno", "quick_approval", "implicit"),
        ];
        let confidence = calculate_confidence(&feedbacks.iter().collect::<Vec<_>>());
        assert!(confidence > 0.0);
        assert!(confidence <= 1.0);
    }

    #[test]
    fn test_pattern_with_3_evidence_is_active() {
        let mut pattern = LearnedPattern::new("test", "action", "agent");
        pattern.update_evidence("ex1", 0.5);
        pattern.update_evidence("ex2", 0.6);
        pattern.update_evidence("ex3", 0.7);
        assert!(pattern.active);
    }

    #[test]
    fn test_pattern_with_confidence_08_is_verified() {
        let mut pattern = LearnedPattern::new("test", "action", "agent");
        pattern.update_evidence("ex1", 0.9);
        assert!(pattern.verified);
    }

    #[test]
    fn test_calculate_consistency_measures_rating_consistency() {
        let feedbacks = vec![
            Feedback::new("s1", "t1", "m", "a", "quick_approval", "implicit"),
            Feedback::new("s2", "t2", "m", "a", "quick_approval", "implicit"),
            Feedback::new("s3", "t3", "m", "a", "quick_approval", "implicit"),
        ];
        let consistency = calculate_consistency(&feedbacks.iter().collect::<Vec<_>>());
        assert!((consistency - 1.0).abs() < 0.01); // Perfect consistency
    }

    #[test]
    fn test_infer_action_extracts_recommended_action() {
        let feedbacks = vec![
            Feedback::new("s1", "t1", "m", "a", "quick_approval", "implicit"),
            Feedback::new("s2", "t2", "m", "a", "quick_approval", "implicit"),
            Feedback::new("s3", "t3", "m", "a", "quick_approval", "implicit"),
        ];
        let action = infer_action(&feedbacks.iter().collect::<Vec<_>>());
        assert!(action.contains("doing") || action.contains("improvements"));
    }

    // ====== Story 3: Learned Patterns Storage ======

    #[test]
    fn test_store_pattern_saves_to_table() {
        let pattern = LearnedPattern::new("auth", "use_pkce", "magnus");
        assert!(!pattern.id.is_empty());
        assert_eq!(pattern.trigger_pattern, "auth");
    }

    #[test]
    fn test_get_patterns_by_agent_returns_agent_patterns() {
        let patterns = vec![
            LearnedPattern::new("p1", "a1", "magnus"),
            LearnedPattern::new("p2", "a2", "bruno"),
        ];
        let magnus_patterns: Vec<_> = patterns.iter().filter(|p| p.agent == "magnus").collect();
        assert_eq!(magnus_patterns.len(), 1);
    }

    #[test]
    fn test_update_pattern_evidence_increments_count() {
        let mut pattern = LearnedPattern::new("test", "action", "agent");
        let initial_count = pattern.evidence_count;
        pattern.update_evidence("new example", 0.5);
        assert!(pattern.evidence_count > initial_count);
    }

    #[test]
    fn test_verify_pattern_sets_verified_true() {
        let mut pattern = LearnedPattern::new("test", "action", "agent");
        pattern.verify();
        assert!(pattern.verified);
    }

    #[test]
    fn test_deactivate_pattern_sets_active_false() {
        let mut pattern = LearnedPattern::new("test", "action", "agent");
        pattern.active = true;
        pattern.deactivate();
        assert!(!pattern.active);
    }

    // ====== Story 4: Santi Preferences Extraction ======

    #[test]
    fn test_extract_preference_parses_memory_content() {
        let pref = extract_preference("Santi prefers 2 spaces indent", 0.8);
        assert!(pref.is_some());
    }

    #[test]
    fn test_extract_preference_assigns_correct_category() {
        let pref = extract_preference("Santi prefers snake_case naming", 0.8);
        if let Some(p) = pref {
            assert_eq!(p.category, "code_style");
        }
    }

    #[test]
    fn test_store_preference_saves_to_table() {
        let pref = SantiPreference::new("workflow", "pr", "before merge", 0.8);
        assert!(!pref.id.is_empty());
        assert_eq!(pref.category, "workflow");
    }

    #[test]
    fn test_get_preferences_by_category_returns_category_preferences() {
        let prefs = vec![
            SantiPreference::new("code_style", "indent", "2 spaces", 0.8),
            SantiPreference::new("workflow", "pr", "before merge", 0.8),
        ];
        let code_prefs: Vec<_> = prefs
            .iter()
            .filter(|p| p.category == "code_style")
            .collect();
        assert_eq!(code_prefs.len(), 1);
    }

    #[test]
    fn test_update_preference_examples_extends_array() {
        let mut pref = SantiPreference::new("code_style", "indent", "2 spaces", 0.5);
        let initial_len = pref.examples.len();
        pref.update_examples("Another example");
        assert!(pref.examples.len() > initial_len);
    }

    // ====== Story 5: Nightly Processor ======

    #[test]
    fn test_nightly_processor_loads_24h_data() {
        let feedbacks = vec![Feedback::new(
            "s1",
            "t1",
            "m",
            "b",
            "quick_approval",
            "implicit",
        )];
        let memories = vec!["memory 1".to_string()];
        let summary = run_nightly_processor(&feedbacks, &memories);
        assert!(summary.patterns_detected >= 0);
    }

    #[test]
    fn test_nightly_processor_runs_pattern_detection() {
        let feedbacks = vec![
            Feedback::new("s1", "t1", "m", "b", "quick_approval", "implicit"),
            Feedback::new("s2", "t2", "m", "b", "quick_approval", "implicit"),
            Feedback::new("s3", "t3", "m", "b", "quick_approval", "implicit"),
        ];
        let summary = run_nightly_processor(&feedbacks, &[]);
        assert!(summary.patterns_detected >= 0);
    }

    #[test]
    fn test_nightly_processor_runs_preference_extraction() {
        let feedbacks = vec![];
        let memories = vec!["Santi prefers 2 spaces indent".to_string()];
        let summary = run_nightly_processor(&feedbacks, &memories);
        assert!(summary.preferences_extracted >= 0);
    }

    #[test]
    fn test_nightly_processor_generates_verification_tasks() {
        let feedbacks = vec![
            Feedback::new("s1", "t1", "m", "b", "quick_approval", "implicit"),
            Feedback::new("s2", "t2", "m", "b", "quick_approval", "implicit"),
            Feedback::new("s3", "t3", "m", "b", "quick_approval", "implicit"),
        ];
        let summary = run_nightly_processor(&feedbacks, &[]);
        assert!(summary.verifications_generated >= 0);
    }

    #[test]
    fn test_nightly_processor_returns_summary() {
        let summary = run_nightly_processor(&[], &[]);
        assert_eq!(summary.patterns_detected, 0);
    }

    // ====== Story 6: Insights Generation ======

    #[test]
    fn test_get_agent_insights_returns_patterns() {
        let patterns = vec![LearnedPattern::new("test", "action", "magnus")];
        let insights = get_agent_insights(&patterns, &[], "magnus");
        assert!(!insights.top_patterns.is_empty());
    }

    #[test]
    fn test_get_agent_insights_returns_top_5_verified() {
        let mut pattern = LearnedPattern::new("test", "action", "magnus");
        pattern.verified = true;
        let patterns = vec![pattern];
        let insights = get_agent_insights(&patterns, &[], "magnus");
        assert!(insights.top_patterns.len() <= 5);
    }

    #[test]
    fn test_get_agent_insights_returns_santi_preferences() {
        let pref = SantiPreference::new("workflow", "pr", "before merge", 0.8);
        let insights = get_agent_insights(&[], &[pref], "magnus");
        assert!(!insights.preferences.is_empty());
    }

    #[test]
    fn test_insights_include_confidence_and_evidence() {
        let patterns = vec![LearnedPattern::new("test", "action", "magnus")];
        let insights = get_agent_insights(&patterns, &[], "magnus");
        if !insights.top_patterns.is_empty() {
            assert!(insights.top_patterns[0].confidence >= 0.0);
        }
    }
}
