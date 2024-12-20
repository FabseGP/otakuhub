#![feature(let_chains)]
#![feature(duration_constructors)]
#![recursion_limit = "256"]

pub mod apis;
pub mod app;
pub mod auth;
pub mod components;
pub mod config;
pub mod errors;
pub mod pages;
pub mod utils;

cfg_if::cfg_if! {
    if #[cfg(feature="ssr")] {
        pub mod db;
        pub mod routes;
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
