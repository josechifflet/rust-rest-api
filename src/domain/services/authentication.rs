use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::authentication::{
    LogInSuccessful, LoginUser, RegisterSuccessful, RegisterUser,
};

#[async_trait]
pub trait AuthenticationService: Sync + Send {
    async fn login(&self, user: LoginUser) -> Result<LogInSuccessful, CommonError>;
    async fn register(&self, user: RegisterUser) -> Result<RegisterSuccessful, CommonError>;
}
