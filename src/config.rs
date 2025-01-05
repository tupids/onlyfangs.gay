use std::net::SocketAddr;

use rand::Rng;

#[derive(smart_default::SmartDefault, serde::Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    #[default(env_or_default("DATABASE_URL", None))]
    pub database_url: Option<String>,
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
    std::env::var(key)
        .map(Into::into)
        .unwrap_or_else(|_| default.into())
}

scuffle_settings::bootstrap!(Config);
