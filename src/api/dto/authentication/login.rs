use serde::{Deserialize, Serialize};

use crate::{
    api::dto::user::UserDTO,
    domain::models::authentication::{LogInSuccessful, LoginUser},
};

#[derive(Deserialize, Serialize)]
pub struct LoginRequestDTO {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponseDTO {
    pub user: UserDTO,
    pub jwt: String,
}

impl Into<LoginRequestDTO> for LoginUser {
    fn into(self) -> LoginRequestDTO {
        LoginRequestDTO {
            email: self.email,
            password: self.password,
        }
    }
}

impl Into<LoginUser> for LoginRequestDTO {
    fn into(self) -> LoginUser {
        LoginUser {
            email: self.email,
            password: self.password,
        }
    }
}

impl Into<LoginResponseDTO> for LogInSuccessful {
    fn into(self) -> LoginResponseDTO {
        LoginResponseDTO {
            user: self.user.into(),
            jwt: self.jwt,
        }
    }
}
