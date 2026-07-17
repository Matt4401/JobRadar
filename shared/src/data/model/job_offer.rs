use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct OptionalField {
    pub salary: Option<String>,
    pub study_level: Option<String>,
    pub contract_type: Option<String>,
    pub duration: Option<String>,
    pub remote: Option<String>,
    pub experience_level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct JobOffer {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub company: String,
    pub location: String,
    pub url: String,
    pub score: i32,
    pub created_at: NaiveDateTime,

    pub optional_field: OptionalField,
}
