use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub profile_image: Option<String>,
    pub session_token: Option<Uuid>,
}
