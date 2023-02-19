use image::{ImageBuffer, Rgba, RgbaImage};
use simple_simplex::NoiseConfig;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;
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
pub fn greet(name: &str) -> String {
    format!("Rust is saying hello to {}!", name)
}

fn get_canvas(
    canvas_id: &str,
) -> (
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context)
}

#[wasm_bindgen]
pub fn noise_fill(
    canvas_id: &str,
    seed: u64,
    octaves: Option<i32>,
    x_frequency: Option<f32>,
    y_frequency: Option<f32>,
    amplitude: Option<f32>,
    lacunarity: Option<f32>,
    gain: Option<f32>,
) {
    console_error_panic_hook::set_once();
    let (canvas, context) = get_canvas(canvas_id);
    let config: NoiseConfig = NoiseConfig::new(
        octaves.unwrap_or(3),        // Octaves
        x_frequency.unwrap_or(0.01), // X-Frequency
        y_frequency.unwrap_or(0.01), // Y-Frequency
        amplitude.unwrap_or(0.05),   // Amplitude
        lacunarity.unwrap_or(2.5),   // Lacunarity
        gain.unwrap_or(0.5),         // Gain
        (0.0, 255.0),                // range
        seed,                        // seed
    );

    let mut img: RgbaImage = ImageBuffer::new(canvas.width(), canvas.height());
    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            let noise_value = config.generate_raw_range(
                x as f32 / canvas.width() as f32 * 2.0,
                y as f32 / canvas.height() as f32 * 2.0,
            ) as u8;
            img.put_pixel(x, y, Rgba([noise_value, noise_value, noise_value, 255]));
        }
    }

    let clamped_buf: Clamped<&[u8]> = Clamped(img.as_raw());
    let image_data_temp =
        ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, img.width(), img.height())
            .unwrap();
    context.put_image_data(&image_data_temp, 0.0, 0.0).unwrap();
}
