use crate::components::{footer::Footer, nav_bar::NavBar};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Home - OtakuHub" />
        <main>
            <div class="flex flex-col min-h-screen font-mono">
                <NavBar />
                <div class="container flex-grow py-8 px-4 mx-auto">
                    <div class="mb-8 text-center">
                        <h1 class="mb-4 text-4xl font-bold text-primary">Welcome to OtakuHub</h1>
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
