use async_trait::async_trait;

use crate::domain::error::CommonError;
use crate::domain::models::user::{CreateUser, User};

#[async_trait]
pub trait UserService: Sync + Send {
    async fn create(&self, user: CreateUser) -> Result<User, CommonError>;
}
