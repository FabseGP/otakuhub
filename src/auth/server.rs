use crate::auth::users::SessionUser;
use cfg_if::cfg_if;
use leptos::prelude::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::db::backend::PostgreSQLBackend;
        use axum_login::{AuthSession, AuthnBackend};
    }
}

#[server(GetUser, "/api", "Url", "get_user")]
pub async fn get_user() -> Result<Option<SessionUser>, ServerFnError> {
    let session: AuthSession<PostgreSQLBackend> = use_context().expect("session not provided");
    Ok(session.user)
}

#[server(LoginUser, "/api", "Url", "login")]
pub async fn login(
    username: String,
    password: String,
) -> Result<Option<SessionUser>, ServerFnError> {
    let mut auth: AuthSession<PostgreSQLBackend> = use_context().unwrap();
    let user = auth.backend.authenticate((username, password)).await?;

    if let Some(user) = user {
        auth.login(&user).await?;
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

#[server(LogoutUser, "/api", "Url", "logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    let mut auth: AuthSession<PostgreSQLBackend> = use_context().unwrap();
    auth.logout().await?;
    Ok(())
}

#[server(RegisterUser, "/api", "Url", "register")]
pub async fn register(
    username: String,
    password: String,
) -> Result<Option<SessionUser>, ServerFnError> {
    let mut auth_session: AuthSession<PostgreSQLBackend> =
        use_context().expect("auth-session not provided");
    let user = auth_session.backend.add_user(username, password).await?;

    if let Some(user) = user {
        auth_session.login(&user).await?;
        Ok(Some(user))
    } else {
        Ok(None)
    }
}
