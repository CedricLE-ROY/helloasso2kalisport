use wasm_bindgen::prelude::*;

/// Entry point called by the WASM runtime
#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}

mod app;
pub mod pages;
pub mod routes;
