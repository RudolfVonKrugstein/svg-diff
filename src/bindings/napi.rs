use crate::{diff_from_strings, DiffStep};

use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi]
#[derive(Serialize, Debug)]
struct JSResult {
    svgs: Vec<String>,
    diffs: Vec<Vec<DiffStep>>,
}

#[napi]
#[derive(Deserialize, Debug)]
struct Config {
    config: Option<crate::config::Config>,
}

#[napi]
fn svg_diffs(svg_strings: Vec<String>, config: &Config) -> JSResult {
    // Read the config
    let default_config = crate::config::Config::default();
    let use_config = if let Some(c) = &config.config {
        c
    } else {
        &default_config
    };

    // Convert the svgs
    let sdiff = diff_from_strings(&svg_strings, use_config).unwrap();

    JSResult {
        svgs: sdiff.0,
        diffs: sdiff.1,
    }
}
