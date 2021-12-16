use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r"export function prefers_dark() {
        return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    }")]
extern "C" {
    pub fn prefers_dark() -> bool;
}

#[wasm_bindgen(inline_js = r"export function prefers_light() {
        return window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches;
    }")]
extern "C" {
    pub fn prefers_light() -> bool;
}
