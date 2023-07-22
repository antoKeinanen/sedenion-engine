use wasm_bindgen::prelude::*;

mod error;
pub mod numeric_evaluator;
mod math;

#[wasm_bindgen]
pub fn evaluate(expression: &str) -> f64 {
    numeric_evaluator::evaluate(expression).unwrap()
}