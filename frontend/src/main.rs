fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<frontend::App>::new().render();
}

mod frontend;
