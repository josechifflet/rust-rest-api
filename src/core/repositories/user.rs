use std::sync::Arc;
use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;

use crate::domain::models::user::{CreateUser, User};
use crate::domain::repositories::repository::{RepositoryResult};
use crate::domain::repositories::user::{UserRepository};
use crate::core::error::DieselRepositoryError;
use crate::core::db::postgresql::DBConn;
use crate::core::models::user::{CreateUserDiesel, UserDiesel};

pub struct UserDieselRepository {
    pub pool: Arc<DBConn>
}

impl UserDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        UserDieselRepository { pool: db }
    }
}

#[async_trait]
impl UserRepository for UserDieselRepository {

    async fn create(&self, new_user: &CreateUser) -> RepositoryResult<User> {
        use crate::core::schema::users::dsl::users;
        let new_user_diesel: CreateUserDiesel = CreateUserDiesel::from(new_user.clone());
        let mut conn = self.pool.get().unwrap();
        let result: UserDiesel = run(move || diesel::insert_into(users).values(new_user_diesel)
            .get_result(&mut conn))
            .await
            .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }
}