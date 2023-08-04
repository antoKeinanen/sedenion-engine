use wasm_bindgen::prelude::*;

mod math;
mod error;
mod parser;
mod optimizer;
pub mod numeric_evaluator;

#[cfg(test)]
mod tests;


#[wasm_bindgen]
pub fn evaluate(expression: &str) -> f64 {
    numeric_evaluator::evaluate(expression).unwrap()
}