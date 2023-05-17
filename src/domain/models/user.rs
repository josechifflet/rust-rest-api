use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub profile_image: Option<String>,
    pub session_token: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub profile_image: Option<String>,
}
