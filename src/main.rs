use wasm_bindgen::prelude::*;

// This is the entry point for the WebAssembly module
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}