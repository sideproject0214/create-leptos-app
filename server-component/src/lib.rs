mod error_template;
mod errors;
#[cfg(feature = "ssr")]
pub mod fallback;
pub mod pages;
pub mod functions;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::pages::todo::Fivemintwentiesfour;

    _ = console_log::init_with_level(log::Level::Error);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(Fivemintwentiesfour);
}
