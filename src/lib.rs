// Only include napi crates if we are building for node
#[cfg(feature = "node")]
extern crate napi;
#[cfg(feature = "node")]
extern crate napi_derive;

// Also only include our napi modules if we build for node
#[cfg(feature = "node")]
mod bindings;
#[cfg(feature = "node")]
pub use bindings::napi as napi_bindings;

// External crates we use
extern crate random_string;
extern crate error_chain;
extern crate serde_json;
extern crate serde;
extern crate svg;
extern crate svgtypes;
extern crate regex;

mod diff;
mod errors;
mod svg_tag;
pub use svg_tag::parse_svg_tag;
pub use svg_tag::parse_svg_string;
pub use svg_tag::SVGTag;
pub use svg_tag::print_svg;

pub use self::diff::diff;
pub use self::diff::diff_from_strings;
pub use self::diff::diffs;

pub use self::diff::JsonDiff;
pub use self::diff::write_json_diff;
