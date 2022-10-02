use crate::diff::DiffStep;
use crate::diff_from_strings;

use napi_derive::napi;
use std::collections::HashMap;

#[napi]
fn svg_diffs(svgs_strings: Vec<String>) -> HashMap<String, Vec<String>> {
    // Convert the svgs
    let sdiff = diff_from_strings(&svgs_strings).unwrap();

    let mut res = HashMap::new();
    res.insert("svgs".to_string(), sdiff.0);
    res.insert(
        "diffs".to_string(),
        sdiff
            .1
            .iter()
            .map(|d| DiffStep::write_json(d).unwrap())
            .collect(),
    );
    res
}
