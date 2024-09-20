use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

fn draw_diamond(context: &web_sys::CanvasRenderingContext2d) {
    context.set_line_width(5.0);
    context.begin_path();
    context.move_to(300.0, 50.0);
    context.line_to(50.0, 300.0);
    context.line_to(300.0, 550.0);
    context.line_to(550.0, 300.0);
    context.line_to(300.0, 50.0);
    context.close_path();
    context.stroke();
}

fn draw_middle_line(context: &web_sys::CanvasRenderingContext2d) {
    context.set_line_width(5.0);
    context.begin_path();
    context.move_to(175.0, 175.0);
    context.line_to(425.0, 425.0);
    context.stroke();
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    draw_diamond(&context);
    draw_middle_line(&context);

    Ok(())
}
