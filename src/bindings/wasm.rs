use crate::config::Config;
use crate::{diff_from_strings, DiffStep};
use std::collections::HashMap;

pub struct JSResult {
    svgs: Vec<String>,
    diffs: Vec<Vec<DiffStep>>,
}

pub fn svg_diffs(svgs_strings: Vec<String>, config: Option<String>) -> JSResult {
    // Read the config
    let use_config = if let Some(c) = config {
        serde_yaml::from_str(&c).unwrap()
    } else {
        Config::default()
    };

    // Convert the svgs
    let sdiff = diff_from_strings(&svgs_strings, &use_config).unwrap();

    JSResult {
        svgs: sdiff.0,
        diffs: sdiff.1,
    }
}
