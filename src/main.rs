use std::sync::Arc;

use anyhow::Context;
use database::schema::health_check;
use diesel::query_dsl::methods::FindDsl;
use diesel::ExpressionMethods;
use diesel_async::{AsyncConnection, RunQueryDsl};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

mod app;
mod config;
mod database;
mod global;
mod migrations;

impl scuffle_bootstrap::Global for global::Global {
    type Config = config::Config;

    async fn init(config: Self::Config) -> anyhow::Result<Arc<Self>> {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive(config.level.parse()?)),
            )
            .init();

        tracing::info!("starting server.");

        let Some(db_url) = config.db_url.as_deref() else {
            anyhow::bail!("DATABASE_URL is not set");
        };

        tracing::info!("running migrations");

        tokio::time::timeout(std::time::Duration::from_secs(10), run_migrations(db_url))
            .await
            .context("migrations timed out")?
            .context("migrations failed")?;

        tracing::info!("migrations complete");

        let database = diesel_async::pooled_connection::bb8::Pool::builder()
            .build(diesel_async::pooled_connection::AsyncDieselConnectionManager::new(db_url))
            .await
            .context("build database pool")?;

        tracing::info!("database initialized");

        Ok(Arc::new(Self { config, database }))
    }
}

async fn run_migrations(url: &str) -> anyhow::Result<()> {
    let conn = diesel_async::pg::AsyncPgConnection::establish(url)
        .await
        .context("establish connection")?;

    migrations::run_migrations(conn).await.context("run migrations")?;

    Ok(())
}

impl scuffle_signal::SignalConfig for global::Global {
    async fn on_shutdown(self: &Arc<Self>) -> anyhow::Result<()> {
        tracing::info!("shutting down");
        Ok(())
    }
}

impl scuffle_bootstrap_telemetry::TelemetryConfig for global::Global {
    fn bind_address(&self) -> Option<std::net::SocketAddr> {
        self.config.telemetry_bind
    }

    async fn health_check(&self) -> Result<(), anyhow::Error> {
        let mut conn = self.database.get().await.context("get database connection")?;

        // Health check to see if the database is healthy and can be reached.
        // We do an update here because we want to make sure the database is
        // not just readable but also writable.
        diesel::update(health_check::dsl::health_check.find(1))
            .set(health_check::dsl::updated_at.eq(chrono::Utc::now()))
            .execute(&mut conn)
            .await
            .context("update health check")?;

        Ok(())
    }
}

scuffle_bootstrap::main! {
    global::Global {
        scuffle_signal::SignalSvc,
        scuffle_bootstrap_telemetry::TelemetrySvc,
        app::svc,
    }
}
