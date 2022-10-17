use crate::errors;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use svg::node::Value;
use svgtypes::{Color, PathParser, PathSegment, Transform};

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
    pub fn from_prop(prop: &String, value: &Value) -> errors::Result<SVGAttValue> {
        match prop.as_str() {
            "transform" => {
                // parse the transformation
                Ok(SVGAttValue::Matrix(matrix::MatrixValue::from_string(
                    &value.to_string(),
                )?))
            }
            "viewBox" => Ok(SVGAttValue::ViewBox(view_box::ViewBoxValue::from_string(
                &value.to_string(),
            )?)),
            "d" => Ok(SVGAttValue::Path(path::PathValue::from_string(
                &value.to_string(),
            )?)),
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

    pub fn to_string(&self) -> String {
        match &self {
            SVGAttValue::String(s) => s.clone(),
            SVGAttValue::Matrix(m) => m.to_string(),
            SVGAttValue::ViewBox(v) => v.to_string(),
            SVGAttValue::Path(p) => p.to_string(),
        }
    }

    pub fn hash_with_modifier<H: Hasher>(&self, with_pos: bool, with_style: bool, hasher: &mut H) {
        match &self {
            SVGAttValue::String(s) => s.hash(hasher),
            SVGAttValue::Matrix(m) => m.hash(hasher),
            SVGAttValue::ViewBox(v) => v.hash(hasher),
            SVGAttValue::Path(p) => p.hash_with_modifier(with_pos, hasher),
        }
    }
}
