use std::net::SocketAddr;

use rand::Rng;

#[derive(smart_default::SmartDefault, serde::Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    #[default(env_or_default("DATABASE_URL", None))]
    pub db_url: Option<String>,
    #[default(env_or_default("LOG_LEVEL", "info"))]
    pub level: String,
    #[default(None)]
    pub telemetry_bind: Option<SocketAddr>,
    #[default(vec![])]
    pub admin_twitch_ids: Vec<i32>,
    #[default(SocketAddr::from(([0, 0, 0, 0], 3000)))]
    pub http_bind: SocketAddr,
    pub twitch: TwitchConfig,
    #[default(random_secret())]
    pub jwt_secret: String,
    #[default(env_or_default("PUBLIC_API_URL", "https://onlyfangs.gay/api"))]
    pub api_url: String,
    #[default(env_or_default("PUBLIC_APP_URL", "https://onlyfangs.gay"))]
    pub app_url: String,
    #[default(env_or_default("VITE_DIST_DIR", "dist"))]
    pub vite_dist_dir: String,
}

#[derive(Default, serde::Deserialize, Debug)]
pub struct TwitchConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

fn random_secret() -> String {
    let mut rng = rand::thread_rng();
    // 32 bytes of random data
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    hex::encode(bytes)
}

fn env_or_default<T: From<String>>(key: &'static str, default: impl Into<T>) -> T {
    std::env::var(key).map(Into::into).unwrap_or_else(|_| default.into())
}

scuffle_settings::bootstrap!(Config);
