use wasm_bindgen::prelude::*;

mod math;
mod error;
mod parser;
pub mod numeric_evaluator;

#[wasm_bindgen]
pub fn evaluate(expression: &str) -> f64 {
    numeric_evaluator::evaluate(expression).unwrap()
}