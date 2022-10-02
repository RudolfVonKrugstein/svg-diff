use std::str::FromStr;
use serde::{Serialize, Deserialize};
use svg::node::Value;
use svgtypes::{Color, Transform};
use crate::errors::*;

/** We distinguish between some attribute types, because they have to be handle
 *  specidal (tansform).
 * But mostly we convert attributes to strings.
 */

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MatrixValue {
    a: f64, b: f64, c: f64, d: f64, e: f64, f: f64
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum SVGAttValue {
    String(String),
    Matrix(MatrixValue)
}

impl SVGAttValue {
    pub fn from_prop(prop: &String, value: &Value) -> Result<SVGAttValue> {
        if prop == "transform" {
            // parse the transformation
            let t = Transform::from_str(value.to_string().as_str())?;
            Ok(SVGAttValue::Matrix(MatrixValue {
                a: t.a,
                b: t.b,
                c: t.c,
                d: t.d,
                e: t.e,
                f: t.f,
            }))
        } else {
            if prop == "fill" || prop == "stroke" {
                let color = Color::from_str(value.to_string().as_str())?;
                if color.alpha == 255 {
                    Ok(SVGAttValue::String(format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)))
                } else {
                    Ok(SVGAttValue::String(format!("#{:02x}{:02x}{:02x}{:02x}", color.red, color.green, color.blue, color.alpha)))
                }
            } else {
                Ok(SVGAttValue::String(value.to_string()))
            }
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            SVGAttValue::String(s) => s.clone(),
            SVGAttValue::Matrix(m) => format!("matrix({},{},{},{},{},{})", m.a, m.b, m.c, m.d, m.e, m.f)
        }
    }
}
