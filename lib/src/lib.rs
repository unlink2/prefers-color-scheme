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

#[wasm_bindgen(inline_js = r"export function prefers_none() {
        return window.matchMedia && window.matchMedia('(prefers-color-scheme: no-preference)').matches;
    }")]
extern "C" {
    pub fn prefers_none() -> bool;
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_should_return_true_for_dark_mode() {
        assert!(prefers_dark());
    }

    #[wasm_bindgen_test]
    fn it_should_return_false_for_light_mode() {
        assert!(!prefers_light());
    }

    #[wasm_bindgen_test]
    fn it_should_return_false_for_none() {
        assert!(!prefers_none());
    }
}
