use serde::{Deserialize, Serialize};

use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum MemoryType {
    OBSERVATION,
    ERROR,
    PLAN,
    PREFERENCES,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ReflectionType {
    PATTERN,
    PRINCIPLE,
    WARNING,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ReflectionLevel {
    FACT,
    PATTERN,
    PRINCIPLE,
}
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Sessions {
    pub session_id: String,
    pub project: String,
    pub created_at: i64,
    pub last_active: i64,
    pub deleted_at: Option<i64>,
    pub ended_at: Option<i64>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Memories {
    pub memory_id: String,
    pub session_id: String,
    pub r#type: Option<String>,
    pub title: String,
    pub what: Option<String>,
    pub where_field: Option<String>,
    pub why: Option<String>,
    pub learned: Option<String>,
    pub created_at: i64,
    pub deleted_at: Option<i64>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Reflection {
    pub reflection_id: String,
    pub session_id: String,
    pub content: String,
    pub r#type: String,
    pub importance: i32,
    pub level: String,
    pub source_summary: Option<String>,
    pub created_at: i64,
}
