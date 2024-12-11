use crate::{components::nav_bar::*, utils::search::*};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_query_map;

#[component]
pub fn SearchPage() -> impl IntoView {
    let query = use_query_map();
    let search_term = move || query.get().get("q").unwrap_or_default();
    let search_results = Resource::new(
        move || search_term(),
        |term| async move { search_engine(term).await.unwrap_or_default() },
    );
    view! {
        <Title text="Otakuhub: Next-generation animanga platform!" />
        <main>
            <div class="font-mono flex flex-col min-h-screen">
                <NavBar />
                <div class="container mx-auto px-4 py-8">
                    <div class="text-center mb-8">
                        <h1 class="text-4xl font-bold text-primary mb-4">Search Results</h1>
                        {move || {
                            view! {
                                <p class="text-xl text-primary">
                                    "Showing results for: " {search_term()}
                                </p>
                            }
                        }}
                    </div>
                    <Suspense fallback=|| view! { <p class="text-center">"Loading search results..."</p> }>
                        {move || search_results.get().map(|results| {
                            view! {
                                <div class="flex justify-center">
                                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4 max-w-[1200px] w-full">
                                        {results.into_iter().map(|result| {
                                            view! {
                                                <div class="flex flex-col items-center justify-center">
                                                    <div class="w-full max-w-xs flex flex-col items-center">
                                                        <img
                                                            src=result.images.webp.image_url.clone()
                                                            alt=format!("Anime: {}", result.titles.first().unwrap().title)
                                                            class="w-full h-[300px] object-cover rounded-lg mb-2"
                                                        />
                                                        <p class="text-center text-sm line-clamp-2">
                                                            {result.titles.first().unwrap().title.clone()}
                                                        </p>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        })}
                    </Suspense>
                </div>
            </div>
        </main>
    }
}
