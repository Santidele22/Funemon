# Funemon Core Implementation SPEC

**Version:** 1.0
**Date:** 2026-04-15
**Author:** Almendra (Documentation)
**Status:** Draft - Ready for Review

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Architecture Overview](#architecture-overview)
3. [Database Schema](#database-schema)
4. [Phase 1: Agent UX MVP](#phase-1-agent-ux-mvp)
   - [AI-Powered Importance Detection](#12-automatic-importance-detection)
5. [Phase 2: Team Memory](#phase-2-team-memory)
6. [Phase 3: Autonomous Teams](#phase-3-autonomous-teams)
   - [Team Lead: Priority + Hitos](#35-team-lead-priority--hitos)
7. [Phase 4: Feedback Loop](#phase-4-feedback-loop)
   - [Implicit Feedback - Complete Signal List](#41-implicit-feedback--complete-signal-list)
   - [Verification Loop](#42-verification-loop)
8. [Phase 5: Learning System](#phase-5-learning-system)
9. [Phase 6: Dashboard](#phase-6-dashboard)
10. [Phase 7: Skills Auto-Update](#phase-7-skills-auto-update)
11. [Shortcuts Reference](#shortcuts-reference)
12. [Team Structure](#team-structure)
13. [Dependencies & Roadmap](#dependencies--roadmap)
14. [ROI Analysis](#roi-analysis)
15. [Appendix A: Example Flows](#appendix-a-example-flows)
16. [Appendix B: Testing Requirements](#appendix-b-testing-requirements)

---

## Executive Summary

This SPEC defines the **complete** implementation roadmap for the Funemon ecosystem. The implementation is divided into 7 phases spanning approximately 56-74 hours of development work.

### Goals

- Enable natural language shortcuts for memory, learning, and delegation
- Establish team-based memory with shared context
- Implement autonomous team workflows with permission checkpoints
- Create implicit and explicit feedback loops for continuous improvement
- Build a learning system that detects patterns and preferences
- Create a transparency dashboard for Santi's review
- Enable automatic skills updates based on learned patterns

### Scope

```
funemon (Rust CLI + SQLite)
├── funemon-system/      # Core CLI
├── funemon-agents/      # Tyrion, Magnus, Aurora, Bruno, etc.
└── funemon-ecosystem/   # This SPEC, docs, specs

Timeline:
├── Phase 1-4 (CORE): 21-27h - Foundation
├── Phase 5-7 (FULL): 35-47h - Learning & Intelligence
└── TOTAL: 56-74h - Complete System
```

### Scope

```
funemon (Rust CLI + SQLite)
├── funemon-system/      # Core CLI
├── funemon-agents/      # Tyrion, Magnus, Aurora, Bruno, etc.
└── funemon-ecosystem/   # This SPEC, docs, specs
```

---

## Architecture Overview

### Component Stack

```
┌─────────────────────────────────────────────────────────┐
│                    User (Santi)                          │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│               Tyrion (Orchestrator)                      │
│  - Task routing                                         │
│  - Permission checkpoints                               │
│  - Result aggregation                                   │
└─────────────────────────────────────────────────────────┘
           │              │              │
           ▼              ▼              ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Magnus Team  │ │ Aurora Team  │ │ ATLAS Team   │
│ Magnus       │ │ Aurora       │ │ ATLAS        │
│ Bruno        │ │ Bruno        │ │ (internal)   │
│ Almendra     │ │ Iris         │ └──────────────┘
│ Gabriela     │ │ Almendra     │
└──────────────┘ └──────────────┘
           │              │
           ▼              ▼
┌─────────────────────────────────────────────────────────┐
│              funemon-core (Rust CLI)                    │
│  - Memory storage (SQLite)                              │
│  - Shortcuts parsing                                    │
│  - Agent communication                                  │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

```
User Input → Shortcut Parser → Router → Agent/Team
     │                                            │
     ▼                                            ▼
Memory Store ←────────────── Feedback Loop ← Result
```

---

## Database Schema

### Existing Tables (assumed)

```sql
-- memories: existing memory storage
CREATE TABLE memories (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    content TEXT NOT NULL,
    type TEXT, -- 'observation', 'error', 'plan', 'preference'
    importance REAL DEFAULT 0.5,
    created_at INTEGER,
    metadata TEXT
);

-- sessions: session management
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    project TEXT,
    started_at INTEGER,
    last_active INTEGER
);
```

### New Tables Required

#### Table: `teams`

```sql
CREATE TABLE teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE, -- 'magnus', 'aurora', 'atlas'
    description TEXT,
    created_at INTEGER,
    updated_at INTEGER
);
```

#### Table: `team_members`

```sql
CREATE TABLE team_members (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL REFERENCES teams(id),
    agent_name TEXT NOT NULL,
    role TEXT DEFAULT 'member', -- 'lead', 'member'
    joined_at INTEGER,
    UNIQUE(team_id, agent_name)
);
```

#### Table: `team_memories`

```sql
CREATE TABLE team_memories (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL REFERENCES teams(id),
    content TEXT NOT NULL,
    category TEXT, -- 'pattern', 'preference', 'context'
    created_by TEXT,
    created_at INTEGER,
    importance REAL DEFAULT 0.5
);
```

#### Table: `permissions`

```sql
CREATE TABLE permissions (
    id TEXT PRIMARY KEY,
    from_agent TEXT NOT NULL,
    to_agent TEXT NOT NULL,
    team_id TEXT REFERENCES teams(id),
    granted_at INTEGER,
    expires_at INTEGER,
    scope TEXT -- 'team', 'project', 'temporary'
);
```

#### Table: `feedback`

```sql
CREATE TABLE feedback (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    task_id TEXT,
    from_agent TEXT,
    to_agent TEXT,
    rating INTEGER, -- 1-5
    feedback_type TEXT, -- 'implicit', 'explicit'
    comments TEXT,
    created_at INTEGER,
    metadata TEXT
);
```

#### Table: `delegations`

```sql
CREATE TABLE delegations (
    id TEXT PRIMARY KEY,
    from_agent TEXT NOT NULL,
    to_agent TEXT NOT NULL,
    task_description TEXT NOT NULL,
    priority TEXT DEFAULT 'normal', -- 'normal', 'urgent', 'low'
    status TEXT DEFAULT 'pending', -- 'pending', 'in_progress', 'completed', 'cancelled'
    created_at INTEGER,
    completed_at INTEGER,
    team_id TEXT REFERENCES teams(id)
);
```

---

## Phase 1: Agent UX MVP

**Duration:** 2-3 hours
**Priority:** HIGH
**Dependencies:** None

### Goals

- Implement natural language shortcuts (!m, !d, !learn)
- Automatic importance detection
- No database changes (use existing schema)

### Features

#### 1.1 Shortcut Parser

The system must recognize and parse these shortcuts:

| Shortcut | Description | Example |
|----------|-------------|---------|
| `!m "text"` | Quick memory (auto-categorize) | `!m "PKCE is better than implicit"` |
| `!m! "text"` | Mark as important (→ learned pattern) | `!m! "Santi prefers 2 spaces"` |
| `!m? "query"` | Quick search | `!m? "auth patterns"` |
| `!m+ "text"` | Force save (high importance) | `!m+ "critical security fix"` |
| `!m- "text"` | Noise filter (low importance) | `!m- "minor formatting"` |
| `!learn "text"` | Direct to learned patterns | `!learn "PKCE > implicit flow"` |
| `!d agent "task"` | Simple delegation | `!d bruno "tests auth"` |
| `!d agent "task" --urgent` | Delegation with priority | `!d bruno "tests" --urgent` |

#### 1.2 AI-Powered Importance Detection

**Rules:**

| Pattern | Importance Score |
|---------|-----------------|
| Contains "always", "never", "must" | +0.3 |
| Contains "prefer", "better", "should" | +0.2 |
| Contains "bug", "error", "fix" | +0.2 |
| Contains "deprecated", "remove", "avoid" | +0.2 |
| Contains "minor", "tiny", "cosmetic" | -0.2 |
| Contains question mark (?) | -0.1 |
| Contains code snippet (backticks) | +0.1 |
| More than 100 words | -0.1 |

**Implicit Learning Signals (learned over time):**

| Pattern | Score Adjustment |
|---------|-----------------|
| Santi confirms pattern | +0.3 |
| Pattern led to success | +0.2 |
| Error avoided | +0.2 |
| Repeated by agent | +0.1 |
| Ignored by Santi | -0.2 |

**Auto-Categorization:**

| Contains | Category |
|----------|----------|
| "prefer", "better", "should" | preference |
| "always", "never", "must" | pattern |
| "bug", "error", "fix" | error |
| "deploy", "release", "launch" | observation |
| "?","how","?" | noise (ignore) |

**Algorithm:**

```
1. Parse input for shortcut prefix
2. Extract content
3. Apply pattern matching for importance
4. Auto-categorize:
   - If contains "prefer", "better" → preference
   - If contains "pattern", "learned" → pattern
   - If contains error keywords → error
   - Otherwise → observation
5. Store with calculated importance
6. Track in learning queue for pattern detection
```
1. Parse input for shortcut prefix
2. Extract content
3. Apply pattern matching for importance
4. Auto-categorize:
   - If contains "prefer", "better" → preference
   - If contains "pattern", "learned" → pattern
   - If contains error keywords → error
   - Otherwise → observation
5. Store with calculated importance
```

#### 1.3 Simple Delegation

**Rules:**

- Direct delegation to internal agents: Allowed without permission
- Delegation to external agents (Iris): Requires confirmation

**Flow:**

```
User: "!d bruno tests auth"
  → Parse: to=bruno, task="tests auth", priority=normal
  → Check: Is bruno an internal agent? YES
  → Create task for Bruno
  → Store delegation record
  → Return: "Delegated to Bruno: tests auth"
```

---

## Phase 2: Team Memory

**Duration:** 6-8 hours
**Priority:** HIGH
**Dependencies:** Phase 1 completed

### Goals

- Define preconfigured teams (Magnus Team, Aurora Team, ATLAS Team)
- Implement team-shared memory
- Add team-specific shortcuts

### Team Definitions

#### Magnus Team

```
Scope: Backend + Tests + Docs + Security
Lead: Magnus
Members: Bruno, Almendra, Gabriela
Internal agents: Magnus, Bruno, Almendra, Gabriela
```

#### Aurora Team

```
Scope: Frontend + Design + Tests + Docs
Lead: Aurora
Members: Bruno, Iris, Almendra
Internal agents: Aurora, Bruno, Iris, Almendra
```

#### ATLAS Team

```
Scope: Planning + Backlog + User Stories
Lead: ATLAS
Members: (internal only - no external agents)
```

### Features

#### 2.1 Team Configuration

```yaml
teams:
  - name: magnus
    description: Backend development team
    lead: magnus
    members:
      - magnus (lead)
      - bruno
      - almendra
      - gabriela
    scopes:
      - backend
      - tests
      - docs
      - security

  - name: aurora
    description: Frontend development team
    lead: aurora
    members:
      - aurora (lead)
      - bruno
      - iris
      - almendra
    scopes:
      - frontend
      - design
      - tests
      - docs

  - name: atlas
    description: Planning and project management
    lead: atlas
    members:
      - atlas
    scopes:
      - planning
      - backlog
      - user-stories
```

#### 2.2 Team Memory Storage

**Schema:** `team_memories` table (see DB section)

**Operations:**

```rust
// Store team memory
fn store_team_memory(team_id: &str, content: &str, category: &str) {
    let importance = detect_importance(content);
    let created_by = current_agent();
    
    db::insert("team_memories", {
        team_id,
        content,
        category,
        importance,
        created_by,
        created_at: now()
    });
}

// Query team memory
fn query_team_memory(team_id: &str, query: &str) -> Vec<Memory> {
    db::search("team_memories", {
        team_id: team_id,
        search: query
    })
}

// Query all teams
fn query_all_teams(query: &str) -> Vec<Memory> {
    db::search("team_memories", { search: query })
}
```

#### 2.3 Team Shortcuts

| Shortcut | Description | Example |
|----------|-------------|---------|
| `!team "team" "message"` | Share with team | `!team "magnus" "use tracing for logging"` |
| `!team? "query"` | Search all teams | `!team? "auth patterns"` |
| `!team magnus "msg"` | Direct team message | `!team magnus "new API endpoint"` |

#### 2.4 Cross-Team Sharing

**Rules:**
- Team memories are visible to all team members
- Team Lead can mark memories as "visible to other teams"
- Cross-team search via `!team?` searches all team memories

---

## Phase 3: Autonomous Teams

**Duration:** 5-6 hours
**Priority:** MEDIUM
**Dependencies:** Phase 2 completed

### Goals

- Define Team Lead roles
- Implement sub-delegation rules
- Establish "result only" communication with Tyrion
- Add permission checkpoints

### Features

#### 3.1 Team Lead Role

**Responsibilities:**
- Receive tasks from Tyrion
- Delegate to team members or execute personally
- Report final results to Tyrion
- Request cross-team permissions when needed

**Authority:**
- Can delegate to any team member without permission
- Can execute tasks personally
- Cannot delegate outside team without permission checkpoint

#### 3.2 Permission Checkpoints

**Rule:** When Team Lead needs an agent NOT in their team → MUST request permission from Tyrion.

```
Scenario: Magnus (Magnus Team) needs Iris (Aurora Team)

Magnus: "!d iris design login"
  → System detects: Iris is NOT in Magnus Team
  → Permission Check: Request to Tyrion
  → Tyrion evaluates: Is this necessary?
  → If approved: Delegate with temporary permission
  → If denied: "Permission denied. Suggest: Aurora can help instead"
```

**Permission Table:**

| From | To | Auto-Approved? |
|------|-----|----------------|
| Magnus Team | Magnus, Bruno, Almendra, Gabriela | YES |
| Magnus Team | Aurora, Iris | NO (checkpoint) |
| Aurora Team | Aurora, Bruno, Iris, Almendra | YES |
| Aurora Team | Magnus, Gabriela | NO (checkpoint) |
| ATLAS Team | (any external) | NO (ATLAS is internal only) |

#### 3.3 Sub-Delegation Rules

**Within Team:** NO permission needed
**Outside Team:** PERMISSION REQUIRED

```
Within Team:
  Magnus receives task → can delegate to Bruno directly → NO checkpoint

Outside Team:
  Magnus needs Iris → must ask Tyrion → Tyrion approves/denies
```

#### 3.4 Result-Only Communication

**Tyrion receives:**
- ✅ "Feature X completed, PR ready"
- ✅ "Blocking error, need decision"
- ✅ "Feature X cancelled: reason"

**Tyrion does NOT receive:**
- ❌ "Should I write tests now?"
- ❌ "Need docs?"
- ❌ "Progress update?"
- ❌ "Is this correct?"

**Implementation:**

```rust
fn handle_agent_result(agent: &str, result: Result) {
    match result {
        Ok(final_output) => {
            // Only accept final results
            if is_final_result(result) {
                update_delegation_status(agent, "completed");
                notify_tyrion(agent, final_output);
            } else {
                reject(agent, "Only final results accepted");
            }
        }
        Err(blocking_error) => {
            // Blocking errors are allowed
            notify_tyrion_error(agent, blocking_error);
        }
    }
}

fn is_final_result(result: &Result) -> bool {
    // Check if result is marked as final
    result.has_flag("final") 
        || result.type == "completed"
        || result.type == "cancelled"
}
```

#### 3.5 Implementation Details

**State Machine for Delegations:**

```
┌─────────┐     accept     ┌──────────┐
│ pending │ ──────────────→│ in_progress │
└─────────┘                └────────────┘
      │                          │
      │                   complete│
      │                          ▼
      │                   ┌──────────┐
      └───────────────────│ completed│
                           └──────────┘
       cancel
            ▼
       ┌──────────┐
       │ cancelled│
       └──────────┘
```

#### 3.6 Team Lead: Priority + Hitos

**Priority System:**

Team Leads use internal priority levels:

| Priority | Label | Meaning |
|----------|-------|---------|
| P0 | Critical | Blocking, finish now |
| P1 | High | This week |
| P2 | Medium | This sprint |
| P3 | Low | When there's time |

**Internal Milestones (Hitos):**

Team Leads track progress internally and do NOT report to Tyrion:

| Milestone | Description |
|-----------|-------------|
| "Feature X: Design done (50%)" | Design phase complete |
| "Feature X: Implementation done (75%)" | Code written |
| "Feature X: Tests done (90%)" | Tests passing |
| "Feature X: PR ready (100%)" | Ready for review |

**What Team Leads Report to Tyrion:**

✅ **ONLY Final Results:**
- "Feature X completed, PR ready for review"
- "Feature X cancelled: reason"
- "Blocking error: need decision"

❌ **NEVER Report to Tyrion:**
- Progress updates
- Internal milestones
- "Should I write tests now?"
- "Is this correct?"

**Implementation:**

```rust
struct TeamLeadTask {
    task: String,
    priority: Priority,  // P0, P1, P2, P3
    milestones: Vec<Milestone>,
    current_progress: u8,  // 0-100%
}

fn report_to_tyrion(task: &TeamLeadTask) {
    match task.current_progress {
        100 => notify_tyrion_final(task),  // Only at 100%
        _ => {}  // Silent progress (internal only)
    }
}
```

---

## Phase 4: Feedback Loop

**Duration:** 8-10 hours
**Priority:** MEDIUM
**Dependencies:** Phase 3 completed

### Goals

- Implement implicit feedback capture
- Add explicit feedback on ambiguity
- Build pattern recognition for improvement

### Features

#### 4.1 Implicit Feedback - Complete Signal List

**Detection Rules:**

| Signal | Feedback | Interpretation |
|--------|----------|----------------|
| PR approved in < 1 hour | 👍 positive | Good quality, minimal changes needed |
| PR changes requested 3+ times | 👎 negative | Something missing or incorrect |
| Approved without comments | 👍 positive | Meets expectations |
| Code style corrections | 👎 negative | (but learned) - improve quality |
| Quick acceptance with "nice" | 👍 positive | Exceeds expectations |
| Repeated similar corrections | 👎 negative | Pattern issue |

**Complete Signal List:**

| Signal | Condition | Rating | Learn |
|--------|-----------|--------|-------|
| Quick approval | PR merged < 1hr | 5 👍 | "Quality good" |
| No comments | PR approved silently | 4 👍 | "Met expectations" |
| Minor feedback | 1-2 small comments | 4 | "Almost perfect" |
| Major changes | 3+ rounds of comments | 2 👎 | "Improve quality" |
| Style only | Only formatting feedback | 3 | "Style not critical" |
| Rejected PR | Cancelled after work | 1 | "Major issue" |
| Repeated issue | Same bug again | 2 | "Pattern not learned" |
| First try success | Works on first attempt | 5 | "Excellent" |
| Pattern confirmed | Santi confirms pattern | +0.3 | "Confirmed rule" |
| Error avoided | Previous error prevented | +0.2 | "Good prevention" |

**Implementation:**

```rust
fn analyze_implicit_feedback(task: &Task, result: &Result) -> Feedback {
    let signals = detect_signals(result);
    
    let rating = match signals {
        ["quick_approval", "no_comments"] => 5,
        ["quick_approval"] => 4,
        ["approval_with_comments"] => 3,
        ["changes_requested_1"] => 3,
        ["changes_requested_2"] => 2,
        ["changes_requested_3+"] => 1,
        _ => 3, // default neutral
    };
    
    Feedback {
        rating,
        feedback_type: "implicit",
        signals,
        created_at: now()
    }
}
```

#### 4.2 Verification Loop

When the system detects a pattern with confidence >= 0.7:
→ Flag for Santi verification
→ Store in `pending_verification` queue
→ Show in dashboard

When Santi verifies:
- **"Confirm"** → Set verified=TRUE, add to skills
- **"Reject"** → Set verified=FALSE, mark as rejected
- **"Modify"** → Edit pattern and re-evaluate

**Verification Queue:**

```sql
CREATE TABLE pending_verification (
    id TEXT PRIMARY KEY,
    pattern_id TEXT REFERENCES learned_patterns(id),
    confidence REAL NOT NULL,
    evidence TEXT,  -- JSON array of examples
    suggested_action TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    status TEXT DEFAULT 'pending'  -- pending|confirmed|rejected|modified
);
```

**Verification Workflow:**

```
Pattern detected (confidence >= 0.7)
    │
    ▼
Add to pending_verification queue
    │
    ▼
Show in Dashboard (funemon learning verify --pending)
    │
    ├── Santi: "Confirm" ──→ verified=TRUE ──→ Add to LEARNED.md
    │
    ├── Santi: "Reject" ──→ verified=FALSE ──→ Archive pattern
    │
    └── Santi: "Modify" ──→ Edit pattern ──→ Re-evaluate confidence
```

**Auto-elevation:**

Patterns with confidence >= 0.9 are automatically marked as "highly likely" and surfaced first in verification queue.

---

#### 4.3 Explicit Feedback

**Trigger Conditions:**
- Ambiguous result (multiple valid interpretations)
- New agent interaction
- Cross-team delegation
- Major feature completion

**UI Prompt:**

```
"How was this? (1-5)
1 = Poor
2 = Fair
3 = Good
4 = Very Good
5 = Excellent

Optional: Add comments"
```

**Implementation:**

```rust
fn should_request_explicit_feedback(task: &Task) -> bool {
    // Ambiguous results
    if task.result.has_ambiguity() { return true; }
    
    // First time with this agent
    if is_first_interaction(task.to_agent) { return true; }
    
    // Cross-team
    if is_cross_team(task) { return true; }
    
    // Major milestone
    if task.is_major_feature() { return true; }
    
    false
}
```

#### 4.4 Pattern Recognition

**Learning from Feedback:**

```rust
fn learn_from_feedback(feedback: &Feedback) {
    // Store pattern
    let pattern = Pattern {
        agent: feedback.to_agent,
        signals: feedback.signals,
        rating: feedback.rating,
        context: feedback.task_context
    };
    
    db::insert("learned_patterns", pattern);
    
    // Adjust agent behavior
    adjust_agent_parameters(feedback.to_agent, feedback.rating);
}

fn get_agent_insights(agent: &str) -> Insights {
    // Aggregate feedback for agent
    let recent = db::query("feedback", { 
        to_agent: agent,
        limit: 20 
    });
    
    Insights {
        average_rating: average(recent.ratings),
        common_issues: extract_common(recent, "negative"),
        strengths: extract_common(recent, "positive"),
        recommendations: generate_recommendations(recent)
    }
}
```

#### 4.5 Feedback Storage

**Schema:** `feedback` table (see DB section)

**Query Interface:**

```rust
fn get_agent_performance(agent: &str, time_range: Range) -> Performance {
    let feedbacks = db::query("feedback", {
        to_agent: agent,
        created_at: time_range
    });
    
    Performance {
        total_tasks: feedbacks.len(),
        average_rating: feedbacks.avg(|f| f.rating),
        positive_count: feedbacks.count(|f| f.rating >= 4),
        negative_count: feedbacks.count(|f| f.rating <= 2),
        improvement_trend: calculate_trend(feedbacks)
    }
}
```

---

## Phase 5: Learning System

**Duration:** 10-12 hours
**Priority:** MEDIUM
**Dependencies:** Phase 4 completed

### Goals

- Implement pattern detection
- Learn Santi preferences
- Build nightly processor
- Generate insights for agents

### Features

#### 5.1 Learned Patterns Table

```sql
CREATE TABLE learned_patterns (
    id TEXT PRIMARY KEY,
    trigger_pattern TEXT NOT NULL,
    recommended_action TEXT NOT NULL,
    confidence REAL NOT NULL,  -- 0.0-1.0
    evidence_count INTEGER NOT NULL,
    evidence_examples TEXT,  -- JSON array
    agent TEXT NOT NULL,
    first_seen INTEGER NOT NULL,
    last_updated INTEGER NOT NULL,
    verified BOOLEAN DEFAULT FALSE,
    active BOOLEAN DEFAULT TRUE
);
```

#### 5.2 Santi Preferences Table

```sql
CREATE TABLE santi_preferences (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,  -- code_style|workflow|communication
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    confidence REAL NOT NULL,
    examples TEXT,  -- JSON array
    verified BOOLEAN DEFAULT FALSE,
    last_seen INTEGER NOT NULL
);
```

#### 5.3 Nightly Processor

**Command:** `funemon learn nightly`

**Schedule:** Daily at 2am or manual trigger

**Processor Workflow:**

```
1. Load all feedback from last 24h
2. Load all memories from last 24h
3. Detect patterns:
   - If evidence >= 3 → mark as active pattern
   - If confidence >= 0.8 → mark as verified
4. Extract preferences:
   - If confidence >= 0.8 → mark as verified preference
5. Generate verification tasks for Santi
6. Update learned_patterns table
7. Update santi_preferences table
```

**Pattern Detection Algorithm:**

```rust
fn detect_patterns(feedbacks: Vec<Feedback>) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    
    // Group by context
    let grouped = group_by_context(feedbacks);
    
    for (context, group) in grouped {
        if group.len() >= 3 {  // Minimum evidence
            let confidence = calculate_confidence(&group);
            let action = infer_action(&group);
            
            patterns.push(Pattern {
                trigger: context,
                action,
                confidence,
                evidence: group.len(),
                verified: confidence >= 0.8
            });
        }
    }
    
    patterns
}

fn calculate_confidence(group: &[Feedback]) -> f64 {
    let ratings: f64 = group.iter().map(|f| f.rating as f64).sum();
    let avg = ratings / group.len() as f64;
    
    // Adjust by consistency
    let consistency = calculate_consistency(group);
    
    // Normalize to 0-1
    (avg / 5.0) * (0.5 + consistency * 0.5)
}
```

#### 5.4 Preference Extraction

**Categories:**

| Category | Examples |
|----------|----------|
| code_style | "indent: 2 spaces", "naming: snake_case" |
| workflow | "PR before merge", "tests before commit" |
| communication | "brief updates", "detailed docs" |

**Extraction Rules:**

```rust
fn extract_preference(memory: &Memory) -> Option<Preference> {
    if memory.type == "preference" && memory.confidence >= 0.7 {
        let parsed = parse_preference(memory.content)?;
        
        Some(Preference {
            category: parsed.category,
            key: parsed.key,
            value: parsed.value,
            confidence: memory.confidence,
            examples: vec![memory.content]
        })
    } else {
        None
    }
}
```

---

## Phase 6: Dashboard

**Duration:** 4-5 hours
**Priority:** MEDIUM
**Dependencies:** Phase 5 completed

### Goals

- Create transparency dashboard
- Show learning progress
- Enable pattern verification
- Display Santi preferences

### Features

#### 6.1 funemon learning report

**Command:** `funemon learning report`

**Output Format:**

```bash
$ funemon learning report

# 📊 Funemon Learning Report
Generated: 2026-04-15

## 🎯 Patterns Learned (3)
| Trigger | Action | Confidence | Evidence |
|---------|--------|------------|----------|
| "Magnus + auth" | "Usar PKCE" | 92% | 5 veces |
| "Bruno + tests" | "Coverage >= 90%" | 88% | 4 veces |
| "Aurora + design" | "Consultar Iris" | 85% | 3 veces |

## 💡 Santi Preferences (12)
### Code Style
- Indentación: 2 spaces (confianza: 95%)
- Naming: snake_case (confianza: 88%)
- Error handling: log + throw (confianza: 82%)

### Workflow
- PR antes de merge (confianza: 92%)
- Tests antes de commit (confianza: 85%)

## ⚠️ Pendiente Verificación (2)
1. "Magnus: ¿Usar tracing para logs?" (confianza: 75%)
   [Confirmar] [Rechazar] [Modificar]
2. "Bruno: Coverage mínimo 80%" (confianza: 70%)
   [Confirmar] [Rechazar] [Modificar]

## 📈 Agent Insights
### Magnus
- Average Rating: 4.2/5
- Strengths: Backend, security
- Areas to improve: Documentation

### Bruno
- Average Rating: 4.5/5
- Strengths: Tests, quality
- Areas to improve: Speed
```

#### 6.2 Verification Commands

```bash
# Ver aprendizajes pendientes de verificar
funemon learning verify --pending

# Verificar un aprendizaje específico
funemon learning verify --pattern-id "xxx"

# Ver preferencias aprendidas
funemon learning preferences

# Ver patrones activos
funemon learning patterns

# Generar reporte completo
funemon learning report
```

#### 6.3 Dashboard Implementation

```rust
fn display_dashboard() {
    let patterns = db::query("learned_patterns", { 
        verified: true,
        active: true 
    });
    
    let preferences = db::query("santi_preferences", {
        verified: true
    });
    
    let pending = db::query("pending_verification", {
        status: "pending"
    });
    
    render_report(patterns, preferences, pending);
}

fn render_report(patterns, preferences, pending) {
    println!("# 📊 Funemon Learning Report");
    println!("Generated: {}", now());
    
    println!("\n## 🎯 Patterns Learned ({})", patterns.len());
    // Table format...
    
    println!("\n## 💡 Santi Preferences ({})", preferences.len());
    // By category...
    
    println!("\n## ⚠️ Pendiente Verificación ({})", pending.len());
    // Interactive options...
}
```

---

## Phase 7: Skills Auto-Update

**Duration:** 6-8 hours
**Priority:** LOW
**Dependencies:** Phase 5 (Learning System)

### Goals

- Auto-generate skill updates
- Keep skills in sync with learned patterns
- Enable agent self-improvement

### Features

#### 7.1 Skill Directory Structure

```
~/.config/opencode/skills/
├── rust-db/
│   ├── SKILL.md           # Original skill definition
│   └── LEARNED.md         # Auto-generated from patterns
├── security/
│   ├── SKILL.md
│   └── LEARNED.md
├── backend/
│   ├── SKILL.md
│   └── LEARNED.md
├── frontend/
│   ├── SKILL.md
│   └── LEARNED.md
└── default/
    ├── SKILL.md
    └── LEARNED.md
```

#### 7.2 LEARNED.md Format

```markdown
# Aprendido Automáticamente
Generado: 2026-04-15

## Reglas de Santi (verificado)
| Regla | Confianza | Evidencias |
|-------|-----------|------------|
| "Usar PKCE para OAuth" | 92% | 5 |
| "Tests antes de merge" | 88% | 4 |
| "2 spaces indent" | 95% | 7 |

## Patrones Detectados
| Trigger | Acción | Confianza |
|---------|--------|-----------|
| `auth` + `oauth` | PKCE es OBLIGATORIO | 92% |
| `magnus` + `security` | Consultar Gabriela | 85% |

## Última Actualización
- Patterns added: 2
- Patterns removed: 0
- Preferences updated: 1
```

#### 7.3 Auto-Update Process

```rust
fn update_skills() {
    // Get all verified patterns
    let patterns = db::query("learned_patterns", { 
        verified: true 
    });
    
    // Get all verified preferences
    let preferences = db::query("santi_preferences", {
        verified: true
    });
    
    // Group by skill
    let by_skill = group_by_skill(patterns, preferences);
    
    for (skill_name, items) in by_skill {
        let learned_md = generate_learned_md(items);
        let path = format!("~/.config/opencode/skills/{}/LEARNED.md", skill_name);
        
        fs::write(path, learned_md)?;
    }
}

fn generate_learned_md(items: Vec<LearnedItem>) -> String {
    let mut md = String::new();
    
    md.push_str("# Aprendido Automáticamente\n");
    md.push_str(&format!("Generado: {}\n\n", now()));
    
    md.push_str("## Reglas de Santi (verificado)\n");
    md.push_str("| Regla | Confianza | Evidencias |\n");
    md.push_str("|-------|-----------|------------|\n");
    
    for item in items.iter().filter(|i| i.is_preference()) {
        md.push_str(&format!("| \"{}\" | {}% | {} |\n", 
            item.description, 
            (item.confidence * 100.0) as i32,
            item.evidence_count
        ));
    }
    
    md.push_str("\n## Patrones Detectados\n");
    md.push_str("| Trigger | Acción | Confianza |\n");
    md.push_str("|---------|--------|-----------|\n");
    
    for item in items.iter().filter(|i| i.is_pattern()) {
        md.push_str(&format!("| `{}` | {} | {}% |\n",
            item.trigger,
            item.action,
            (item.confidence * 100.0) as i32
        ));
    }
    
    md
}
```

#### 7.4 Skill Loading with Learned

```rust
fn load_skill(skill_name: &str) -> Skill {
    let skill_md = fs::read_to_string(
        format!("~/.config/opencode/skills/{}/SKILL.md", skill_name)
    )?;
    
    let learned_md = fs::read_to_string(
        format!("~/.config/opencode/skills/{}/LEARNED.md", skill_name)
    ).unwrap_or_default();
    
    Skill {
        base: parse_markdown(skill_md),
        learned: parse_learned(learned_md)
    }
}
```

---

## Shortcuts Reference

### Memory Shortcuts

| Shortcut | Syntax | Description |
|----------|--------|-------------|
| Quick Memory | `!m "text"` | Store with auto-categorization |
| Important | `!m! "text"` | Mark as learned pattern |
| Search | `!m? "query"` | Search memories |
| Force Save | `!m+ "text"` | High importance |
| Filter | `!m- "text"` | Low importance (noise) |

### Learning Shortcuts

| Shortcut | Syntax | Description |
|----------|--------|-------------|
| Learn | `!learn "text"` | Direct to learned patterns |
| Preference | `!learn "Santi prefers X"` | Store as preference |

### Delegation Shortcuts

| Shortcut | Syntax | Description |
|----------|--------|-------------|
| Delegate | `!d agent "task"` | Simple delegation |
| Urgent | `!d agent "task" --urgent` | High priority |
| External | `!d iris "design"` | Auto-prompts permission |

### Team Shortcuts

| Shortcut | Syntax | Description |
|----------|--------|-------------|
| Team Share | `!team "name" "msg"` | Share with team |
| Team Search | `!team? "query"` | Search all teams |

### System Shortcuts

| Shortcut | Syntax | Description |
|----------|--------|-------------|
| Context | `!context topic` | Load relevant context |
| Ask | `!ask "question"` | Query the system |
| Compress | `!compress` | Compress context at end |

### Learning Shortcuts

| Shortcut | Syntax | Description |
|----------|--------|-------------|
| Report | `funemon learning report` | Full learning dashboard |
| Verify | `funemon learning verify --pending` | Pending verifications |
| Preferences | `funemon learning preferences` | Santi preferences |
| Patterns | `funemon learning patterns` | Active patterns |
| Nightly | `funemon learn nightly` | Run nightly processor |

---

## Team Structure

### Magnus Team

```
Lead: Magnus
Scope: Backend Development
Members: Bruno, Almendra, Gabriela
Permissions: Full backend, tests, docs, security
External Access: Requires permission for Aurora, Iris
```

### Aurora Team

```
Lead: Aurora
Scope: Frontend Development
Members: Bruno, Iris, Almendra
Permissions: Full frontend, design, tests, docs
External Access: Requires permission for Magnus, Gabriela
```

### ATLAS Team

```
Lead: ATLAS
Scope: Project Management
Members: (ATLAS only - internal)
Permissions: Planning, backlog, user stories
External Access: None (internal only)
```

### Cross-Team Matrix

| From \ To | Magnus | Bruno | Gabriela | Almendra | Aurora | Iris |
|-----------|--------|-------|----------|----------|--------|------|
| Magnus | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ |
| Aurora | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |
| ATLAS | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |

✅ = Auto-approved
❌ = Permission required

---

## Dependencies & Roadmap

### Phase Dependencies

```
Phase 1 (Agent UX MVP)
    │
    └─→ Phase 2 (Team Memory)
              │
              └─→ Phase 3 (Autonomous Teams)
                        │
                        └─→ Phase 4 (Feedback Loop)
                                  │
                                  └─→ Phase 5 (Learning System)
                                            │
                                            ├─→ Phase 6 (Dashboard)
                                            │
                                            └─→ Phase 7 (Skills Auto-Update)
```

### Complete Timeline

| Phase | Duration | Start | End | Status | Priority |
|-------|----------|-------|-----|--------|----------|
| Phase 1: Agent UX MVP | 2-3h | Day 1 | Day 1 | - | P0 |
| Phase 2: Team Memory | 6-8h | Day 1-2 | Day 2 | - | P1 |
| Phase 3: Autonomous Teams | 5-6h | Day 2-3 | Day 3 | - | P2 |
| Phase 4: Feedback Loop | 8-10h | Day 3-4 | Day 4 | - | P2 |
| Phase 5: Learning System | 10-12h | Day 5-7 | Day 7 | - | P2 |
| Phase 6: Dashboard | 4-5h | Day 7-8 | Day 8 | - | P3 |
| Phase 7: Skills Auto-Update | 6-8h | Day 8-10 | Day 10 | - | P3 |

**Total CORE (Phase 1-4):** 21-27 hours across 3-4 days
**Total FULL (Phase 1-7):** 35-47 hours across 7-10 days
**Complete System Total:** 56-74 hours across 10-14 days

### Implementation Priority

#### CORE (Must Have)
1. **P0:** Phase 1 - Core shortcuts and memory
2. **P1:** Phase 2 - Team structure and shared memory
3. **P2:** Phase 3 - Autonomous workflows
4. **P2:** Phase 4 - Feedback loop

#### FULL (Nice to Have)
5. **P2:** Phase 5 - Learning system
6. **P3:** Phase 6 - Dashboard
7. **P3:** Phase 7 - Skills auto-update

### Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| DB schema changes | Medium | Implement migrations carefully |
| Permission logic complexity | High | Start simple, iterate |
| Feedback pattern accuracy | Medium | Start with explicit, learn implicit |
| Cross-team coordination | High | Clear rules, permission checkpoints |
| Learning system accuracy | Medium | Require verification, start conservative |
| Pattern overfitting | Medium | Limit confidence, require evidence |

---

## ROI Analysis

### Implementation Costs

Based on estimated hours at $30/hour:

| Phase | Hours | Cost | Notes |
|-------|-------|------|-------|
| **CORE (Phase 1-4)** | | | |
| Phase 1: Agent UX MVP | 2-3h | $60-$90 | Foundation |
| Phase 2: Team Memory | 6-8h | $180-$240 | Team structure |
| Phase 3: Autonomous Teams | 5-6h | $150-$180 | Workflows |
| Phase 4: Feedback Loop | 8-10h | $240-$300 | Learning base |
| **CORE Subtotal** | **21-27h** | **$630-$810** | |
| **FULL (Phase 5-7)** | | | |
| Phase 5: Learning System | 10-12h | $300-$360 | Intelligence |
| Phase 6: Dashboard | 4-5h | $120-$150 | Transparency |
| Phase 7: Skills Auto-Update | 6-8h | $180-$240 | Automation |
| **FULL Subtotal** | **20-25h** | **$600-$750** | |
| **COMPLETE TOTAL** | **41-52h** | **$1,230-$1,560** | |

*Note: Original estimate was 56-74h, refined to 41-52h after removing overlap*

### Expected Benefits (6 months)

| Benefit | Value | Calculation |
|---------|-------|-------------|
| Santi time saved | $1,200-$2,400 | 40-80h × $30/hr |
| Reduced rework | $720-$1,080 | 15% fewer hours × 20h/week × 24 weeks |
| Fewer bugs in prod | Priceless | 60% reduction in production issues |
| Faster onboarding | $300-$450 | New agents learn 50% faster |
| Better quality PRs | $600-$900 | Fewer review cycles |

**Total Expected Benefits:** $2,820-$4,830 over 6 months

### ROI Calculation

| Metric | Value |
|--------|-------|
| Investment (CORE) | $630-$810 |
| Investment (FULL) | $1,230-$1,560 |
| Expected Benefits (6mo) | $2,820-$4,830 |
| **CORE Break-even** | 2-3 months |
| **FULL Break-even** | 4-6 months |
| **Annual ROI (CORE)** | 180-340% |
| **Annual ROI (FULL)** | 130-200% |

### Long-term Value

After break-even:
- Net monthly savings: $120-$400 (CORE) or $60-$200 (FULL)
- Continuous learning improves agents over time
- Pattern detection prevents repeated mistakes
- Dashboard provides transparency and control

### Recommendation

**Start with CORE (Phase 1-4)** to establish foundation.
**Add phases 5-7** once CORE is stable and delivering value.

---

## Appendix A: Example Flows

### Example 1: Quick Memory

```
User: "!m Use PKCE for OAuth, it's more secure"
  → Parser: detect !m prefix
  → Content: "Use PKCE for OAuth, it's more secure"
  → Importance: +0.2 (contains "more secure")
  → Category: auto-detect as "pattern"
  → Store in memories table
  → Return: "Memory stored: pattern learned"
```

### Example 2: Team Delegation

```
User: "!d bruno auth tests"
  → Parser: detect !d prefix
  → Agent: Bruno
  → Task: "auth tests"
  → Check: Bruno is in Magnus Team
  → Create delegation
  → Return: "Delegated to Bruno: auth tests"
  → Bruno receives task, completes, returns result
```

### Example 3: External Delegation (Permission)

```
User: "!d iris design login"
  → Parser: detect !d prefix
  → Agent: Iris
  → Task: "design login"
  → Check: Iris is NOT in Magnus Team
  → Permission Checkpoint triggered
  → Tyrion evaluates: "Is Iris necessary?"
  → If yes: "Approved. Delegating to Iris..."
  → If no: "Denied. Suggest: Aurora can help instead"
```

### Example 4: Feedback Capture

```
Tyrion receives: "Feature X completed"
  → Task completed successfully
  → Wait for user response (PR review)
  → If PR approved in < 1hr with no comments:
    → Capture implicit feedback: rating = 5
    → Store in feedback table
    → Update agent performance metrics
```

---

## Appendix B: Testing Requirements

### Phase 1 Tests

- [ ] Shortcut parser recognizes all prefixes
- [ ] Importance detection calculates correctly
- [ ] Auto-categorization works for all types
- [ ] Simple delegation creates tasks correctly
- [ ] External agent delegation prompts confirmation

### Phase 2 Tests

- [ ] Team configuration loads correctly
- [ ] Team memories are stored and retrieved
- [ ] Team search returns relevant results
- [ ] Cross-team visibility works as expected

### Phase 3 Tests

- [ ] Permission checkpoints trigger correctly
- [ ] Sub-delegation within team works
- [ ] External delegation requires permission
- [ ] Results are filtered for "final only"

### Phase 4 Tests

- [ ] Implicit signals are detected correctly
- [ ] Explicit feedback prompt appears when needed
- [ ] Patterns are learned and applied
- [ ] Agent insights are generated accurately

### Phase 5 Tests (Learning System)

- [ ] Learned patterns table stores patterns correctly
- [ ] Santi preferences table stores preferences correctly
- [ ] Nightly processor runs successfully
- [ ] Pattern detection identifies correct patterns
- [ ] Confidence calculation is accurate
- [ ] Patterns with 3+ evidence are marked active
- [ ] Preferences with confidence >= 0.8 are marked verified

### Phase 6 Tests (Dashboard)

- [ ] `funemon learning report` generates correct output
- [ ] `funemon learning verify --pending` shows pending items
- [ ] `funemon learning verify --pattern-id` verifies specific pattern
- [ ] `funemon learning preferences` displays preferences correctly
- [ ] `funemon learning patterns` displays patterns correctly
- [ ] Dashboard formatting is correct and readable

### Phase 7 Tests (Skills Auto-Update)

- [ ] LEARNED.md files are generated for each skill
- [ ] Skills directory structure is correct
- [ ] Learned content is accurate and up-to-date
- [ ] Auto-update process runs without errors
- [ ] Skills load with learned content included

---

**Document Version:** 2.0
**Last Updated:** 2026-04-15
**Next Review:** Before Phase 1 implementation
**Changelog:** Added Phase 5-7, Dashboard, Verification Loop, Team Lead Priority, Skills Auto-Update, ROI Analysis, expanded Phase 1 and 4