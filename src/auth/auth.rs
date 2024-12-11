use crate::auth::users::User;
use cfg_if::cfg_if;
use leptos::prelude::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::db::backend::PostgreSQLBackend;
        use axum_login::{AuthSession, AuthnBackend};
    }
}

#[server(GetUser, "/api", "Url", "get_user")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let session: AuthSession<PostgreSQLBackend> = use_context().expect("session not provided");
    Ok(session.user)
}

#[server(Login, "/api", "Url", "login")]
pub async fn login(username: String, password: String) -> Result<Option<User>, ServerFnError> {
    let mut auth: AuthSession<PostgreSQLBackend> = use_context().unwrap();
    let user = auth.backend.authenticate((username, password)).await?;

    if let Some(user) = user.as_ref() {
        auth.login(user).await?;
        Ok(Some(user.clone()))
    } else {
        Ok(None)
    }
}

#[server(Register, "/api", "Url", "register")]
pub async fn register(username: String, password: String) -> Result<Option<User>, ServerFnError> {
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
