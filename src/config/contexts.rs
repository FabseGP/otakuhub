use crate::{auth::users::SessionUser, components::theme_selector::Theme};

use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Clone, Store, Serialize, Deserialize, Default)]
pub struct UserGlobalState {
    pub user: Option<SessionUser>,
    pub is_authenticated: bool,
    pub theme_preference: Option<Theme>,
}
