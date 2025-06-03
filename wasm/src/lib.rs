mod types;

use types::GreetResponse;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: Option<String>) -> GreetResponse {
    GreetResponse {
        value: "Hello, World!".to_string(),
        name,
    }
}
