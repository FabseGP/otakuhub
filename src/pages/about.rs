use crate::components::nav_bar::*;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About OtakuHub" />
        <main>
            <div class="font-mono flex flex-col min-h-screen">
                <NavBar />
                <div class="container mx-auto px-4 py-8">
                    <div class="text-center mb-8">
                        <h1 class="text-4xl font-bold text-primary mb-4">About OtakuHub</h1>
                        <p class="text-xl text-primary max-w-4xl mx-auto">
                            Discover, track, and explore your favorite anime and manga with our next-generation platform.
                            Connect with fellow otaku and dive deep into the world of Japanese entertainment.
                        </p>
                    </div>
                </div>
            </div>
        </main>
    }
}
