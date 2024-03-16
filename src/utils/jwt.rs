use std::env;

use actix_web::http::header::{HeaderMap, AUTHORIZATION};
use base64::{engine::general_purpose, Engine as _};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::error::CommonError;

extern crate base64;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct JWTPayload {
    #[validate(length(min = 1))]
    aud: String,
    #[validate(range(min = 0))]
    exp: i64,
    #[validate(range(min = 0))]
    iat: i64,
    #[validate(length(min = 1))]
    iss: String,
    #[validate(length(min = 1))]
    jti: String,
    #[validate(range(min = 0))]
    nbf: i64,
    #[validate(length(min = 1))]
    sub: String,
}

pub fn validate_jwt_payload(jwt_payload: JWTPayload) -> Result<JWTPayload, CommonError> {
    jwt_payload.validate().map_err(|e| CommonError {
        message: e.to_string(),
        code: 400,
    })?;

    Ok(jwt_payload)
}

pub fn sign_jws(jti: Uuid, id: Uuid, expiration: i64) -> Result<String, CommonError> {
    #[derive(Debug, Serialize)]
    struct Claims {
        aud: String,
        exp: i64,
        iat: i64,
        iss: String,
        jti: String,
        nbf: i64,
        sub: String,
    }

    let payload = Claims {
        aud: "jwt_audience".to_string(),
        exp: (Utc::now() + Duration::minutes(expiration)).timestamp(),
        iat: Utc::now().timestamp(),
        iss: "jwt_issuer".to_string(),
        jti: jti.to_string(),
        nbf: Utc::now().timestamp(),
        sub: id.to_string(),
    };

    let header = Header::new(Algorithm::EdDSA);

    let b64_key = env::var("JWT_PRIVATE_KEY").map_err(|e| CommonError {
        message: e.to_string(),
        code: 500,
    })?;

    let buffer_key = &general_purpose::STANDARD
        .decode(b64_key)
        .map_err(|e| CommonError {
            message: e.to_string(),
            code: 500,
        })?;

    let private_key = EncodingKey::from_ed_pem(&buffer_key).map_err(|e| CommonError {
        message: e.to_string(),
        code: 500,
    })?;

    encode(&header, &payload, &private_key).map_err(|e| CommonError {
        message: "Error signing Jwt:".to_string() + &e.to_string(),
        code: 500,
    })
}

pub async fn verify_token(token: &str) -> Result<JWTPayload, CommonError> {
    let b64_key = env::var("JWT_PUBLIC_KEY").map_err(|e| CommonError {
        message: e.to_string(),
        code: 500,
    })?;

    let buffer_key = &general_purpose::STANDARD
        .decode(b64_key)
        .map_err(|e| CommonError {
            message: e.to_string(),
            code: 500,
        })?;

    let public_key = DecodingKey::from_rsa_pem(&buffer_key).map_err(|e| CommonError {
        message: e.to_string(),
        code: 500,
    })?;

    let token_data =
        decode::<JWTPayload>(token, &public_key, &Validation::default()).map_err(|e| {
            CommonError {
                message: e.to_string(),
                code: 500,
            }
        })?;

    Ok(token_data.claims)
}

pub fn extract_jwt(headers: &HeaderMap) -> Result<String, CommonError> {
    let token = headers
        .get(AUTHORIZATION)
        .map(|h| h.to_str().unwrap().split_at(7).1.to_string());

    if token.is_none() {
        return Err(CommonError {
            message: "You are not logged in, please provide token".to_string(),
            code: 500,
        });
    }

    token.ok_or(CommonError {
        message: "You are not logged in, please provide token".to_string(),
        code: 500,
    })
}