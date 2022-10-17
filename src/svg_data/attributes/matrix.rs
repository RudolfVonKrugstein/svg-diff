use crate::errors::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use svgtypes::Transform;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MatrixValue {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

impl Display for MatrixValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "matrix({},{},{},{},{},{})",
            self.a, self.b, self.c, self.d, self.e, self.f
        )
    }
}

impl MatrixValue {
    pub fn from_string(input: &str) -> Result<MatrixValue> {
        let t = Transform::from_str(input)?;
        Ok(MatrixValue {
            a: t.a,
            b: t.b,
            c: t.c,
            d: t.d,
            e: t.e,
            f: t.f,
        })
    }
}

// Don't use for indexing hash maps!
// But this is good enough for comparing values for equality in our case.
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for MatrixValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!(
            "{:.8}{:.8}{:.8}{:.8}{:.8}{:.8}",
            self.a, self.b, self.c, self.d, self.e, self.f
        )
        .hash(state);
    }
}
