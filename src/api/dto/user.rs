use crate::domain::models::user::{CreateUser, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CreateUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub profile_image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserDTO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub profile_image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<UserDTO> for User {
    fn into(self) -> UserDTO {
        UserDTO {
            id: self.id,
            username: self.username,
            email: self.email,
            full_name: self.full_name,
            profile_image: self.profile_image,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Into<CreateUser> for CreateUserDTO {
    fn into(self) -> CreateUser {
        CreateUser {
            username: self.username,
            email: self.email,
            password: self.password,
            full_name: self.full_name,
            profile_image: self.profile_image,
        }
    }
}

impl Into<CreateUserDTO> for CreateUser {
    fn into(self) -> CreateUserDTO {
        CreateUserDTO {
            username: self.username,
            email: self.email,
            password: self.password,
            full_name: self.full_name,
            profile_image: self.profile_image,
        }
    }
}
