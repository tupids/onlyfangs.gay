use std::sync::Arc;

use axum::async_trait;
use axum::http::request::Parts;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

use super::error::ApiError;
use crate::database::enums::TwitchAccountType;
use crate::global::Global;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub twitch_user_id: i32,
    pub twitch_username: String,
    pub twitch_display_name: String,
    pub twitch_profile_image_url: String,
    pub twitch_account_type: TwitchAccountType,
    pub follow_count: i32,
}

#[derive(Debug, Clone)]
pub struct TwitchUser(pub User);

impl TwitchUser {
    pub async fn extract(req: &mut Parts, global: &Arc<Global>) -> Result<Self, ApiError> {
        let token = req
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(ApiError::unauthorized)?;

        let jwt_secret = DecodingKey::from_secret(global.config.jwt_secret.as_bytes());
        let Ok(claims) = jsonwebtoken::decode::<User>(token, &jwt_secret, &Validation::new(Algorithm::HS256)) else {
            return Err(ApiError::unauthorized());
        };

        Ok(TwitchUser(claims.claims))
    }
}

pub struct TwitchAdminUser(pub User);

#[async_trait]
impl axum::extract::FromRequestParts<Arc<Global>> for TwitchUser {
    type Rejection = ApiError;

    async fn from_request_parts(req: &mut Parts, global: &Arc<Global>) -> Result<Self, Self::Rejection> {
        TwitchUser::extract(req, global).await
    }
}

#[async_trait]
impl axum::extract::FromRequestParts<Arc<Global>> for TwitchAdminUser {
    type Rejection = ApiError;

    async fn from_request_parts(req: &mut Parts, global: &Arc<Global>) -> Result<Self, Self::Rejection> {
        let TwitchUser(twitch_user_id) = TwitchUser::extract(req, global).await?;

        if !global.config.admin_twitch_ids.contains(&twitch_user_id.twitch_user_id) {
            return Err(ApiError::unauthorized());
        }

        Ok(TwitchAdminUser(twitch_user_id))
    }
}
