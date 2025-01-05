use std::sync::Arc;

use axum::{async_trait, http::request::Parts};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};

use crate::global::Global;

use super::error::ApiError;

#[derive(Clone)]
pub struct AuthLayer {
    pub jwt_secret: Arc<DecodingKey>,
}

impl<S> tower::Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            jwt_secret: self.jwt_secret.clone(),
            next: inner,
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    pub jwt_secret: Arc<DecodingKey>,
    pub next: S,
}

impl AuthLayer {
    pub fn new(jwt_secret: &str) -> Self {
        Self {
            jwt_secret: Arc::from(DecodingKey::from_secret(jwt_secret.as_bytes())),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    twitch_user_id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwitchUserId(pub i32);

#[async_trait]
impl axum::extract::FromRequestParts<Arc<Global>> for TwitchUserId {
    type Rejection = ApiError;

    async fn from_request_parts(
        req: &mut Parts,
        global: &Arc<Global>,
    ) -> Result<Self, Self::Rejection> {
        let token = req
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or_else(ApiError::unauthorized)?;

        let jwt_secret = DecodingKey::from_secret(global.config.jwt_secret.as_bytes());
        let Ok(claims) =
            jsonwebtoken::decode::<Claims>(token, &jwt_secret, &Validation::new(Algorithm::HS256))
        else {
            return Err(ApiError::unauthorized());
        };

        Ok(TwitchUserId(claims.claims.twitch_user_id))
    }
}
