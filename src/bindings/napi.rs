use crate::{diff_from_strings, DiffStep};

use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi]
#[derive(Serialize, Debug)]
struct JSResult {
    pub svgs: Vec<String>,
    pub diffs: Vec<Vec<serde_json::Value>>,
}

#[napi]
fn svg_diffs(svg_strings: Vec<String>, config: Option<serde_json::Value>) -> JSResult {
    // Read the config
    let default_config = crate::config::Config::default();
    let use_config = if let Some(c) = config {
        serde_json::from_value(c).unwrap()
    } else {
        default_config
    };

    // Convert the svgs
    let sdiff = diff_from_strings(&svg_strings, &use_config).unwrap();

    JSResult {
        svgs: sdiff.0,
        diffs: sdiff
            .1
            .iter()
            .map(|v| v.iter().map(|v| serde_json::to_value(v).unwrap()).collect())
            .collect(),
    }
}
