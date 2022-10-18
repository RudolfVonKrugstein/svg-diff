// Only include napi crates if we are building for node
#[cfg(feature = "node")]
extern crate napi;
#[cfg(feature = "node")]
extern crate napi_derive;

// Same for wasm
#[cfg(feature = "wasm")]
extern crate wasm_bindgen;

// Also only include our napi modules if we build for node
#[cfg(all(feature = "node", feature = "wasm"))]
mod bindings;

#[cfg(feature = "node")]
pub use bindings::napi as napi_bindings;

#[cfg(feature = "wasm")]
pub use bindings::wasm as wasm_bindings;

// External crates we use
extern crate error_chain;
extern crate flange_flat_tree;
extern crate random_string;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate svg;
extern crate svgtypes;

pub mod config;
mod diff;
mod errors;
mod svg_data;

pub use svg_data::print_svg;
pub use svg_data::SVG;

pub use self::diff::diff;
pub use self::diff::diff_from_strings;
pub use self::diff::diffs;
pub use self::diff::DiffStep;
