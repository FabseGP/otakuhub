use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub session_auth_hash: Vec<u8>,
}

cfg_if! {
    if #[cfg(feature="ssr")] {
        use crate::errors::error_template::*;
        use argon2::password_hash::{PasswordHash, Encoding::B64};
        use axum_login::AuthUser;
        use sqlx::FromRow;

        #[derive(Clone, PartialEq, Eq, Debug, FromRow)]
        pub struct SqlUser {
            pub id: i64,
            pub username: String,
            pub pass_hash: String,
        }

        impl AuthUser for User {
            type Id = i64;
            fn id(&self) -> Self::Id {
                self.id
            }

            fn session_auth_hash(&self) -> &[u8] {
                self.session_auth_hash.as_ref()
            }
        }

        impl SqlUser {
            pub fn to_user(self) -> Result<User, AppError> {
                let PasswordHash { hash, .. } =
                    PasswordHash::parse(&self.pass_hash, B64)
                        .map_err(|e| AppError::InternalError(format!("Decode password: {e}")))?;
                let hash: Vec<u8> = hash
                    .map(|output| output.as_bytes().to_owned())
                    .ok_or_else(|| AppError::InternalError("Badly formatted password hash".into()))?;

                Ok(User {
                    id: self.id,
                    username: self.username,
                    session_auth_hash: hash,
                })
            }
        }
    }
}
