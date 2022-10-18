use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use crate::config::Config;
use crate::diff_from_strings;

pub fn svg_diffs(svgs_strings: Vec<String>, config: Option<String>) -> HashMap<String, Vec<String>> {
    // Read the config
    let use_config = if let Some(c) = config {
        serde_yaml::from_str(&c).unwrap()
    } else {
        Config::default()
    };

    // Convert the svgs
    let sdiff = diff_from_strings(&svgs_strings, &use_config).unwrap();

    let mut res = HashMap::new();
    res.insert("svgs".to_string(), sdiff.0);
    res.insert(
        "diffs".to_string(),
        sdiff
            .1
            .iter()
            .map(|d| serde_json::to_string(d).unwrap())
            .collect(),
    );
    res
}

