mod picture_gen;
use image::ImageBuffer;
use picture_gen::{get_canvas, put_image_to_canvas, create};
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
    let (canvas, context) = get_canvas(canvas_id);
    let (width, height) = (canvas.width(), canvas.height());
    let mut img = ImageBuffer::new(width, height);
    create(&mut img, seed);
    put_image_to_canvas(&img, context);
}
