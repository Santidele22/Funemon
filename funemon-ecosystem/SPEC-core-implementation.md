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
5. [Phase 2: Team Memory](#phase-2-team-memory)
6. [Phase 3: Autonomous Teams](#phase-3-autonomous-teams)
7. [Phase 4: Feedback Loop](#phase-4-feedback-loop)
8. [Shortcuts Reference](#shortcuts-reference)
9. [Team Structure](#team-structure)
10. [Dependencies & Roadmap](#dependencies--roadmap)

---

## Executive Summary

This SPEC defines the core implementation roadmap for the Funemon ecosystem. The implementation is divided into 4 phases spanning approximately 21-27 hours of development work.

### Goals

- Enable natural language shortcuts for memory, learning, and delegation
- Establish team-based memory with shared context
- Implement autonomous team workflows with permission checkpoints
- Create implicit and explicit feedback loops for continuous improvement

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

#### 1.2 Automatic Importance Detection

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

#### 4.1 Implicit Feedback

**Detection Rules:**

| Signal | Feedback | Interpretation |
|--------|----------|----------------|
| PR approved in < 1 hour | 👍 positive | Good quality, minimal changes needed |
| PR changes requested 3+ times | 👎 negative | Something missing or incorrect |
| Approved without comments | 👍 positive | Meets expectations |
| Code style corrections | 👎 negative | (but learned) - improve quality |
| Quick acceptance with "nice" | 👍 positive | Exceeds expectations |
| Repeated similar corrections | 👎 negative | Pattern issue |

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

#### 4.2 Explicit Feedback

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

#### 4.3 Pattern Recognition

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

#### 4.4 Feedback Storage

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
```

### Timeline

| Phase | Duration | Start | End | Status |
|-------|----------|-------|-----|--------|
| Phase 1: Agent UX MVP | 2-3h | Day 1 | Day 1 | - |
| Phase 2: Team Memory | 6-8h | Day 1-2 | Day 2 | - |
| Phase 3: Autonomous Teams | 5-6h | Day 2-3 | Day 3 | - |
| Phase 4: Feedback Loop | 8-10h | Day 3-4 | Day 4 | - |

**Total:** 21-27 hours across 3-4 days

### Implementation Priority

1. **P0:** Phase 1 - Core shortcuts and memory
2. **P1:** Phase 2 - Team structure and shared memory
3. **P2:** Phase 3 - Autonomous workflows
4. **P3:** Phase 4 - Feedback loop (nice to have)

### Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| DB schema changes | Medium | Implement migrations carefully |
| Permission logic complexity | High | Start simple, iterate |
| Feedback pattern accuracy | Medium | Start with explicit, learn implicit |
| Cross-team coordination | High | Clear rules, permission checkpoints |

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

---

**Document Version:** 1.0
**Last Updated:** 2026-04-15
**Next Review:** Before Phase 1 implementation