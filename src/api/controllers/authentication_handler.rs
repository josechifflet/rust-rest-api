use crate::api::dto::authentication::login::LoginResponseDTO;
use crate::api::dto::authentication::register::{RegisterRequestDTO, RegisterResponseDTO};
use crate::domain::services::authentication::AuthenticationService;
use crate::{api::dto::authentication::login::LoginRequestDTO, domain::error::ApiError};
use actix_web::{web, Result};

pub async fn login_handler(
    authentication_service: web::Data<dyn AuthenticationService>,
    login_data: web::Json<LoginRequestDTO>,
) -> Result<web::Json<LoginResponseDTO>, ApiError> {
    let user = authentication_service
        .login(login_data.into_inner().into())
        .await?;

    Ok(web::Json(user.into()))
}

pub async fn register_handler(
    authentication_service: web::Data<dyn AuthenticationService>,
    register_data: web::Json<RegisterRequestDTO>,
) -> Result<web::Json<RegisterResponseDTO>, ApiError> {
    let user_registered = authentication_service
        .register(register_data.into_inner().into())
        .await?;

    Ok(web::Json(user_registered.into()))
}
