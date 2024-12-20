use crate::{
    components::{footer::Footer, nav_bar::NavBar},
    config::consts::TOASTS_TIMEOUT,
};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use std::time::Duration;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let params = use_params_map();
    let username = params.read().get("username").unwrap();
    let show_login_toast = RwSignal::new(true);
    set_timeout(
        move || show_login_toast.set(false),
        Duration::from_secs(TOASTS_TIMEOUT),
    );

    view! {
        <Title text=format!("{username}'s profile - OtakuHub") />
        <main>
            <div class="flex flex-col min-h-screen font-mono">
                <NavBar />
                {move || {
                    show_login_toast
                        .get()
                        .then(|| {
                            view! {
                                <div class="toast toast-end">
                                    <div class="alert alert-success">
                                        <span>
                                            Login successful! Welcome back, {username.clone()}!
                                        </span>
                                    </div>
                                </div>
                            }
                        })
                }}
                <div class="container py-8 px-4 mx-auto">
                    <div class="mb-8 text-center">
                        <h1 class="mb-4 text-4xl font-bold text-primary">"Welcome!"</h1>
                        <p class="mx-auto max-w-4xl text-xl text-primary">
                            Discover, track, and explore your favorite anime and manga with our next-generation platform.
                            Connect with fellow otaku and dive deep into the world of Japanese entertainment.
                        </p>
                    </div>
                </div>
                <Footer />
            </div>
        </main>
    }
}
