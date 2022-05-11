mod quiz;
mod utils;

use quiz::Quiz;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, odyssey!");
}

#[wasm_bindgen]
pub fn create_quiz(json_input: String) -> Option<Quiz> {
    console_error_panic_hook::set_once();
    Quiz::new(json_input)
}
