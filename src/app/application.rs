use crate::database::types::Application;
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};

use super::error::ApiError;

pub fn routes() -> Router {
    Router::new()
        .route("/:id", get(get_application))
        .route("/:id", post(update_application))
        .route("/:id/comment", post(add_comment))
}

async fn get_application(id: Path<i32>) -> Result<Json<Application>, ApiError> {
    Err(ApiError::not_implemented())
}

#[derive(serde::Deserialize)]
struct UpdateApplicationRequest {}

#[derive(serde::Serialize)]
struct UpdateApplicationResponse {}

async fn update_application(
    id: Path<i32>,
    body: Json<UpdateApplicationRequest>,
) -> Result<Json<UpdateApplicationResponse>, ApiError> {
    Err(ApiError::not_implemented())
}

#[derive(serde::Deserialize)]
struct AddCommentRequest {
    comment: String,
}

#[derive(serde::Serialize)]
struct AddCommentResponse {
    id: i32,
}

async fn add_comment(
    id: Path<i32>,
    body: Json<AddCommentRequest>,
) -> Result<Json<AddCommentResponse>, ApiError> {
    Err(ApiError::not_implemented())
}
