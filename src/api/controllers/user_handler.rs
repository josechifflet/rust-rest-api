use crate::api::dto::user::{CreateUserDTO, UserDTO};
use crate::domain::error::ApiError;
use crate::domain::services::user::UserService;
use actix_web::{web, Result};

pub async fn register_user_handler(
    user_service: web::Data<dyn UserService>,
    user_data: web::Json<CreateUserDTO>,
) -> Result<web::Json<UserDTO>, ApiError> {
    todo!()
}
