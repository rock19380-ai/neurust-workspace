use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt;

// --- Enums ---

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Team,
    Pro,
    Free,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "device_code_status", rename_all = "snake_case")]
pub enum DeviceStatus {
    Pending,
    Verified,
    Expired,
}

// --- Structs ---

#[derive(Debug, Serialize, Deserialize,Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub wallet_address: String,
    pub role: UserRole,
    pub credits: i32,
    pub team_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DeviceFlow {
    pub device_code: String,
    pub user_code: String,
    pub status: DeviceStatus,
    pub wallet_address: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key_hash: String,
    pub name: String,
    pub is_active: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UsageLog {
    pub id: i64,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub model_used: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    // sqlx maps DECIMAL to rust_decimal::Decimal or bigdecimal::BigDecimal usually.
    // For simplicity in this MVP, we might cast to f64 in queries or use string if needed.
    // Here we assume f64 via sqlx feature "postgres"
    pub cost_usd: f64, 
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct KnowledgeItem {
    pub id: i64,
    pub topic: String,
    pub source_url: String,
    pub content: String,
    pub last_scraped_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}