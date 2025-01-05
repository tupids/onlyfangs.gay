use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::prelude::Insertable;
use diesel::query_dsl::methods::{FilterDsl, FindDsl, SelectDsl};
use diesel::{ExpressionMethods, SelectableHelper};
use diesel_async::RunQueryDsl;

use super::auth::{TwitchAdminUser, TwitchUser};
use super::error::ApiError;
use crate::database::enums::ApplicationStatus;
use crate::database::schema;
use crate::database::types::{Application, ApplicationComment};
use crate::global::Global;

pub fn routes() -> Router<Arc<Global>> {
    Router::new()
        .route("/:id", get(get_application))
        .route("/:id", post(update_application))
        .route("/:id/comment", post(add_comment))
        .route("/:id/comments", get(get_comments))
}

/// GET /applications/:id
/// Get an application by id
/// Scope: user (own application) or admin (any application)
async fn get_application(
    State(global): State<Arc<Global>>,
    Path(id): Path<i32>,
    TwitchUser(user): TwitchUser,
) -> Result<Json<Application>, ApiError> {
    let mut db = global.database.get().await.map_err(|err| {
        tracing::error!("Failed to get database: {err}");
        ApiError::internal_server_error()
    })?;

    let application = Application::fetch_by_id(&mut db, id)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch application: {err}");
            ApiError::internal_server_error()
        })?
        .ok_or_else(ApiError::not_found)?;

    if application.twitch_id != user.twitch_user_id && !global.config.admin_twitch_ids.contains(&user.twitch_user_id) {
        return Err(ApiError::not_found());
    }

    Ok(Json(application))
}

#[derive(serde::Deserialize)]
struct UpdateApplicationRequest {
    status: ApplicationStatus,
}

#[derive(serde::Serialize)]
struct UpdateApplicationResponse {
    application_id: i32,
}

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::application_comments)]
struct InsertComment<'a> {
    application_id: i32,
    comment: &'a str,
    twitch_user_id: i32,
    twitch_username: &'a str,
    twitch_display_name: &'a str,
    twitch_profile_image_url: &'a str,
}

/// POST /applications/:id
/// Update an application by id
/// Scope: admin
async fn update_application(
    State(global): State<Arc<Global>>,
    Path(id): Path<i32>,
    TwitchAdminUser(user): TwitchAdminUser,
    body: Json<UpdateApplicationRequest>,
) -> Result<Json<UpdateApplicationResponse>, ApiError> {
    let mut db = global.database.get().await.map_err(|err| {
        tracing::error!("Failed to get database: {err}");
        ApiError::internal_server_error()
    })?;

    let application = Application::fetch_by_id(&mut db, id)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch application: {err}");
            ApiError::internal_server_error()
        })?
        .ok_or_else(ApiError::not_found)?;

    diesel::update(schema::applications::dsl::applications.find(id))
        .set((
            schema::applications::dsl::status.eq(body.status),
            schema::applications::dsl::updated_at.eq(chrono::Utc::now()),
        ))
        .execute(&mut db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to update application: {err}");
            ApiError::internal_server_error()
        })?;

    let comment = match body.status {
        ApplicationStatus::Approved => "Moved to accepted",
        ApplicationStatus::Rejected => "Moved to rejected",
        ApplicationStatus::Maybe => "Moved to maybe",
        ApplicationStatus::Pending => "Moved to pending",
    };

    diesel::insert_into(schema::application_comments::dsl::application_comments)
        .values(InsertComment {
            application_id: id,
            comment,
            twitch_user_id: user.twitch_user_id,
            twitch_username: &user.twitch_username,
            twitch_display_name: &user.twitch_display_name,
            twitch_profile_image_url: &user.twitch_profile_image_url,
        })
        .execute(&mut db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to add comment: {err}");
            ApiError::internal_server_error()
        })?;

    Ok(Json(UpdateApplicationResponse { application_id: application.id }))
}

#[derive(serde::Deserialize)]
struct AddCommentRequest {
    comment: String,
}

#[derive(serde::Serialize)]
struct AddCommentResponse {
    comment_id: i32,
}

/// POST /applications/:id/comment
/// Add a comment to an application
/// Scope: user (own application) or admin (any application)
async fn add_comment(
    State(global): State<Arc<Global>>,
    Path(id): Path<i32>,
    TwitchUser(twitch_user_id): TwitchUser,
    body: Json<AddCommentRequest>,
) -> Result<Json<AddCommentResponse>, ApiError> {
    let mut db = global.database.get().await.map_err(|err| {
        tracing::error!("Failed to get database: {err}");
        ApiError::internal_server_error()
    })?;

    let application = Application::fetch_by_id(&mut db, id)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch application: {err}");
            ApiError::internal_server_error()
        })?
        .ok_or_else(ApiError::not_found)?;

    if application.twitch_id != twitch_user_id.twitch_user_id
        && !global.config.admin_twitch_ids.contains(&twitch_user_id.twitch_user_id)
    {
        return Err(ApiError::not_found());
    }

    if body.comment.len() > 1000 {
        return Err(ApiError::bad_request("comment too long"));
    }

    let comment_id = diesel::insert_into(schema::application_comments::dsl::application_comments)
        .values(InsertComment {
            application_id: id,
            comment: &body.comment,
            twitch_user_id: twitch_user_id.twitch_user_id,
            twitch_username: &twitch_user_id.twitch_username,
            twitch_display_name: &twitch_user_id.twitch_display_name,
            twitch_profile_image_url: &twitch_user_id.twitch_profile_image_url,
        })
        .returning(schema::application_comments::dsl::id)
        .get_result(&mut db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to add comment: {err}");
            ApiError::internal_server_error()
        })?;

    Ok(Json(AddCommentResponse { comment_id }))
}

/// GET /applications/:id/comments
/// Get comments for an application
/// Scope: user (own application) or admin (any application)
async fn get_comments(
    State(global): State<Arc<Global>>,
    Path(id): Path<i32>,
    TwitchUser(twitch_user_id): TwitchUser,
) -> Result<Json<Vec<ApplicationComment>>, ApiError> {
    let mut db = global.database.get().await.map_err(|err| {
        tracing::error!("Failed to get database: {err}");
        ApiError::internal_server_error()
    })?;

    let application = Application::fetch_by_id(&mut db, id)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch application: {err}");
            ApiError::internal_server_error()
        })?
        .ok_or_else(ApiError::not_found)?;

    if application.twitch_id != twitch_user_id.twitch_user_id
        && !global.config.admin_twitch_ids.contains(&twitch_user_id.twitch_user_id)
    {
        return Err(ApiError::not_found());
    }

    let comments = schema::application_comments::dsl::application_comments
        .filter(schema::application_comments::dsl::application_id.eq(id))
        .select(ApplicationComment::as_select())
        .load(&mut db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch comments: {err}");
            ApiError::internal_server_error()
        })?;

    Ok(Json(comments))
}
