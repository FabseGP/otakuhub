use crate::{
    components::{footer::*, nav_bar::*},
    utils::search::*,
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_query_map;

#[component]
pub fn SearchPage() -> impl IntoView {
    let query = use_query_map();
    let search_term = move || query.read().get("q").unwrap();
    let search_results = Resource::new(
        move || search_term(),
        |term| async move { search_engine(term).await.unwrap() },
    );
    view! {
        <Title text="Otakuhub: Next-generation animanga platform!" />
        <main>
            <div class="flex flex-col min-h-screen font-mono">
                <NavBar />
                <div class="container py-8 px-4 mx-auto">
                    <div class="mb-8 text-center">
                        <h1 class="mb-4 text-4xl font-bold text-primary">Search Results</h1>
                        {move || {
                            view! {
                                <p class="text-xl text-primary">
                                    "Showing results for: " {search_term()}
                                </p>
                            }
                        }}
                    </div>
                    <Suspense fallback=|| {
                        view! {
                            <div class="flex justify-center">
                                <div class="grid grid-cols-2 gap-4 w-full sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 max-w-[1200px]">
                                    {(0..25)
                                        .map(|_| {
                                            view! {
                                                <div class="flex flex-col items-center w-full">
                                                    <div class="flex flex-col items-center w-full max-w-xs">
                                                        <div class="mb-2 w-full bg-gray-200 rounded-lg animate-pulse h-[300px]"></div>
                                                        <div class="w-3/4 h-4 bg-gray-200 rounded animate-pulse"></div>
                                                    </div>
                                                </div>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            </div>
                        }
                    }>
                        {move || {
                            search_results
                                .get()
                                .map(|results| {
                                    view! {
                                        <div class="flex justify-center">
                                            <div class="grid grid-cols-2 gap-4 w-full sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 max-w-[1200px]">
                                                {results
                                                    .into_iter()
                                                    .map(|result| {
                                                        view! {
                                                            <div class="flex flex-col justify-center items-center">
                                                                <div class="flex flex-col items-center w-full max-w-xs">
                                                                    <img
                                                                        src=result.images.webp.image_url.clone()
                                                                        alt=format!(
                                                                            "Anime: {}",
                                                                            result.titles.first().unwrap().title,
                                                                        )
                                                                        class="object-cover mb-2 w-full rounded-lg h-[300px]"
                                                                    />
                                                                    <p class="text-sm text-center line-clamp-2">
                                                                        {result.titles.first().unwrap().title.clone()}
                                                                    </p>
                                                                </div>
                                                            </div>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()}
                                            </div>
                                        </div>
                                    }
                                })
                        }}
                    </Suspense>
                </div>
                <Footer />
            </div>
        </main>
    }
}
