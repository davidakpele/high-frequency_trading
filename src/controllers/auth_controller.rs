use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::MySqlPool;
use crate::{responses::login_responses::LoginResponse, services::auth_service::{self}};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3))]
    pub username: String,

    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum AuthApiResponse<T> {
    Success(T),
    Error(ErrorResponse),
}
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn register_user(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Validate payload first
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Validation error: {}", e),
            }),
        )
            .into_response();
    }

    // Check if email exists first
    let user_repo = crate::repositories::user_repository::UserRepository { db: db.clone() };
    let user_service = crate::services::user_service::UserService::new(user_repo);
    
    match user_service.get_user_by_email(&payload.email).await {
        Ok(_) => {
            return (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    error: "Email already been used by another user, please provide another email address".to_string(),
                }),
            )
            .into_response();
        }
        Err(e) if !e.to_string().contains("not found") => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
            .into_response();
        }
        _ => {} // Continue if user not found
    }

    // Proceed with registration if email is available
    match auth_service::register_user(&db, payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}


pub async fn login_user(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(AuthApiResponse::Error(ErrorResponse {
                error: format!("Validation error: {}", e),
            })),
        );
    }

    match auth_service::login_user(&db, payload).await {
        Ok(auth_response) => {
            let login_response = LoginResponse {
                token: auth_response.token,
                id: auth_response.id,
                email: auth_response.email,
                username: auth_response.username,
            };
            (StatusCode::OK, Json(AuthApiResponse::Success(login_response)))
        },
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(AuthApiResponse::Error(ErrorResponse {
                error: e.to_string(),
            })),
        ),
    }

}