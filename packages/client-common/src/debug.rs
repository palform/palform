use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn enable_debug_hook_js() {
    #[cfg(feature = "debug")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
}
