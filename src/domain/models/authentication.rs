use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Clone, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub profile_image: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct LogInSuccessful {
    pub user: User,
    pub jwt: String,
}

#[derive(Clone, Deserialize)]
pub struct RegisterSuccessful {
    pub user: User,
    pub jwt: String,
}
