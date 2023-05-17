use diesel;
use diesel::prelude::*;
use crate::{domain::models::user::{CreateUser, User}, core::schema::users};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Queryable)]
pub struct UserDiesel {
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

/// Factory method for creating a new UserDiesel from a User
impl From<User> for UserDiesel {
    fn from(t: User) -> Self {
        UserDiesel {
            id: t.id,
            username: t.username,
            email: t.email,
            password: t.password,
            full_name: t.full_name,
            profile_image: t.profile_image,
            session_token: t.session_token,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDiesel {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub profile_image: Option<String>,
}

/// Factory method for creating a new User from a UserDiesel
impl Into<User> for UserDiesel {
    fn into(self) -> User {
        User {
            id: self.id,
            username: self.username,
            email: self.email,
            password: self.password,
            full_name: self.full_name,
            profile_image: self.profile_image,
            session_token: self.session_token,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Factory method for creating a new UserDiesel from a CreateUser
impl From<CreateUser> for CreateUserDiesel {
    fn from(t: CreateUser) -> Self {
        CreateUserDiesel {
            username: t.username,
            email: t.email,
            password: t.password,
            full_name: t.full_name,
            profile_image: t.profile_image,
        }
    }
}

/// Factory method for creating a new User from a CreateUserDiesel
impl Into<User> for CreateUserDiesel {
    fn into(self) -> User {
        User {
            id: Uuid::new_v4(),
            username: self.username,
            email: self.email,
            password: self.password,
            full_name: self.full_name,
            profile_image: self.profile_image,
            session_token: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}