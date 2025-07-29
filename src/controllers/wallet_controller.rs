use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    AppState,
    services::wallet_service,
    models::wallet::{CreateWalletRequest, DepositRequest, WithdrawRequest, UpdatePinRequest},
};

pub async fn get_wallet(State(state): State<AppState>) -> impl IntoResponse {
    match wallet_service::get_wallet(&state).await {
        Ok(wallet) => (StatusCode::OK, Json(wallet)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn create_wallet(
    State(state): State<AppState>,
    Json(payload): Json<CreateWalletRequest>,
) -> impl IntoResponse {
    match wallet_service::create_wallet(&state, payload).await {
        Ok(wallet) => (StatusCode::CREATED, Json(wallet)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn deposit_wallet(
    State(state): State<AppState>,
    Json(payload): Json<DepositRequest>,
) -> impl IntoResponse {
    match wallet_service::deposit(&state, payload).await {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn wallet_withdraw(
    State(state): State<AppState>,
    Json(payload): Json<WithdrawRequest>,
) -> impl IntoResponse {
    match wallet_service::withdraw(&state, payload).await {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn update_withdrawal_pin(
    State(state): State<AppState>,
    Json(payload): Json<UpdatePinRequest>,
) -> impl IntoResponse {
    match wallet_service::update_pin(&state, payload).await {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
