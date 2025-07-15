#[cfg(feature = "ssr")]
use serde::Deserialize;

#[cfg(feature = "ssr")]
#[derive(Deserialize, Clone)]
pub struct MainConfig {
    pub log_level: String,
    pub jaeger: String,
    pub site_name: String,
}
