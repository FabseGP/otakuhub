use crate::auth::users::User;
use leptos::prelude::*;

#[server]
pub async fn require_login(mut next: Option<String>) -> Result<Option<User>, ServerFnError> {
    use leptos_router::{
        NavigateOptions,
        hooks::{use_location, use_navigate},
        location::Location,
    };
    use urlencoding::encode;
    let mut next = next;
    let return_to = next.take().map_or_else(
        || {
            let Location {
                pathname,
                search,
                hash,
                ..
            } = use_location();
            format!(
                "{}{}{}",
                pathname.get_untracked(),
                search.get_untracked(),
                hash.get_untracked()
            )
        },
        |s| s,
    );
    if let Some(user) = get_user().await? {
        return Ok(Some(user));
    }
    let nav = use_navigate();
    nav(
        &format!("/login?c={}", encode(&return_to)),
        NavigateOptions::default(),
    );
    Ok(None)
}

#[server(GetUser, "/api", "Url", "get_user")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::db::backend::PostgreSQLBackend;
    use axum_login::AuthSession;
    let session: AuthSession<PostgreSQLBackend> = use_context().expect("session not provided");
    Ok(session.user)
}

#[server(LoginUser, "/api", "Url", "login")]
pub async fn login_user(username: String, password: String) -> Result<Option<User>, ServerFnError> {
    use crate::db::backend::PostgreSQLBackend;
    use axum_login::{AuthSession, AuthnBackend};
    let mut auth: AuthSession<PostgreSQLBackend> = use_context().unwrap();
    let user = auth.backend.authenticate((username, password)).await?;

    if let Some(user) = user.as_ref() {
        auth.login(user).await?;
        Ok(Some(user.clone()))
    } else {
        Ok(None)
    }
}

#[server(RegisterNewUser, "/api", "Url", "register")]
pub async fn register_new_user(
    username: String,
    password: String,
) -> Result<Option<User>, ServerFnError> {
    use crate::db::backend::PostgreSQLBackend;
    use axum_login::AuthSession;
    let auth_session: AuthSession<PostgreSQLBackend> =
        use_context().expect("auth-session not provided");
    let user = auth_session
        .backend
        .add_user(username, password)
        .await
        .unwrap();
    user.map_or(Ok(None), |user| Ok(Some(user)))
}
