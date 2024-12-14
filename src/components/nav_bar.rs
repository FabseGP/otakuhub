use crate::{
    auth::auth::get_user,
    components::{search_bar::*, theme_selector::*},
};
use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    // let auth_status = Resource::new(|| (), |_| async move { get_user().await.ok().flatten() });

    view! {
        <div class="navbar bg-primary text-primary-content flex flex-col md:flex-row">
            <div class="navbar-start flex flex-col md:flex-row items-center">
                <a href="/" class="btn btn-ghost text-xl">
                    Otakuhub
                </a>
                <ThemeSelector />
                <SearchBar />
            </div>
            <div class="navbar-end flex flex-col md:flex-row">
                <a href="/about" class="btn btn-ghost">
                    About
                </a>
                <a href="/login" class="btn btn-ghost">
                    Login
                </a>
                <a href="/signup" class="btn btn-ghost">
                    Signup
                </a>
            </div>
        </div>
    }
}
