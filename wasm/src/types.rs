use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GreetResponse {
    pub value: String,
    #[tsify(optional)]
    pub name: Option<String>,
}
