use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern{
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name))
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().except("no window (linux better)");
    let document = window.document().except("no documento");
    let body = document.body().except("no body");

    document.getElementById("money").set_text_content(Some("test"))
    greet();
}