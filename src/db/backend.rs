use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::{auth::users::{SessionUser, SqlUser}, config::consts::*};
        use axum_login::{AuthnBackend, UserId};
        use sqlx::{Pool, Postgres, migrate, query, query_as, query_scalar};
        use async_trait::async_trait;
        use argon2::{
            password_hash::{
                rand_core::OsRng,
                PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
                Encoding::B64,
            },
            Argon2
        };

    }
}
use crate::errors::error_template::*;

#[derive(Clone)]
pub struct PostgreSQLBackend {
    pub pool: Pool<Postgres>,
}

impl PostgreSQLBackend {
    #[must_use]
    pub const fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    pub async fn migrate(&self) -> Result<(), AppError> {
        migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| AppError::InternalError(format!("In migrations: {e}")))
    }

    pub async fn add_user(
        &self,
        username: String,
        password: String,
    ) -> Result<Option<SessionUser>, AppError> {
        if !(USERNAME_LENGTH_MINIMUM..=USERNAME_LENGTH_MAXIMUM).contains(&username.len())
            || !(PASSWORD_LENGTH_MINIMUM..=PASSWORD_LENGTH_MAXIMUM).contains(&password.len())
        {
            return Err(AppError::InvalidData(format!(
                "Username must be {USERNAME_LENGTH_MINIMUM}-{USERNAME_LENGTH_MAXIMUM} characters and password {PASSWORD_LENGTH_MINIMUM}-{PASSWORD_LENGTH_MAXIMUM} characters"
            )));
        }
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let pass_hash: PasswordHash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(format!("Password hashing error: {e}")))?;

        let new_id = query!(
            "INSERT INTO users (username, pass_hash) VALUES ($1, $2) RETURNING id",
            username,
            pass_hash.to_string(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Error inserting user: {e}")))?
        .id;

        Ok(Some(SessionUser {
            id: new_id,
            username,
            session_auth_hash: pass_hash.hash.unwrap().as_bytes().to_owned(),
        }))
    }

    pub async fn user_exist(&self, username: String) -> Result<bool, AppError> {
        let username_record = query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Error checking existence of user: {e}")))?;

        Ok(username_record.unwrap_or(false))
    }
}

#[async_trait]
impl AuthnBackend for PostgreSQLBackend {
    type User = SessionUser;
    type Credentials = (String, String);
    type Error = AppError;

    async fn authenticate(
        &self,
        (username, password): Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<SqlUser> =
            query_as!(SqlUser, "SELECT * FROM users WHERE username = $1", username)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::InternalError(format!("Fetch user: {e}")))?;
        if let Some(user) = user {
            let hash = PasswordHash::parse(user.pass_hash.as_ref(), B64)
                .map_err(|e| AppError::InternalError(format!("Corrupted password hash: {e}")))?;
            let hasher = Argon2::default();
            if hasher.verify_password(password.as_bytes(), &hash).is_ok() {
                return Ok(Some(user.to_user()?));
            }
        }
        Ok(None)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<SqlUser> =
            query_as!(SqlUser, "SELECT * FROM users WHERE id = $1", user_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| AppError::InternalError(format!("Fetch user: {e}")))?;
        if let Some(user) = user {
            Ok(Some(user.to_user()?))
        } else {
            Ok(None)
        }
    }
}
