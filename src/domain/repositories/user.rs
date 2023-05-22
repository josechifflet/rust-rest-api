use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::domain::repositories::repository::{
    QueryParams, RepositoryResult, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for UserQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, new_user: &CreateUser) -> RepositoryResult<User>;
    async fn update(&self, id: Uuid, user: &UpdateUser) -> RepositoryResult<User>;
    async fn find_by_email(&self, email: &String) -> RepositoryResult<Option<User>>;
}
