use axum::{
    routing::{get, post},
    Json, Router,
};

use super::error::ApiError;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(login))
        .route("/complete", post(login_complete))
}

#[derive(serde::Serialize)]
struct LoginResponse {}

async fn login() -> Result<Json<LoginResponse>, ApiError> {
    Err(ApiError::not_implemented())
}

#[derive(serde::Deserialize)]
struct LoginCompleteRequest {}

#[derive(serde::Serialize)]
struct LoginCompleteResponse {}

async fn login_complete(
    body: Json<LoginCompleteRequest>,
) -> Result<Json<LoginCompleteResponse>, ApiError> {
    Err(ApiError::not_implemented())
}
