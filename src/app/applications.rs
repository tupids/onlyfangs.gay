use std::sync::Arc;

use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use diesel::prelude::Insertable;
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

use super::auth::{TwitchAdminUser, TwitchUser};
use super::error::ApiError;
use crate::database::enums::{ApplicationStatus, TwitchAccountType};
use crate::database::schema;
use crate::database::types::Application;
use crate::global::Global;

pub fn routes() -> Router<Arc<Global>> {
    Router::new()
        .route("/", get(get_applications))
        .route("/me", get(get_my_applications))
        .route("/submit", post(submit_application))
}

#[derive(serde::Deserialize)]
struct GetApplicationsRequest {
    status: Option<ApplicationStatus>,
    twitch_account_type: Option<TwitchAccountType>,
    min_follow_count: Option<i32>,
    twitch_username: Option<String>,
}

/// GET /applications
/// Get all applications by some query filters
/// Scope: admin
async fn get_applications(
    State(global): State<Arc<Global>>,
    TwitchAdminUser(_): TwitchAdminUser,
    Query(request): Query<GetApplicationsRequest>,
) -> Result<Json<Vec<Application>>, ApiError> {
    let mut db = global.database.get().await.map_err(|err| {
        tracing::error!("Failed to get database: {err}");
        ApiError::internal_server_error()
    })?;

    let mut query = schema::applications::table.into_boxed();

    if let Some(status) = request.status {
        query = query.filter(schema::applications::dsl::status.eq(status));
    }

    if let Some(twitch_account_type) = request.twitch_account_type {
        query = query.filter(schema::applications::dsl::twitch_account_type.eq(twitch_account_type));
    }

    if let Some(min_follow_count) = request.min_follow_count {
        if min_follow_count < 0 {
            return Err(ApiError::bad_request("min_follow_count must be greater than 0"));
        }

        query = query.filter(schema::applications::dsl::follow_count.ge(min_follow_count));
    }

    if let Some(twitch_username) = request.twitch_username {
        if twitch_username.len() > 100 {
            return Err(ApiError::bad_request("twitch_username too long"));
        }

        query = query.filter(schema::applications::dsl::twitch_username.ilike(format!("%{}%", twitch_username)));
    }

    let applications = query
        .select(Application::as_select())
        .load::<Application>(&mut db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch applications: {err}");
            ApiError::internal_server_error()
        })?;

    Ok(Json(applications))
}

#[derive(serde::Deserialize)]
struct SubmitApplicationRequest {
    reason: String,
    support_clip_url: String,
}

#[derive(serde::Serialize)]
struct SubmitApplicationResponse {
    application_id: i32,
}

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::applications)]
struct InsertApplication {
    reason: String,
    support_clip_url: String,
    twitch_id: i32,
    twitch_username: String,
    twitch_display_name: String,
    twitch_profile_image_url: String,
    twitch_account_type: TwitchAccountType,
    follow_count: i32,
}

/// POST /applications/submit
/// Submit an application
/// Scope: user
async fn submit_application(
    State(global): State<Arc<Global>>,
    TwitchUser(user): TwitchUser,
    Json(body): Json<SubmitApplicationRequest>,
) -> Result<Json<SubmitApplicationResponse>, ApiError> {
    let mut db = global.database.get().await.map_err(|err| {
        tracing::error!("Failed to get database: {err}");
        ApiError::internal_server_error()
    })?;

    let application_id = diesel::insert_into(schema::applications::table)
        .values(InsertApplication {
            reason: body.reason,
            support_clip_url: body.support_clip_url,
            twitch_id: user.twitch_user_id,
            twitch_username: user.twitch_username,
            twitch_display_name: user.twitch_display_name,
            twitch_profile_image_url: user.twitch_profile_image_url,
            twitch_account_type: user.twitch_account_type,
            follow_count: user.follow_count,
        })
        .returning(schema::applications::dsl::id)
        .get_result(&mut db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to insert application: {err}");
            ApiError::internal_server_error()
        })?;

    Ok(Json(SubmitApplicationResponse { application_id }))
}

/// GET /applications/me
/// Get all applications for the current user
/// Scope: user
async fn get_my_applications(
    State(global): State<Arc<Global>>,
    TwitchUser(user): TwitchUser,
) -> Result<Json<Vec<Application>>, ApiError> {
    let mut db = global.database.get().await.map_err(|err| {
        tracing::error!("Failed to get database: {err}");
        ApiError::internal_server_error()
    })?;

    let applications = schema::applications::table
        .filter(schema::applications::dsl::twitch_id.eq(user.twitch_user_id))
        .select(Application::as_select())
        .load::<Application>(&mut db)
        .await
        .map_err(|err| {
            tracing::error!("Failed to fetch applications: {err}");
            ApiError::internal_server_error()
        })?;

    Ok(Json(applications))
}
