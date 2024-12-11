use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PostgresConfig {
    pub host: String,
    pub user: String,
    pub database: String,
    pub password: String,
    pub max_connections: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MainConfig {
    pub log_level: String,
    pub jaeger: String,
    pub site_name: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthConfig {
    pub session_cleanup_interval_seconds: u64,
    pub session_timeout_seconds: i64,
    pub table_name: String,
}

#[derive(Deserialize, Debug)]
pub struct MeilisearchConfig {
    pub host: String,
    pub master_key: String,
}

#[derive(Deserialize, Debug)]
pub struct APISConfig {
    pub fetch_interval_hours: u64,
}
