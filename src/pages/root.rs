use crate::components::{footer::Footer, nav_bar::NavBar};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn RootPage() -> impl IntoView {
    view! {
        <Title text="Otakuhub: Next-generation animanga platform!" />
        <main>
            <div class="flex flex-col min-h-screen font-mono">
                <NavBar />
                <div class="min-h-screen hero bg-base-200">
                    <div class="flex-col lg:flex-row hero-content">
                        <img src="/images/frontpage.webp" class="max-w-sm rounded-lg shadow-2xl" />
                        <div>
                            <h1 class="text-5xl font-bold text-primary">Welcome to OtakuHub!</h1>
                            <p class="py-6 text-primary">
                                Discover, track, and explore your favorite anime and manga with our next-generation platform.
                                Connect with fellow otaku and dive deep into the world of Japanese entertainment.
                            </p>
                        </div>
                    </div>
                </div>
                <Footer />
            </div>
        </main>
    }
}
