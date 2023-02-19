use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Rust is saying hello to {}!", name)
}

#[wasm_bindgen]
pub fn draw_circle(canvas_id: &str) {
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

    context.set_fill_style(&"#000000".into());
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    context.begin_path();
    context
        .arc(
            canvas.width() as f64 / 2.0, 
            canvas.height() as f64 / 2.0, 
            std::cmp::min(canvas.width() / 2, canvas.height() / 2) as f64 * 0.7,
            0.0, 
            std::f64::consts::PI * 2.0
        ).unwrap();
    context.set_fill_style(&"#ffffff".into());
    context.fill();
}
