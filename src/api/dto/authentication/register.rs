use serde::{Deserialize, Serialize};

use crate::{
    api::dto::user::UserDTO,
    domain::models::{
        authentication::{RegisterSuccessful, RegisterUser},
        user::CreateUser,
    },
};

#[derive(Deserialize, Serialize)]
pub struct RegisterRequestDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub profile_image: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterResponseDTO {
    pub user: UserDTO,
    pub jwt: String,
}

impl Into<RegisterRequestDTO> for RegisterUser {
    fn into(self) -> RegisterRequestDTO {
        RegisterRequestDTO {
            username: self.username,
            email: self.email,
            password: self.password,
            full_name: self.full_name,
            profile_image: self.profile_image,
        }
    }
}

impl Into<RegisterUser> for RegisterRequestDTO {
    fn into(self) -> RegisterUser {
        RegisterUser {
            username: self.username,
            email: self.email,
            password: self.password,
            full_name: self.full_name,
            profile_image: self.profile_image,
        }
    }
}

impl Into<RegisterResponseDTO> for RegisterSuccessful {
    fn into(self) -> RegisterResponseDTO {
        RegisterResponseDTO {
            user: self.user.into(),
            jwt: self.jwt,
        }
    }
}

impl Into<CreateUser> for RegisterUser {
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
