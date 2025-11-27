use chrono::{DateTime, Utc}; 
use uuid::Uuid;
use sqlx::FromRow;
use serde::Serialize;


#[derive(Debug, FromRow, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub flagged: bool,
}