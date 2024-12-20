use crate::config::contexts::UserGlobalState;
use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Store)]
pub enum Theme {
    Acid,
    Aqua,
    Autumn,
    Black,
    Bumblebee,
    Business,
    Coffee,
    Corporate,
    Cupcake,
    Cyberpunk,
    Cmyk,
    Dark,
    Dim,
    Dracula,
    Emerald,
    Fantasy,
    Forest,
    Garden,
    Halloween,
    Lemonade,
    Light,
    Lofi,
    Luxury,
    Night,
    Nord,
    Pastel,
    Retro,
    Sunset,
    Synthwave,
    Valentine,
    Water,
    Wireframe,
    Winter,
}

impl Theme {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Acid => "acid",
            Self::Aqua => "aqua",
            Self::Autumn => "autumn",
            Self::Black => "black",
            Self::Bumblebee => "bumblebee",
            Self::Business => "business",
            Self::Coffee => "coffee",
            Self::Corporate => "corporate",
            Self::Cupcake => "cupcake",
            Self::Cyberpunk => "cyberpunk",
            Self::Cmyk => "cmyk",
            Self::Dark => "dark",
            Self::Dim => "dim",
            Self::Dracula => "dracula",
            Self::Emerald => "emerald",
            Self::Fantasy => "fantasy",
            Self::Forest => "forest",
            Self::Garden => "garden",
            Self::Halloween => "halloween",
            Self::Lemonade => "lemonade",
            Self::Light => "light",
            Self::Lofi => "lofi",
            Self::Luxury => "luxury",
            Self::Night => "night",
            Self::Nord => "nord",
            Self::Pastel => "pastel",
            Self::Retro => "retro",
            Self::Sunset => "sunset",
            Self::Synthwave => "synthwave",
            Self::Valentine => "valentine",
            Self::Water => "water",
            Self::Wireframe => "wireframe",
            Self::Winter => "winter",
        }
    }

    const fn display_name(self) -> &'static str {
        match self {
            Self::Acid => "Acid",
            Self::Aqua => "Aqua",
            Self::Autumn => "Autumn",
            Self::Black => "Black",
            Self::Bumblebee => "Bumblebee",
            Self::Business => "Business",
            Self::Coffee => "Coffee",
            Self::Corporate => "Corporate",
            Self::Cupcake => "Cupcake",
            Self::Cyberpunk => "Cyberpunk",
            Self::Cmyk => "CMYK",
            Self::Dark => "Dark",
            Self::Dim => "Dim",
            Self::Dracula => "Dracula",
            Self::Emerald => "Emerald",
            Self::Fantasy => "Fantasy",
            Self::Forest => "Forest",
            Self::Garden => "Garden",
            Self::Halloween => "Halloween",
            Self::Lemonade => "Lemonade",
            Self::Light => "Light",
            Self::Lofi => "Lo-Fi",
            Self::Luxury => "Luxury",
            Self::Night => "Night",
            Self::Nord => "Nord",
            Self::Pastel => "Pastel",
            Self::Retro => "Retro",
            Self::Sunset => "Sunset",
            Self::Synthwave => "Synthwave",
            Self::Valentine => "Valentine",
            Self::Water => "Water",
            Self::Wireframe => "Wireframe",
            Self::Winter => "Winter",
        }
    }

    fn all_themes() -> Vec<Self> {
        vec![
            Self::Acid,
            Self::Aqua,
            Self::Autumn,
            Self::Black,
            Self::Bumblebee,
            Self::Business,
            Self::Coffee,
            Self::Corporate,
            Self::Cupcake,
            Self::Cyberpunk,
            Self::Cmyk,
            Self::Dark,
            Self::Dim,
            Self::Dracula,
            Self::Emerald,
            Self::Fantasy,
            Self::Forest,
            Self::Garden,
            Self::Halloween,
            Self::Lemonade,
            Self::Light,
            Self::Lofi,
            Self::Luxury,
            Self::Night,
            Self::Nord,
            Self::Pastel,
            Self::Retro,
            Self::Sunset,
            Self::Synthwave,
            Self::Valentine,
            Self::Water,
            Self::Wireframe,
            Self::Winter,
        ]
    }
}

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let user_state = expect_context::<Store<UserGlobalState>>();

    let (theme, set_theme) = signal(user_state.get().theme_preference.unwrap_or(Theme::Night));
    let (is_open, set_is_open) = signal(false);
    let (search_query, set_search_query) = signal(String::new());

    let filtered_themes = move || {
        let query = search_query.get();
        Theme::all_themes()
            .into_iter()
            .filter(|t| t.as_str().contains(&query))
            .collect::<Vec<_>>()
    };

    let change_theme = move |new_theme: Theme| {
        if let Some(document) = document().document_element() {
            document
                .set_attribute("data-theme", new_theme.as_str())
                .unwrap();
        }
        set_theme.set(new_theme);
        user_state.update(|state| {
            state.theme_preference = Some(new_theme);
        });
        set_is_open.set(false);
    };

    view! {
        <div class="dropdown dropdown-end">
            <label
                tabindex="0"
                class="m-1 btn btn-sm md:btn-md"
                on:click=move |_| set_is_open.set(!is_open.get())
                on:touchstart=move |_| set_is_open.set(!is_open.get())
            >
                <svg
                    class="inline-block w-4 h-4 stroke-current md:w-5 md:h-5"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
                </svg>
                <span class="hidden ml-1 md:inline">{move || theme.get().display_name()}</span>
            </label>
            <div class="overflow-y-auto relative left-0 z-50 p-2 mt-2 w-64 max-h-96 shadow transform dropdown-content menu bg-base-200 rounded-box text-base-content">
                <div class="sticky top-0 p-2 bg-base-200">
                    <input
                        type="text"
                        placeholder="Search themes..."
                        class="w-full input input-sm input-bordered text-base-content"
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    />
                </div>
                {move || {
                    filtered_themes()
                        .into_iter()
                        .map(|theme| {
                            view! {
                                <button
                                    class="flex gap-2 items-center py-2 px-4 w-full rounded-lg hover:bg-base-300"
                                    on:click=move |_| change_theme(theme)
                                >
                                    <span>{theme.display_name()}</span>
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
