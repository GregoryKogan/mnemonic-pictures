mod picture_gen;
use std::hash::Hasher;
use picture_gen::MnemonincGenerator;
use siphasher::sip128::SipHasher;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn init_console_errors() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Rust is saying hello to {}!", name)
}

#[wasm_bindgen]
pub fn generate(canvas_id: &str, seed: u64) {
    let mut generator = MnemonincGenerator::new(canvas_id, seed);
    generator.generate();
}

#[wasm_bindgen]
pub fn generate_from_string(canvas_id: &str, seed_string: &str) {
    let mut hasher = SipHasher::new();
    hasher.write(seed_string.as_bytes());
    let seed = hasher.finish();
    let mut generator = MnemonincGenerator::new(canvas_id, seed);
    generator.generate();
}
