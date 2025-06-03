mod types;

use types::GreetResponse;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

#[wasm_bindgen]
pub fn greet(name: Option<String>) -> GreetResponse {
    log_1(&format!("greet called with name: {:?}", name).into());
    GreetResponse {
        value: "Hello, World!".to_string(),
        name,
    }
}
