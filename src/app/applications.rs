use std::sync::Arc;

use axum::{
    routing::{get, post},
    Json, Router,
};

use crate::{database::types::Application, global::Global};

use super::{auth::TwitchUserId, error::ApiError};

pub fn routes() -> Router<Arc<Global>> {
    Router::new()
        .route("/", get(get_applications))
        .route("/submit", post(submit_application))
}

async fn get_applications(
    TwitchUserId(twitch_user_id): TwitchUserId,
) -> Result<Json<Vec<Application>>, ApiError> {
    Err(ApiError::not_implemented())
}

#[derive(serde::Deserialize)]
struct SubmitApplicationRequest {}

#[derive(serde::Serialize)]
struct SubmitApplicationResponse {}

async fn submit_application(
    TwitchUserId(twitch_user_id): TwitchUserId,
    body: Json<SubmitApplicationRequest>,
) -> Result<Json<SubmitApplicationResponse>, ApiError> {
    Err(ApiError::not_implemented())
}
