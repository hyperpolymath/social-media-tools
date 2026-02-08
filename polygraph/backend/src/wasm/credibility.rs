// WASM-optimized credibility scoring for browser execution
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredibilityInput {
    source_credibility: f64,
    fact_check_count: u32,
    corroborating_sources: u32,
    claim_complexity: f64,
}

#[wasm_bindgen]
pub struct CredibilityScorer {
    weights: Vec<f64>,
}

#[wasm_bindgen]
impl CredibilityScorer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            weights: vec![0.35, 0.25, 0.15, 0.25],
        }
    }

    #[wasm_bindgen]
    pub fn score(&self, input: JsValue) -> Result<f64, JsValue> {
        let input: CredibilityInput = serde_wasm_bindgen::from_value(input)?;

        let score = (
            input.source_credibility * self.weights[0]
            + (input.fact_check_count as f64 / 10.0).min(1.0) * self.weights[1]
            + (input.corroborating_sources as f64 / 5.0).min(1.0) * self.weights[2]
            + (1.0 - input.claim_complexity) * self.weights[3]
        ).clamp(0.0, 1.0);

        Ok(score)
    }

    #[wasm_bindgen]
    pub fn get_recommendation(&self, score: f64) -> String {
        match score {
            s if s >= 0.8 => "highly_credible".to_string(),
            s if s >= 0.6 => "likely_credible".to_string(),
            s if s >= 0.4 => "uncertain".to_string(),
            s if s >= 0.2 => "likely_not_credible".to_string(),
            _ => "not_credible".to_string(),
        }
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
