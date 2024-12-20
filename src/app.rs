use crate::{
    config::contexts::UserGlobalState,
    pages::{
        home::HomePage, login::LoginPage, profile::ProfilePage, root::RootPage, search::SearchPage,
        settings::SettingsPage, signup::SignupPage,
    },
};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};
use reactive_stores::Store;
use thaw::ssr::SSRMountStyleProvider;

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                    <MetaTags />
                    <Stylesheet id="leptos" href="/pkg/otakuhub.css" />
                    <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                </head>
                <body>
                    <App />
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(Store::new(UserGlobalState::default()));

    view! {
        <Title text="Welcome to OtakuHub" />
        <Router>
            <main>
                <Routes fallback=|| "Page not found".into_view()>
                    <Route path=StaticSegment("/") view=RootPage />
                    <Route path=path!("/search/:category") view=SearchPage />
                    <Route path=StaticSegment("/login") view=LoginPage />
                    <Route path=StaticSegment("/signup") view=SignupPage />
                    <Route path=StaticSegment("/home") view=HomePage />
                    <Route path=path!("/user/:username") view=ProfilePage />
                    <Route path=StaticSegment("/settings") view=SettingsPage />
                </Routes>
            </main>
        </Router>
    }
}
