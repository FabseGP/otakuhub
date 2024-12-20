use leptos::{ev::KeyboardEvent, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_navigate};
use urlencoding::encode;

#[component]
pub fn SearchBar() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let navigate = use_navigate();

    view! {
        <div class="p-2 w-full md:w-auto">
            <input
                type="text"
                placeholder="Search animanga..."
                class="w-full input input-sm input-bordered text-base-content text-primary md:input-md"
                prop:value=search_query
                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                on:keypress=move |ev: KeyboardEvent| {
                    if ev.key() == "Enter" {
                        let query = move || search_query.get();
                        if !query().is_empty() {
                            navigate(
                                &format!("/search/anime?q={}", encode(&query())),
                                NavigateOptions::default(),
                            );
                        }
                    }
                }
            />
        </div>
    }
}
