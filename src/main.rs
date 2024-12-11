#![feature(let_chains)]
#![recursion_limit = "256"]

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        Router,
        routing::{get, post},
        serve,
    };
    use axum_login::AuthManagerLayerBuilder;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, file_and_error_handler, generate_route_list};
    use meilisearch_sdk::client::Client as MClient;
    use opentelemetry::{KeyValue, global::set_tracer_provider, trace::TracerProvider as _};
    use opentelemetry_otlp::{SpanExporter, WithExportConfig as _};
    use opentelemetry_sdk::{Resource, runtime::Tokio, trace::TracerProvider};
    use otakuhub::{
        apis::tracker::*,
        app::{App, shell},
        config::{
            settings::{APISConfig, AuthConfig, MainConfig, MeilisearchConfig, PostgresConfig},
            types::{AppState, MEILISEARCH_CLIENT, UTILS_CONFIG, UtilsConfig},
        },
        db::backend::PostgreSQLBackend,
        routes::{leptos_routes_handler, server_func_handler},
    };
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    use std::{fs::read_to_string, sync::Arc};
    use time::Duration;
    use tokio::{net::TcpListener, task::spawn, time::Duration as TDuration};
    use toml::{Table, Value};
    use tower::ServiceBuilder;
    use tower_sessions::{Expiry, SessionManagerLayer, session_store::ExpiredDeletion};
    use tower_sessions_sqlx_store::PostgresStore;
    use tracing::{Level, info};
    use tracing_opentelemetry::layer;
    use tracing_subscriber::{
        Registry, filter::LevelFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt as _,
    };

    let config_toml: Table = read_to_string("config.toml")
        .unwrap()
        .parse()
        .expect("config.toml in project root-folder");
    let main_config: MainConfig = Value::try_into(config_toml["Main"].clone()).unwrap();
    let postgres_config: PostgresConfig =
        Value::try_into(config_toml["PostgreSQL"].clone()).unwrap();
    let auth_config: AuthConfig = Value::try_into(config_toml["Auth"].clone()).unwrap();
    let meilisearch_config: MeilisearchConfig =
        Value::try_into(config_toml["Meilisearch"].clone()).unwrap();
    let apis_config: APISConfig = Value::try_into(config_toml["APIS"].clone()).unwrap();

    let pool_options = PgConnectOptions::new()
        .host(&postgres_config.host)
        .port(postgres_config.port)
        .username(&postgres_config.user)
        .database(&postgres_config.database)
        .password(&postgres_config.password);
    let pool = PgPoolOptions::default()
        .max_connections(postgres_config.max_connections)
        .connect_with(pool_options)
        .await
        .expect("postgresql-database accessible with the given auths");

    let session_store = PostgresStore::new(pool.clone())
        .with_table_name(auth_config.table_name)
        .unwrap();
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store.clone())
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(
            auth_config.session_timeout_seconds,
        )));

    spawn(
        session_store
            .clone()
            .continuously_delete_expired(TDuration::from_secs(
                auth_config.session_cleanup_interval_seconds,
            )),
    );

    UTILS_CONFIG
        .set(Arc::new(UtilsConfig {
            main: main_config.clone(),
        }))
        .unwrap();

    MEILISEARCH_CLIENT
        .set(Arc::new(
            MClient::new(meilisearch_config.host, Some(meilisearch_config.master_key)).unwrap(),
        ))
        .unwrap();

    let log_level = match main_config.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let new_exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(&main_config.jaeger)
        .build()
        .unwrap();

    let provider = TracerProvider::builder()
        .with_batch_exporter(new_exporter, Tokio)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            main_config.site_name.clone(),
        )]))
        .build();

    set_tracer_provider(provider.clone());

    Registry::default()
        .with(LevelFilter::from_level(log_level))
        .with(fmt::layer())
        .with(layer().with_tracer(provider.tracer(main_config.site_name)))
        .init();

    let backend = PostgreSQLBackend::new(pool.clone());
    let auth_session_layer =
        ServiceBuilder::new().layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

    start_api_scraping_tasks(apis_config.fetch_interval_hours)
        .await
        .unwrap();

    let conf = get_configuration(None).expect("config set in Cargo.toml");
    let addr = conf.leptos_options.site_addr;

    let routes = generate_route_list(App);

    let app_state = AppState {
        pool,
        leptos_options: conf.leptos_options,
        routes: routes.clone(),
    };

    let app = Router::new()
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .route("/api/*fn_name", post(server_func_handler))
        .fallback(file_and_error_handler::<AppState, _>(shell))
        .layer(auth_session_layer)
        .with_state(app_state);

    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Listening on http://{}", &addr);
    serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
