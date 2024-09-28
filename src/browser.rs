use anyhow::{anyhow, Result};
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlCanvasElement, Window};

pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| anyhow!("no window found"))
}

pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| anyhow!("no document found"))
}

pub fn canvas() -> Result<HtmlCanvasElement> {
    document()?
        .get_element_by_id("canvas")
        .ok_or_else(|| anyhow!("no canvas found"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| anyhow!("failed to cast to HtmlCanvasElement"))
}

pub fn context() -> Result<web_sys::CanvasRenderingContext2d> {
    canvas()?
        .get_context("2d")
        .map_err(|_| anyhow!("Error getting 2d context"))?
        .ok_or_else(|| anyhow!("No 2d context found"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .map_err(|element| {
            anyhow!(
                "Error converting {:#?} to CanvasRenderingContext2d",
                element
            )
        })
}
