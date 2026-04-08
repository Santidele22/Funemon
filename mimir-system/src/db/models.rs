use rusqlite::{
    types::{FromSql, FromSqlError, ToSql, ValueRef},
    Error as RusqliteError,
};
use serde::{Deserialize, Serialize};
use std::fmt;

use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum MemoryType {
    OBSERVATION,
    ERROR,
    PLAN,
    PREFERENCES,
}

impl fmt::Display for MemoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryType::OBSERVATION => write!(f, "observation"),
            MemoryType::ERROR => write!(f, "error"),
            MemoryType::PLAN => write!(f, "plan"),
            MemoryType::PREFERENCES => write!(f, "preference"),
        }
    }
}

impl FromSql for MemoryType {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        let s = value.as_str()?;
        match s.to_lowercase().as_str() {
            "observation" => Ok(MemoryType::OBSERVATION),
            "error" => Ok(MemoryType::ERROR),
            "plan" => Ok(MemoryType::PLAN),
            "preference" | "preferences" => Ok(MemoryType::PREFERENCES),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for MemoryType {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>, RusqliteError> {
        Ok(self.to_string().into())
    }
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
    pub reflection_id: String,
    pub session_id: String,
    pub content: String,
    pub r#type: String,
    pub importance: i32,
    pub level: String,
    pub source_summary: Option<String>,
    pub created_at: i64,
}
