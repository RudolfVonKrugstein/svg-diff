use crate::errors::*;
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct ViewBoxValue {
    view_box: svgtypes::ViewBox,
}

impl Serialize for ViewBoxValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}

// Don't use for indexing hash maps!
// But this is good enough for comparing values for equality in our case.
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for ViewBoxValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!(
            "{:.8} {:.8} {:.8} {:.8}",
            self.view_box.x, self.view_box.y, self.view_box.w, self.view_box.h
        )
        .hash(state);
    }
}

impl ViewBoxValue {
    pub fn from_string(s: &str) -> Result<ViewBoxValue> {
        Ok(ViewBoxValue {
            view_box: svgtypes::ViewBox::from_str(s)?,
        })
    }
}

impl Display for ViewBoxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.view_box.x, self.view_box.y, self.view_box.w, self.view_box.h
        )
    }
}
