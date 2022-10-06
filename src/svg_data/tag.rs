use std::collections::HashMap;
use svg::node::Value;
use crate::svg_data::attributes::SVGAttValue;
use crate::errors::*;

#[derive(Debug, Clone)]
pub struct Tag {
    pub(crate) name: String,
    pub(crate) text: String,
    pub(crate) args: HashMap<String, SVGAttValue>,
}

impl Tag {
    pub fn new(name: String, text: String, in_args: HashMap<String, Value>) -> Result<Tag> {
        let mut args = HashMap::new();
        for (prop, value) in in_args.iter() {
            args.insert(prop.clone(), SVGAttValue::from_prop(prop, value)?);
        }
        Ok(Tag {
            name,
            text,
            args,
        })
    }
}
