use std::borrow::Cow;

use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Debug, serde::Serialize)]
pub struct ApiError {
    #[serde(serialize_with = "serialize_status_code")]
    pub status: StatusCode,
    pub message: Cow<'static, str>,
}

fn serialize_status_code<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u16(status.as_u16())
}

impl ApiError {
    pub const fn new(status: StatusCode, message: Cow<'static, str>) -> Self {
        ApiError { status, message }
    }

    pub const fn internal_server_error() -> Self {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Cow::Borrowed("Internal server error"),
        )
    }

    pub const fn not_found() -> Self {
        ApiError::new(StatusCode::NOT_FOUND, Cow::Borrowed("Not found"))
    }

    pub const fn unauthorized() -> Self {
        ApiError::new(StatusCode::UNAUTHORIZED, Cow::Borrowed("Unauthorized"))
    }

    pub const fn not_implemented() -> Self {
        ApiError::new(
            StatusCode::NOT_IMPLEMENTED,
            Cow::Borrowed("Not implemented"),
        )
    }

    pub fn bad_request(message: impl Into<Cow<'static, str>>) -> Self {
        ApiError::new(StatusCode::BAD_REQUEST, message.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}
