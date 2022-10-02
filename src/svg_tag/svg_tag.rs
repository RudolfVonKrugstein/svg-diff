use crate::diff::MatchingState;
use std::collections::HashMap;
use svg::node::Value;
use crate::svg_tag::attributes::SVGAttValue;
use crate::svg_tag::treehash::TreeHash;
use crate::errors::*;

#[derive(Debug, Clone)]
pub struct SVGTag {
    pub(crate) name: String,
    pub(crate) children: Vec<SVGTag>,
    pub(crate) text: String,
    pub(crate) args: HashMap<String, SVGAttValue>,
    pub(crate) hash: TreeHash,
    pub(crate) matching: Option<MatchingState>,
}

impl SVGTag {
    pub fn new(
        name: String,
        text: String,
        children: Vec<SVGTag>,
        args: HashMap<String, Value>,
    ) -> Result<SVGTag> {
        let mut trans_args = HashMap::new();
        for (prop, value) in args.iter() {
            trans_args.insert(prop.clone(), SVGAttValue::from_prop(prop, value)?);
        }
        Ok(SVGTag {
            hash: TreeHash::new(
                &name,
                &text,
                &children.iter().map(|c| &c.hash).collect(),
                &args,
            ),
            text,
            name,
            args: trans_args,
            children,
            matching: None,
        })
    }

    pub fn new_empty(name: String, args: HashMap<String, Value>) -> Result<SVGTag> {
        let mut trans_args = HashMap::new();
        for (prop, value) in args.iter() {
            trans_args.insert(prop.clone(), SVGAttValue::from_prop(prop, value)?);
        }
        SVGTag::new(name, "".to_string(), Vec::new(), args)
    }

    pub fn reset_matching(&mut self) {
        self.for_all_nodes_mut(
            &|n| n.matching = None
        )
    }

    pub fn copy_matching_ids_to_args(&mut self) {
        self.for_all_nodes_mut(&|t| {
            if let Some(m) = &t.matching {
                t.args.insert("id".to_string(), SVGAttValue::String(m.get_id()));
            }
        });
    }

    pub fn for_all_nodes_mut<F>(&mut self, f: &F)
    where
        F: Fn(&mut SVGTag),
    {
        for child in &mut self.children {
            child.for_all_nodes_mut(f);
        }
        f(self);
    }
}
