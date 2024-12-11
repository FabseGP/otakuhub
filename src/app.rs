use crate::pages::{about::*, login::*, root::*, search::*, signup::*};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <Stylesheet id="leptos" href="/pkg/otakuhub.css" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Welcome to OtakuHub" />
        <Router>
            <main>
                <Routes fallback=|| "Page not found".into_view()>
                    <Route path=StaticSegment("/") view=RootPage />
                    <Route path=StaticSegment("/about") view=AboutPage />
                    <Route path=StaticSegment("/search") view=SearchPage />
                    <Route path=StaticSegment("/login") view=LoginPage />
                    <Route path=StaticSegment("/signup") view=SignupPage />
                </Routes>
            </main>
        </Router>
    }
}
