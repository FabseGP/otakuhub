use crate::{app::shell, config::types::AppState, db::backend::PostgreSQLBackend};

use axum::{
    body::Body,
    extract::State,
    http::Request,
    response::{IntoResponse, Response},
};
use axum_login::AuthSession;
use leptos::prelude::*;
use leptos_axum::{handle_server_fns_with_context, render_route_with_context};
use tower_sessions::Session;

pub async fn leptos_routes_handler(
    auth_session: AuthSession<PostgreSQLBackend>,
    state: State<AppState>,
    req: Request<Body>,
) -> Response {
    let State(app_state) = state.clone();
    let handler = render_route_with_context(
        app_state.routes.clone(),
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        },
        move || shell(app_state.leptos_options.clone()),
    );
    handler(state, req).await.into_response()
}

#[cfg(feature = "ssr")]
pub async fn server_func_handler(
    auth_session: AuthSession<PostgreSQLBackend>,
    session: Session,
    State(app_state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(auth_session.clone());
            provide_context(session.clone());
            provide_context(app_state.clone());
        },
        req,
    )
    .await
}
