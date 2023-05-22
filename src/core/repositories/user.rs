use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::db::postgresql::DBConn;
use crate::core::error::DieselRepositoryError;
use crate::core::models::user::{CreateUserDiesel, UpdateUserDiesel, UserDiesel};
use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::domain::repositories::repository::RepositoryResult;
use crate::domain::repositories::user::UserRepository;

pub struct UserDieselRepository {
    pub pool: Arc<DBConn>,
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
        let result: UserDiesel = run(move || {
            diesel::insert_into(users)
                .values(new_user_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }

    async fn find_by_email(&self, email: &String) -> RepositoryResult<Option<User>> {
        use crate::core::schema::users::dsl::{email as email_dsl, users};
        let mut conn = self.pool.get().unwrap();
        let email = email.clone().to_lowercase();
        let result: Option<UserDiesel> = run(move || {
            users
                .filter(email_dsl.eq(email))
                .first(&mut conn)
                .optional()
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.map(|v| v.into()))
    }

    async fn update(&self, id: Uuid, user: &UpdateUser) -> RepositoryResult<User> {
        use crate::core::schema::users::dsl::{id as id_dsl, users};
        let updated_user_diesel: UpdateUserDiesel = UpdateUserDiesel::from(user.clone());

        let mut conn = self.pool.get().unwrap();
        let result: UserDiesel = run(move || {
            diesel::update(users.filter(id_dsl.eq(id)))
                .set(updated_user_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|v| DieselRepositoryError::from(v).into_inner())?;
        Ok(result.into())
    }
}
