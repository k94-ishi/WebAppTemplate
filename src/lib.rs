use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calc(a: f64, b: f64, op: &str) -> f64 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                f64::NAN
            } else {
                a / b
            }
        }
        _ => f64::NAN,
    }
}

