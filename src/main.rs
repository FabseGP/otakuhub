#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::{fs::read_to_string, process::exit};

    use anyhow::Context as _;
    use axum::{Router, serve};
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes as _, file_and_error_handler, generate_route_list};
    use opentelemetry::{KeyValue, global::set_tracer_provider, trace::TracerProvider as _};
    use opentelemetry_otlp::{SpanExporter, WithExportConfig as _};
    use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
    use otakuhub::{
        app::{App, shell},
        config::settings::MainConfig,
    };
    use tokio::net::TcpListener;
    use toml::{Table, Value};
    use tracing::{Level, error, info, level_filters::LevelFilter};
    use tracing_opentelemetry::layer;
    use tracing_subscriber::{
        Registry, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _,
    };

    let config_toml: Table = read_to_string("config.toml")?
        .parse()
        .context("config.toml not found")?;

    let main_config: MainConfig = if let Some(table) = config_toml.get("Main")
        && let Ok(value) = Value::try_into(table.clone())
    {
        value
    } else {
        error!("Failed to parse Main-table in Config.toml to MainConfig-struct");
        exit(1)
    };

    let log_level = match main_config.log_level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(
            SpanExporter::builder()
                .with_tonic()
                .with_endpoint(&main_config.jaeger)
                .build()?,
        )
        .with_resource(
            Resource::builder()
                .with_attribute(KeyValue::new("service.name", main_config.site_name.clone()))
                .build(),
        )
        .build();

    set_tracer_provider(provider.clone());

    Registry::default()
        .with(LevelFilter::from_level(log_level))
        .with(fmt::layer())
        .with(layer().with_tracer(provider.tracer(main_config.site_name.clone())))
        .init();

    let conf = match get_configuration(None) {
        Ok(conf) => conf,
        Err(err) => {
            error!("Failed to get Leptos conf from Cargo.toml: {:?}", &err);
            exit(1)
        }
    };
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(file_and_error_handler(shell))
        .with_state(leptos_options);

    info!("listening on http://{addr}");
    let listener = match TcpListener::bind(&addr).await {
        Ok(bind) => bind,
        Err(err) => {
            error!("Failed to bind address: {:?}", &err);
            exit(1);
        }
    };

    if let Err(err) = serve(listener, app.into_make_service()).await {
        error!("Failed to serve application: {:?}", &err);
    } else {
        info!("Successfully served application");
    }

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
