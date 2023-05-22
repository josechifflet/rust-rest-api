use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::authentication::{
    LogInSuccessful, LoginUser, RegisterSuccessful, RegisterUser,
};
use crate::domain::models::user::CreateUser;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::authentication::AuthenticationService;
use crate::utils::argon2::{hash, verify_password};

#[derive(Clone)]
pub struct AuthenticationServiceImpl {
    pub user_repository: Arc<dyn UserRepository>,
}

impl AuthenticationServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        AuthenticationServiceImpl { user_repository }
    }
}

#[async_trait]
impl AuthenticationService for AuthenticationServiceImpl {
    async fn login(&self, login_data: LoginUser) -> Result<LogInSuccessful, CommonError> {
        let cloned = login_data.clone();

        let user_by_email = self
            .user_repository
            .find_by_email(&cloned.email)
            .await
            .map_err(|e| -> CommonError { e.into() });

        match user_by_email {
            Ok(user) => match user {
                Some(user) => match verify_password(&user.password, cloned.password.as_bytes()) {
                    Ok(_) => {
                        let session_token = uuid::Uuid::new_v4();
                        Ok(LogInSuccessful {
                            user: user.into(),
                            jwt: session_token.to_string(),
                        })
                    }
                    Err(_) => {
                        return Err(CommonError {
                            message: "Invalid Credentials".to_string(),
                            code: 401,
                        })
                    }
                },
                None => Err(CommonError {
                    message: "Invalid Credentials".to_string(),
                    code: 401,
                }),
            },
            Err(e) => Err(e),
        }
    }

    async fn register(
        &self,
        register_user: RegisterUser,
    ) -> Result<RegisterSuccessful, CommonError> {
        let cloned = register_user.clone();

        let user_by_email = self
            .user_repository
            .find_by_email(&cloned.email)
            .await
            .map_err(|e| -> CommonError { e.into() });

        match user_by_email {
            Ok(user) => match user {
                Some(_) => Err(CommonError {
                    message: "User already exists".to_string(),
                    code: 409,
                }),
                None => {
                    let user = self
                        .user_repository
                        .create(&CreateUser {
                            username: cloned.username,
                            email: cloned.email,
                            password: hash(cloned.password.as_bytes()),
                            full_name: cloned.full_name,
                            profile_image: cloned.profile_image,
                        })
                        .await
                        .map_err(|e| -> CommonError { e.into() })?;

                    let session_token = uuid::Uuid::new_v4();

                    Ok(RegisterSuccessful {
                        user: user.into(),
                        jwt: session_token.to_string(),
                    })
                }
            },
            Err(e) => Err(e),
        }
    }
}
