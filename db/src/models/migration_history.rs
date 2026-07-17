use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, Default)]
pub struct MigrationsHistory {
    pub id: i32,
    pub version: String,
    pub description: String,
    pub applied_at: NaiveDateTime,
    pub success: bool,
}

impl crate::models::Table for MigrationsHistory {
    const TABLE: &'static str = "migrations_history";
}
