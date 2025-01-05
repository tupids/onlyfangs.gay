use std::sync::Arc;

use crate::{database::types::Application, global::Global};
use axum::{
    extract::Path,
    routing::{get, post}, Json, Router,
};

use super::{auth::TwitchUserId, error::ApiError};

pub fn routes() -> Router<Arc<Global>> {
    Router::new()
        .route("/:id", get(get_application))
        .route("/:id", post(update_application))
        .route("/:id/comment", post(add_comment))
}

async fn get_application(
    id: Path<i32>,
    TwitchUserId(twitch_user_id): TwitchUserId,
) -> Result<Json<Application>, ApiError> {
    Err(ApiError::not_implemented())
}

#[derive(serde::Deserialize)]
struct UpdateApplicationRequest {}

#[derive(serde::Serialize)]
struct UpdateApplicationResponse {}

async fn update_application(
    id: Path<i32>,
    TwitchUserId(twitch_user_id): TwitchUserId,
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
    TwitchUserId(twitch_user_id): TwitchUserId,
    body: Json<AddCommentRequest>,
) -> Result<Json<AddCommentResponse>, ApiError> {
    Err(ApiError::not_implemented())
}
