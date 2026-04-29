use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    veriffyi::app::run_app();
}
