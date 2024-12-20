use crate::{
    auth::server::{get_user, logout},
    components::{search_bar::SearchBar, theme_selector::ThemeSelector},
};
use leptos::{either::Either, prelude::*};
use leptos_router::{NavigateOptions, components::A, hooks::use_navigate};

#[component]
pub fn NavBar() -> impl IntoView {
    let auth_status = Resource::new(|| (), |()| async move { get_user().await.ok().flatten() });
    let dropdown_open = RwSignal::new(false);
    let logout_action = Action::new(|()| async move { logout().await.ok() });

    Effect::new(move |_| {
        if logout_action.value().get().is_some() {
            let navigate = use_navigate();
            navigate("/home", NavigateOptions::default());
        }
    });

    view! {
        <div class="flex flex-col md:flex-row navbar bg-primary text-primary-content">
            <div class="flex flex-col items-center md:flex-row navbar-start">
                <Suspense fallback=move || {
                    view! {
                        <a href="/" class="text-xl btn btn-ghost">
                            OtakuHub
                        </a>
                    }
                }>
                    <a
                        href=move || {
                            if auth_status.get().flatten().is_some() { "/home" } else { "/" }
                        }
                        class="text-xl btn btn-ghost"
                    >
                        Otakuhub
                    </a>
                </Suspense>
                <ThemeSelector />
                <SearchBar />
            </div>
            <div class="flex flex-col md:flex-row navbar-end">
                <Suspense fallback=move || {
                    view! {
                        <a href="/login" class="btn btn-ghost">
                            Login
                        </a>
                    }
                }>
                    {move || {
                        if auth_status.get().flatten().is_some() {
                            Some(
                                Either::Left(
                                    view! {
                                        <div class="dropdown dropdown-end">
                                            <div
                                                tabindex="0"
                                                role="button"
                                                class="cursor-pointer avatar"
                                                on:click=move |_| dropdown_open.set(!dropdown_open.get())
                                            >
                                                <div class="w-10 rounded-full">
                                                    <img src="/images/default_avatar.webp" alt="User avatar" />
                                                </div>
                                            </div>
                                            <ul
                                                tabindex="0"
                                                class=move || {
                                                    format!(
                                                        "dropdown-content relative overflow-y-auto mt-2 text-base-content transform menu p-2 shadow bg-base-100 rounded-box z-50 w-52 text-primary {}",
                                                        if dropdown_open.get() { "block" } else { "hidden" },
                                                    )
                                                }
                                            >
                                                <li>
                                                    <A
                                                        href="/settings"
                                                        on:click=move |_| dropdown_open.set(false)
                                                    >
                                                        Settings
                                                    </A>
                                                </li>
                                                <li>
                                                    <button on:click=move |_| {
                                                        dropdown_open.set(false);
                                                        logout_action.dispatch(());
                                                    }>Logout</button>
                                                </li>
                                            </ul>
                                        </div>
                                    },
                                ),
                            )
                        } else {
                            Some(
                                Either::Right(
                                    view! {
                                        <a href="/login" class="btn btn-ghost">
                                            Login
                                        </a>
                                        <a href="/signup" class="btn btn-ghost">
                                            Signup
                                        </a>
                                    },
                                ),
                            )
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}
