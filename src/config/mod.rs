use cfg_if::cfg_if;

pub mod consts;
pub mod contexts;
pub mod settings;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub mod types;
    }
}
