use crate::config::Config;
use crate::{diff_from_strings, DiffStep};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Debug)]
struct JSResult {
    pub svgs: Vec<String>,
    pub diffs: Vec<Vec<DiffStep>>,
}

#[wasm_bindgen]
pub fn svg_diffs(svgs: JsValue, config: JsValue) -> Result<JsValue, JsValue> {
    let svgs: Vec<String> = serde_wasm_bindgen::from_value(svgs)?;
    let config: Option<Config> = serde_wasm_bindgen::from_value(config)?;
    // Read the config
    let use_config = if let Some(c) = config {
        c
    } else {
        Config::default()
    };

    // Convert the svgs
    let sdiff = diff_from_strings(&svgs, &use_config).map_err(|e| e.to_string())?;

    Ok(serde_wasm_bindgen::to_value(&JSResult {
        svgs: sdiff.0,
        diffs: sdiff.1,
    })?)
}
