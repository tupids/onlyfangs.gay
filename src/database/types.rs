use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TwitchAccountType {
    Pleb,
    Affiliate,
    Partner,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationStatus {
    Pending,
    Approved,
    Maybe,
    Rejected,
}

#[derive(Debug, serde::Serialize)]
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
    pub comments: Vec<Comment>,
}

#[derive(Debug, serde::Serialize)]
pub struct Comment {
    pub id: i32,
    pub comment: String,
    pub twitch_user_id: i32,
    pub twitch_username: String,
    pub twitch_display_name: String,
    pub twitch_profile_image_url: String,
    pub created_at: DateTime<Utc>,
}
