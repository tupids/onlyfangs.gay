use chrono::{DateTime, Utc};
use diesel::pg::Pg;
use diesel::prelude::Queryable;
use diesel::query_dsl::methods::{FindDsl, SelectDsl};
use diesel::{OptionalExtension, Selectable, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use super::enums::{ApplicationStatus, TwitchAccountType};
use super::schema;

#[derive(Debug, serde::Serialize, Selectable, Queryable)]
#[diesel(table_name = schema::applications)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(Pg))]
pub struct Application {
    pub id: i32,
    pub twitch_id: i32,
    pub twitch_username: String,
    pub twitch_display_name: String,
    pub twitch_profile_image_url: String,
    pub twitch_account_type: TwitchAccountType,
    pub status: ApplicationStatus,
    pub reason: String,
    pub support_clip_url: String,
    pub follow_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Application {
    pub async fn fetch_by_id(conn: &mut AsyncPgConnection, id: i32) -> anyhow::Result<Option<Self>> {
        let application: Option<Self> = schema::applications::dsl::applications
            .find(id)
            .select(Application::as_select())
            .get_result(conn)
            .await
            .optional()?;

        Ok(application)
    }
}

#[derive(Debug, serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = schema::application_comments)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(Pg))]
pub struct ApplicationComment {
    pub id: i32,
    pub application_id: i32,
    pub comment: String,
    pub twitch_user_id: i32,
    pub twitch_username: String,
    pub twitch_display_name: String,
    pub twitch_profile_image_url: String,
    pub created_at: DateTime<Utc>,
}
