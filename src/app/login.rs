use std::sync::Arc;

use axum::routing::{get, post};
use axum::{Json, Router};

use super::auth::User;
use super::error::ApiError;
use crate::global::Global;

pub fn routes() -> Router<Arc<Global>> {
    Router::new().route("/", get(login)).route("/complete", post(login_complete))
}

#[derive(serde::Serialize)]
struct LoginResponse {
    url: String,
}

/// GET /login
/// Start the login process
/// Scope: none
async fn login() -> Result<Json<LoginResponse>, ApiError> {
    Err(ApiError::not_implemented())
}

#[derive(serde::Deserialize)]
struct LoginCompleteRequest {
    code: String,
}

#[derive(serde::Serialize)]
struct LoginCompleteResponse {
    token: String,
    user: User,
    is_admin: bool,
}

/// POST /login/complete
/// Complete the login process
/// Scope: none
async fn login_complete(body: Json<LoginCompleteRequest>) -> Result<Json<LoginCompleteResponse>, ApiError> {
    Err(ApiError::not_implemented())
}
