use leptos::{ev::*, prelude::*};
use leptos_router::{NavigateOptions, hooks::use_navigate};
use urlencoding::encode;

#[component]
pub fn SearchBar() -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let navigate = use_navigate();

    let handle_search_query = move |query: String| {
        let encoded_query = encode(&query);
        navigate(
            &format!("/search?q={encoded_query}"),
            NavigateOptions::default(),
        );
    };

    view! {
        <div class="p-2 w-full md:w-auto">
            <input
                type="text"
                placeholder="Search animanga..."
                class="input input-sm md:input-md input-bordered w-full text-base-content"
                prop:value=search_query
                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                on:keypress=move |ev: KeyboardEvent| {
                    if ev.key() == "Enter" {
                        let query = search_query.get();
                        if !query.is_empty() {
                            handle_search_query(query);
                        }
                    }
                }
            />
        </div>
    }
}
