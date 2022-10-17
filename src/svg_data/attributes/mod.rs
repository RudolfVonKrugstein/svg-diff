use crate::errors;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use svg::node::Value;
use svgtypes::Color;

mod matrix;
mod path;
mod view_box;

/** We distinguish between some attribute types, because they have to be handle
*  specidal (tansform).
* But mostly we convert attributes to strings.
 */

#[derive(Serialize, Hash, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SVGAttValue {
    String(String),
    Matrix(matrix::MatrixValue),
    Path(path::PathValue),
    ViewBox(view_box::ViewBoxValue),
}

impl SVGAttValue {
    pub fn from_prop(prop: &str, value: &Value) -> errors::Result<SVGAttValue> {
        match prop {
            "transform" => {
                // parse the transformation
                Ok(SVGAttValue::Matrix(matrix::MatrixValue::from_string(
                    value,
                )?))
            }
            "viewBox" => Ok(SVGAttValue::ViewBox(view_box::ViewBoxValue::from_string(
                value,
            )?)),
            "d" => Ok(SVGAttValue::Path(path::PathValue::from_string(value)?)),
            "fill" | "stroke" => {
                let color = Color::from_str(value.to_string().as_str())?;
                if color.alpha == 255 {
                    Ok(SVGAttValue::String(format!(
                        "#{:02x}{:02x}{:02x}",
                        color.red, color.green, color.blue
                    )))
                } else {
                    Ok(SVGAttValue::String(format!(
                        "#{:02x}{:02x}{:02x}{:02x}",
                        color.red, color.green, color.blue, color.alpha
                    )))
                }
            }
            _ => Ok(SVGAttValue::String(value.to_string())),
        }
    }

    pub fn hash_with_modifier<H: Hasher>(&self, with_pos: bool, _with_style: bool, hasher: &mut H) {
        match &self {
            SVGAttValue::String(s) => s.hash(hasher),
            SVGAttValue::Matrix(m) => m.hash(hasher),
            SVGAttValue::ViewBox(v) => v.hash(hasher),
            SVGAttValue::Path(p) => p.hash_with_modifier(with_pos, hasher),
        }
    }
}

impl Display for SVGAttValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SVGAttValue::String(s) => write!(f, "{}", s),
            SVGAttValue::Matrix(m) => m.fmt(f),
            SVGAttValue::ViewBox(v) => v.fmt(f),
            SVGAttValue::Path(p) => p.fmt(f),
        }
    }
}
