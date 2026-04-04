use serde::{Deserialize, Serialize};

use schemars::{JsonSchema, schema_for};
use uuid::Uuid;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum MemoryType {
    observation,
    error,
    plan,
    preferences,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ReflectionType {
    pattern,
    principle,
    warning,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ReflectionLevel {
    Fact,      // level 1: hecho concreto
    Pattern,   // level 2: patrón recurrente
    Principle, // level 3: principio general
}
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Sessions {
    pub session_id: Uuid,
    pub project: String,
    pub created_at: i64,
    pub last_active: i64,
    pub deleted_at: Option<i64>,
    pub ended_at: Option<i64>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Memories {
    pub memory_id: Uuid,
    pub session_id: Uuid,
    pub r#type: Option<MemoryType>,
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
    pub project: String,
    pub reflection_id: Uuid,
    pub session_id: Uuid,
    pub content: String,
    pub r#type: ReflectionType,
    pub importance: i32,
    pub level: ReflectionLevel,
    pub source_summary: Option<String>,
    pub created_at: i64,
}
