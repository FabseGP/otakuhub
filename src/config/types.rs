use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::config::settings::MainConfig;

        use axum::extract::FromRef;
        use fastrand::Rng;
        use leptos::prelude::*;
        use leptos_axum::AxumRouteListing;
        use meilisearch_sdk::client::Client as MClient;
        use once_cell::sync::{Lazy, OnceCell};
        use reqwest::Client as RClient;
        use sqlx::{Pool, Postgres};
        use std::sync::Arc;
        use tokio::sync::Mutex;

        #[derive(Clone, Debug, FromRef)]
        pub struct AppState {
            pub pool: Pool<Postgres>,
            pub leptos_options: LeptosOptions,
            pub routes: Vec<AxumRouteListing>,
        }

        #[derive(Debug)]
        pub struct UtilsConfig {
            pub main: MainConfig,
        }

        pub static UTILS_CONFIG: OnceCell<Arc<UtilsConfig>> = OnceCell::new();
        pub static HTTP_CLIENT: Lazy<RClient> = Lazy::new(RClient::new);
        pub static MEILISEARCH_CLIENT: OnceCell<Arc<MClient>> = OnceCell::new();
        pub static RNG: Lazy<Mutex<Rng>> = Lazy::new(|| Mutex::new(Rng::new()));
    }
}
