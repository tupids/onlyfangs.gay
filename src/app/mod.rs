use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use axum::Router;
use error::ApiError;
use scuffle_context::ContextFutExt;
use scuffle_http::backend::HttpServer;
use tower_http::services::{ServeDir, ServeFile};

use crate::global::Global;

mod application;
mod applications;
mod auth;
mod error;
mod login;

fn api_routes(global: Arc<Global>) -> Router {
    Router::new()
        .nest("/login", login::routes())
        .nest("/applications", applications::routes())
        .nest("/application", application::routes())
        .with_state(global)
        .fallback(not_found)
}

fn routes(global: Arc<Global>) -> Router {
    let dist_dir = PathBuf::from(&global.config.vite_dist_dir);

    Router::new().nest("/api", api_routes(global.clone())).nest_service(
        "/",
        ServeDir::new(&dist_dir)
            .not_found_service(ServeFile::new(dist_dir.join("fallback.html")))
            .precompressed_br()
            .precompressed_gzip()
            .append_index_html_on_directories(true),
    )
}

#[derive(Debug, serde::Serialize)]
pub struct Response<R> {
    pub status: u16,
    pub data: R,
}

async fn not_found() -> impl axum::response::IntoResponse {
    ApiError::not_found()
}

pub async fn svc(global: Arc<Global>, ctx: scuffle_context::Context) -> anyhow::Result<()> {
    let app = routes(global.clone());

    let server = scuffle_http::backend::tcp::TcpServerConfig::builder()
        .with_bind(global.config.http_bind)
        .build()
        .into_server();

    tracing::info!("starting server on {}", global.config.http_bind);

    server
        .start(scuffle_http::svc::axum_service(app), 1)
        .await
        .context("start server")?;

    tracing::info!("server started");

    server.wait().with_context(&ctx).await.transpose().context("server wait")?;

    tracing::info!("shutting down server");

    server.shutdown().await.context("server shutdown")?;

    tracing::info!("server shutdown complete");

    Ok(())
}
