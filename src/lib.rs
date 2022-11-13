use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(message: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let greeting = "Hello from rust!";
    alert(greeting);
}
