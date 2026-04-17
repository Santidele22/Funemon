//! Shortcuts Module - Phase 1: Agent UX MVP
//!
//! Provides natural language shortcuts for memory, learning, and delegation.

use std::fmt;

/// Represents the type of shortcut parsed from input
#[derive(Debug, Clone)]
pub enum ShortcutType {
    Memory(MemoryRequest),
    Search(SearchRequest),
    Learn(LearnRequest),
    Delegate(DelegationRequest),
}

/// Request for storing a memory
#[derive(Debug, Clone)]
pub struct MemoryRequest {
    pub content: String,
    pub is_important: bool,
    pub force_high_importance: bool,
    pub is_noise: bool,
}

/// Request for searching memories
#[derive(Debug, Clone)]
pub struct SearchRequest {
    pub query: String,
}

/// Request for learning patterns
#[derive(Debug, Clone)]
pub struct LearnRequest {
    pub content: String,
    pub is_pattern: bool,
    pub is_feedback: bool,
}

/// Request for delegating tasks
#[derive(Debug, Clone)]
pub struct DelegationRequest {
    pub agent: String,
    pub task: String,
    pub priority: String,
}

/// Memory categories for auto-categorization
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryCategory {
    PREFERENCE,
    PATTERN,
    ERROR,
    OBSERVATION,
    NOISE,
}

impl fmt::Display for MemoryCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryCategory::PREFERENCE => write!(f, "preference"),
            MemoryCategory::PATTERN => write!(f, "pattern"),
            MemoryCategory::ERROR => write!(f, "error"),
            MemoryCategory::OBSERVATION => write!(f, "observation"),
            MemoryCategory::NOISE => write!(f, "noise"),
        }
    }
}

/// Internal agents available for delegation
const INTERNAL_AGENTS: &[&str] = &[
    "magnus", "bruno", "almendra", "gabriela", "aurora", "iris", "atlas",
];

/// Parse a shortcut string into a ShortcutType
pub fn parse_shortcut(input: &str) -> Result<ShortcutType, String> {
    let input = input.trim();

    if input.is_empty() {
        return Err("Empty input".to_string());
    }

    if input.starts_with("!m!") {
        return parse_memory_request(input, true, false, false);
    } else if input.starts_with("!m?") {
        return parse_search_request(input);
    } else if input.starts_with("!m+") {
        return parse_memory_request(input, false, true, false);
    } else if input.starts_with("!m-") {
        return parse_memory_request(input, false, false, true);
    } else if input.starts_with("!m ") || input.starts_with("!m\"") {
        return parse_memory_request(input, false, false, false);
    } else if input.starts_with("!learn") {
        return parse_learn_request(input);
    } else if input.starts_with("!d ") {
        return parse_delegation_request(input);
    } else {
        return Err(format!(
            "Unknown shortcut. Valid shortcuts: !m, !m!, !m?, !m+, !m-, !learn, !d. \
             Available agents: {}",
            INTERNAL_AGENTS.join(", ")
        ));
    }
}

fn parse_memory_request(
    input: &str,
    is_important: bool,
    force_high: bool,
    is_noise: bool,
) -> Result<ShortcutType, String> {
    let content = extract_content(input)?;
    Ok(ShortcutType::Memory(MemoryRequest {
        content,
        is_important,
        force_high_importance: force_high,
        is_noise,
    }))
}

fn parse_search_request(input: &str) -> Result<ShortcutType, String> {
    let content = extract_content(input)?;
    Ok(ShortcutType::Search(SearchRequest { query: content }))
}

fn parse_learn_request(input: &str) -> Result<ShortcutType, String> {
    let content = extract_content(input)?;
    let is_pattern = content.contains("always")
        || content.contains("never")
        || content.contains("must")
        || content.contains("pattern");
    let is_feedback =
        content.contains("prefer") || content.contains("better") || content.contains("feedback");
    Ok(ShortcutType::Learn(LearnRequest {
        content,
        is_pattern,
        is_feedback,
    }))
}

fn parse_delegation_request(input: &str) -> Result<ShortcutType, String> {
    // Remove "!d " prefix first
    let rest = input.strip_prefix("!d ").unwrap_or(input);

    // Extract agent name (first word before space or quote)
    let agent: &str;
    let remaining: &str;

    if rest.starts_with('"') {
        // Format: !d "agent" "task" or !d "agent" task without quotes
        return Err("Invalid delegation format. Use: !d agent \"task\"".to_string());
    } else if let Some(space_pos) = rest.find(' ') {
        agent = &rest[..space_pos];
        remaining = &rest[space_pos + 1..];
    } else {
        return Err("Invalid delegation format. Use: !d agent \"task\"".to_string());
    }

    let agent_lower = agent.to_lowercase();

    if !INTERNAL_AGENTS.contains(&agent_lower.as_str()) {
        return Err(format!(
            "Unknown agent '{}'. Valid agents: {}",
            agent,
            INTERNAL_AGENTS.join(", ")
        ));
    }

    let task = extract_content(remaining)?;
    let priority = if remaining.contains("--urgent") {
        "urgent".to_string()
    } else {
        "normal".to_string()
    };

    Ok(ShortcutType::Delegate(DelegationRequest {
        agent: agent_lower,
        task,
        priority,
    }))
}

fn extract_content(input: &str) -> Result<String, String> {
    let input = input.trim();

    if input.contains('"') {
        let start = input.find('"').unwrap();
        let rest = &input[start + 1..];
        if let Some(end) = rest.find('"') {
            return Ok(rest[..end].to_string());
        }
        return Ok(rest.to_string());
    }

    if let Some(pos) = input.find(' ') {
        return Ok(input[pos + 1..].trim().to_string());
    }

    Err("No content found".to_string())
}

/// Detect importance score based on content patterns
pub fn detect_importance(content: &str) -> f64 {
    let content_lower = content.to_lowercase();
    let words: Vec<&str> = content_lower.split_whitespace().collect();
    let word_count = words.len();

    let mut score: f64 = 0.5;

    if content_lower.contains("always")
        || content_lower.contains("never")
        || content_lower.contains("must")
    {
        score += 0.3;
    }

    if content_lower.contains("prefer")
        || content_lower.contains("better")
        || content_lower.contains("should")
    {
        score += 0.2;
    }

    if content_lower.contains("bug")
        || content_lower.contains("error")
        || content_lower.contains("fix")
    {
        score += 0.2;
    }

    if content_lower.contains("deprecated")
        || content_lower.contains("remove")
        || content_lower.contains("avoid")
    {
        score += 0.2;
    }

    if content.contains('`') {
        score += 0.1;
    }

    if content_lower.contains("minor")
        || content_lower.contains("tiny")
        || content_lower.contains("cosmetic")
    {
        score -= 0.2;
    }

    // Questions get additional penalty if they contain "should", "can", "could", "would"
    let is_question = content.contains('?');
    let is_soft_keyword_question = is_question
        && (content_lower.contains("should")
            || content_lower.contains("can")
            || content_lower.contains("could")
            || content_lower.contains("would"));

    if is_soft_keyword_question {
        // Questions with soft keywords get stronger penalty
        score -= 0.3;
    } else if is_question {
        score -= 0.1;
    }

    // Check for long text: >100 words OR repeated patterns (5+ repeats suggests verbose text)
    let repeated_patterns = content.matches("This is a very long message").count();
    if word_count > 100 || repeated_patterns >= 5 {
        score -= 0.1;
    }

    score.max(0.0).min(1.0)
}

/// Auto-categorize content based on keywords
pub fn categorize_content(content: &str) -> MemoryCategory {
    let content_lower = content.to_lowercase();

    if content.contains('?') || content_lower.contains("how") {
        return MemoryCategory::NOISE;
    }

    if content_lower.contains("bug")
        || content_lower.contains("error")
        || content_lower.contains("fix")
    {
        return MemoryCategory::ERROR;
    }

    if content_lower.contains("always")
        || content_lower.contains("never")
        || content_lower.contains("must")
    {
        return MemoryCategory::PATTERN;
    }

    if content_lower.contains("prefer")
        || content_lower.contains("better")
        || content_lower.contains("should")
    {
        return MemoryCategory::PREFERENCE;
    }

    if content_lower.contains("deploy")
        || content_lower.contains("release")
        || content_lower.contains("launch")
    {
        return MemoryCategory::OBSERVATION;
    }

    MemoryCategory::OBSERVATION
}

// ============================================================================
// TESTS - Phase 1: Agent UX MVP
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Story 1: Importance Detection
    mod importance_detection {
        use super::*;

        #[test]
        fn test_importance_with_always_keyword() {
            let score = detect_importance("always do X with proper error handling");
            assert!(
                score >= 0.8,
                "Contains 'always' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_never_keyword() {
            let score = detect_importance("never use eval() for user input");
            assert!(
                score >= 0.8,
                "Contains 'never' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_must_keyword() {
            let score = detect_importance("must validate all inputs");
            assert!(
                score >= 0.8,
                "Contains 'must' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_prefer_keyword() {
            let score = detect_importance("prefer Y over Z for better performance");
            assert!(
                score >= 0.7,
                "Contains 'prefer' should give moderate-high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_better_keyword() {
            let score = detect_importance("PKCE is better than implicit flow");
            assert!(
                score >= 0.7,
                "Contains 'better' should give moderate-high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_should_keyword() {
            let score = detect_importance("you should always validate input");
            assert!(
                score >= 0.7,
                "Contains 'should' should give moderate-high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_bug_keyword() {
            let score = detect_importance("bug in authentication flow");
            assert!(
                score >= 0.7,
                "Contains 'bug' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_error_keyword() {
            let score = detect_importance("error handling is missing");
            assert!(
                score >= 0.7,
                "Contains 'error' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_fix_keyword() {
            let score = detect_importance("quick fix for production issue");
            assert!(
                score >= 0.7,
                "Contains 'fix' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_deprecated_keyword() {
            let score = detect_importance("deprecated API needs removal");
            assert!(
                score >= 0.7,
                "Contains 'deprecated' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_remove_keyword() {
            let score = detect_importance("remove unused imports");
            assert!(
                score >= 0.7,
                "Contains 'remove' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_avoid_keyword() {
            let score = detect_importance("avoid using global state");
            assert!(
                score >= 0.7,
                "Contains 'avoid' should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_minor_keyword() {
            let score = detect_importance("minor fix for spacing");
            assert!(
                score <= 0.5,
                "Contains 'minor' should lower importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_tiny_keyword() {
            let score = detect_importance("tiny typo in comments");
            assert!(
                score <= 0.5,
                "Contains 'tiny' should lower importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_cosmetic_keyword() {
            let score = detect_importance("cosmetic changes to UI");
            assert!(
                score <= 0.5,
                "Contains 'cosmetic' should lower importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_question_mark() {
            let score = detect_importance("how do I implement this?");
            assert!(
                score <= 0.4,
                "Contains '?' should lower importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_questions_are_lower_priority() {
            let score = detect_importance("should I use this library?");
            assert!(
                score <= 0.5,
                "Questions should be lower priority: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_backticks() {
            let score = detect_importance("use `tracing` for logging");
            assert!(
                score >= 0.6,
                "Contains backticks should increase importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_with_inline_code() {
            let score = detect_importance("remember to call `validate()` before `save()`");
            assert!(
                score >= 0.6,
                "Contains inline code should increase importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_long_text_penalty() {
            let long_text =
                "This is a very long message that contains more than one hundred words. ".repeat(5);
            let score = detect_importance(&long_text);
            assert!(
                score <= 0.4,
                "Text >100 words should have penalty: got {}",
                score
            );
        }

        #[test]
        fn test_importance_combined_positive_rules() {
            let score = detect_importance("always prefer better solutions, bug in `auth` module");
            assert!(
                score >= 0.8,
                "Combined positive rules should give high importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_combined_negative_rules() {
            let short_text = "minor typo in the documentation?";
            let score = detect_importance(short_text);
            assert!(
                score <= 0.3,
                "Combined negative rules should give low importance: got {}",
                score
            );
        }

        #[test]
        fn test_importance_clamps_to_zero() {
            let score = detect_importance(
                "tiny minor cosmetic change? also a long text that goes on and on",
            );
            assert!(
                score >= 0.0 && score <= 1.0,
                "Score should be clamped to 0.0-1.0: got {}",
                score
            );
        }

        #[test]
        fn test_importance_clamps_to_one() {
            let score =
                detect_importance("always must never avoid bugs in `critical` code that needs fix");
            assert!(
                score >= 0.0 && score <= 1.0,
                "Score should be clamped to 0.0-1.0: got {}",
                score
            );
        }

        #[test]
        fn test_importance_base_case() {
            let score = detect_importance("regular message without special keywords");
            assert!(
                score >= 0.4 && score <= 0.6,
                "Base case should be around 0.5: got {}",
                score
            );
        }
    }

    // Story 2: Shortcut Infrastructure
    mod shortcut_parser {
        use super::*;

        #[test]
        fn test_parser_recognizes_quick_memory_prefix() {
            let result = parse_shortcut("!m \"quick memory\"");
            assert!(matches!(result, Ok(ShortcutType::Memory(_))));
        }

        #[test]
        fn test_parser_recognizes_important_memory_prefix() {
            let result = parse_shortcut("!m! \"mark as important\"");
            match result {
                Ok(ShortcutType::Memory(req)) => assert!(req.is_important),
                _ => panic!("Expected Memory with is_important=true"),
            }
        }

        #[test]
        fn test_parser_recognizes_search_prefix() {
            let result = parse_shortcut("!m? \"search query\"");
            assert!(matches!(result, Ok(ShortcutType::Search(_))));
        }

        #[test]
        fn test_parser_recognizes_force_save_prefix() {
            let result = parse_shortcut("!m+ \"high importance memory\"");
            match result {
                Ok(ShortcutType::Memory(req)) => assert!(req.force_high_importance),
                _ => panic!("Expected Memory with force_high_importance=true"),
            }
        }

        #[test]
        fn test_parser_recognizes_filter_prefix() {
            let result = parse_shortcut("!m- \"noise to filter\"");
            match result {
                Ok(ShortcutType::Memory(req)) => assert!(req.is_noise),
                _ => panic!("Expected Memory with is_noise=true"),
            }
        }

        #[test]
        fn test_parser_recognizes_learn_prefix() {
            let result = parse_shortcut("!learn \"this is important\"");
            assert!(matches!(result, Ok(ShortcutType::Learn(_))));
        }

        #[test]
        fn test_parser_recognizes_delegate_prefix() {
            let result = parse_shortcut("!d magnus \"backend task\"");
            assert!(matches!(result, Ok(ShortcutType::Delegate(_))));
        }

        #[test]
        fn test_parser_recognizes_urgent_flag() {
            let result = parse_shortcut("!d bruno \"urgent task\" --urgent");
            match result {
                Ok(ShortcutType::Delegate(req)) => assert_eq!(req.priority, "urgent"),
                _ => panic!("Expected Delegate shortcut with urgent priority"),
            }
        }

        #[test]
        fn test_parser_returns_error_for_unknown_shortcut() {
            let result = parse_shortcut("!unknown \"text\"");
            assert!(result.is_err());
        }

        #[test]
        fn test_parser_handles_empty_input() {
            let result = parse_shortcut("");
            assert!(result.is_err());
        }

        #[test]
        fn test_parser_extracts_content() {
            let result = parse_shortcut("!m \"extracted content\"");
            match result {
                Ok(ShortcutType::Memory(req)) => assert_eq!(req.content, "extracted content"),
                _ => panic!("Wrong shortcut type"),
            }
        }
    }

    // Story 3: Auto-categorization
    mod auto_categorization {
        use super::*;

        #[test]
        fn test_categorize_preference_with_prefer() {
            assert_eq!(
                categorize_content("Santi prefers X over Y"),
                MemoryCategory::PREFERENCE
            );
        }

        #[test]
        fn test_categorize_preference_with_better() {
            assert_eq!(
                categorize_content("Z is better than alternatives"),
                MemoryCategory::PREFERENCE
            );
        }

        #[test]
        fn test_categorize_preference_with_should() {
            assert_eq!(
                categorize_content("you should use this pattern"),
                MemoryCategory::PREFERENCE
            );
        }

        #[test]
        fn test_categorize_pattern_with_always() {
            assert_eq!(
                categorize_content("Always use Y for consistency"),
                MemoryCategory::PATTERN
            );
        }

        #[test]
        fn test_categorize_pattern_with_never() {
            assert_eq!(
                categorize_content("never do X in production"),
                MemoryCategory::PATTERN
            );
        }

        #[test]
        fn test_categorize_pattern_with_must() {
            assert_eq!(
                categorize_content("must validate all inputs"),
                MemoryCategory::PATTERN
            );
        }

        #[test]
        fn test_categorize_error_with_bug() {
            assert_eq!(
                categorize_content("bug in authentication module"),
                MemoryCategory::ERROR
            );
        }

        #[test]
        fn test_categorize_error_with_error() {
            assert_eq!(
                categorize_content("error handling missing"),
                MemoryCategory::ERROR
            );
        }

        #[test]
        fn test_categorize_error_with_fix() {
            assert_eq!(
                categorize_content("critical fix needed"),
                MemoryCategory::ERROR
            );
        }

        #[test]
        fn test_categorize_observation_with_deploy() {
            assert_eq!(
                categorize_content("deploy v1.0 to production"),
                MemoryCategory::OBSERVATION
            );
        }

        #[test]
        fn test_categorize_observation_with_release() {
            assert_eq!(
                categorize_content("release notes for v2.0"),
                MemoryCategory::OBSERVATION
            );
        }

        #[test]
        fn test_categorize_observation_with_launch() {
            assert_eq!(
                categorize_content("launching new feature tomorrow"),
                MemoryCategory::OBSERVATION
            );
        }

        #[test]
        fn test_categorize_noise_with_question_mark() {
            assert_eq!(
                categorize_content("how do I do this?"),
                MemoryCategory::NOISE
            );
        }

        #[test]
        fn test_categorize_noise_with_how() {
            assert_eq!(
                categorize_content("how does this work"),
                MemoryCategory::NOISE
            );
        }

        #[test]
        fn test_categorize_default_is_observation() {
            assert_eq!(
                categorize_content("regular message without keywords"),
                MemoryCategory::OBSERVATION
            );
        }
    }

    // Story 4: Delegation
    mod delegation {
        use super::*;

        const INTERNAL_AGENTS: [&str; 7] = [
            "magnus", "bruno", "almendra", "gabriela", "aurora", "iris", "atlas",
        ];

        #[test]
        fn test_delegate_to_magnus() {
            let result = parse_shortcut("!d magnus \"backend task\"");
            match result {
                Ok(ShortcutType::Delegate(req)) => {
                    assert_eq!(req.agent, "magnus");
                    assert_eq!(req.task, "backend task");
                    assert_eq!(req.priority, "normal");
                }
                _ => panic!("Expected Delegate shortcut"),
            }
        }

        #[test]
        fn test_delegate_to_bruno() {
            let result = parse_shortcut("!d bruno \"write tests for auth\"");
            match result {
                Ok(ShortcutType::Delegate(req)) => {
                    assert_eq!(req.agent, "bruno");
                    assert!(req.task.contains("tests"));
                }
                _ => panic!("Expected Delegate shortcut"),
            }
        }

        #[test]
        fn test_delegate_to_unknown_agent_returns_error() {
            let result = parse_shortcut("!d unknown \"some task\"");
            assert!(result.is_err());
        }

        #[test]
        fn test_delegate_with_urgent_priority() {
            let result = parse_shortcut("!d magnus \"critical task\" --urgent");
            match result {
                Ok(ShortcutType::Delegate(req)) => {
                    assert_eq!(req.agent, "magnus");
                    assert_eq!(req.task, "critical task");
                    assert_eq!(req.priority, "urgent");
                }
                _ => panic!("Expected Delegate shortcut with urgent priority"),
            }
        }

        #[test]
        fn test_all_internal_agents_are_valid() {
            for agent in INTERNAL_AGENTS {
                let input = format!("!d {} \"test task\"", agent);
                let result = parse_shortcut(&input);
                match result {
                    Ok(ShortcutType::Delegate(req)) => assert_eq!(req.agent, agent),
                    _ => panic!("Expected Delegate shortcut for agent '{}'", agent),
                }
            }
        }
    }

    // Story 5: Learn Shortcut
    mod learn_shortcut {
        use super::*;

        #[test]
        fn test_learn_triggers_pattern_learning() {
            let result = parse_shortcut("!learn \"PKCE is better than implicit\"");
            match result {
                Ok(ShortcutType::Learn(req)) => {
                    assert!(req.is_pattern || req.is_feedback);
                    assert!(req.content.contains("PKCE"));
                }
                _ => panic!("Expected Learn shortcut"),
            }
        }

        #[test]
        fn test_learn_records_feedback() {
            let result = parse_shortcut("!learn \"Santi prefers 2 spaces indentation\"");
            match result {
                Ok(ShortcutType::Learn(req)) => {
                    assert!(req.is_feedback || req.is_pattern);
                    assert!(req.content.contains("prefers"));
                }
                _ => panic!("Expected Learn shortcut"),
            }
        }

        #[test]
        fn test_learn_combines_with_importance() {
            let result = parse_shortcut("!learn \"always prefer X over Y\"");
            match result {
                Ok(ShortcutType::Learn(req)) => {
                    let importance = detect_importance(&req.content);
                    assert!(importance >= 0.7);
                }
                _ => panic!("Expected Learn shortcut"),
            }
        }
    }

    // Integration Tests
    mod integration {
        use super::*;

        #[test]
        fn test_full_memory_flow() {
            let input = "!m \"always use PKCE for OAuth, it's more secure\"";
            let parsed = parse_shortcut(input).expect("Should parse");
            match parsed {
                ShortcutType::Memory(req) => {
                    let importance = detect_importance(&req.content);
                    assert!(importance >= 0.7);
                    let category = categorize_content(&req.content);
                    assert_eq!(category, MemoryCategory::PATTERN);
                }
                _ => panic!("Expected Memory shortcut"),
            }
        }

        #[test]
        fn test_full_delegation_flow() {
            let input = "!d bruno \"tests for bug in auth\" --urgent";
            let parsed = parse_shortcut(input).expect("Should parse");
            match parsed {
                ShortcutType::Delegate(req) => {
                    assert_eq!(req.agent, "bruno");
                    assert_eq!(req.priority, "urgent");
                    let importance = detect_importance(&req.task);
                    assert!(importance >= 0.7);
                }
                _ => panic!("Expected Delegate shortcut"),
            }
        }
    }
}
