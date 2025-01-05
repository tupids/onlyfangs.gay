use axum::{
    routing::{get, post},
    Json, Router,
};

use crate::database::types::Application;

use super::error::ApiError;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_applications))
        .route("/submit", post(submit_application))
}

async fn get_applications() -> Result<Json<Vec<Application>>, ApiError> {
    Err(ApiError::not_implemented())
}

#[derive(serde::Deserialize)]
struct SubmitApplicationRequest {}

#[derive(serde::Serialize)]
struct SubmitApplicationResponse {}

async fn submit_application(
    body: Json<SubmitApplicationRequest>,
) -> Result<Json<SubmitApplicationResponse>, ApiError> {
    Err(ApiError::not_implemented())
}
