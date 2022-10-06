use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::SVG;
use crate::svg_data::Tag;

/** different hashes for comparing SVG tags.
*  This allows fast comparision by tags and tag trees
*  with different criteria.
*/
#[derive(Debug, Clone)]
pub(crate) struct TreeHash {
    all: u64,
    with_reorder: u64,
    without_attr: u64,
    without_text: u64,
    without_subtree: u64,
    only_tag: u64,
}

impl TreeHash {
    pub fn build_for_svg(svg: &SVG) -> Vec<TreeHash> {
        let result: Vec<TreeHash> = svg.tags.depth_first_map(
            |t, children| {
                let tc = children.iter().map(
                    |(_, hash)| -> &TreeHash {hash}
                ).collect();
                TreeHash::new(
                    t,
                    &tc
                )
            }
        );
        result
    }

    fn empty() -> TreeHash {
       TreeHash {
           all: 0,
           with_reorder: 0,
           without_attr: 0,
           without_text: 0,
           without_subtree: 0,
           only_tag: 0,
       }
    }

    fn new(
        tag: &Tag,
        children: &Vec<&TreeHash>
    ) -> TreeHash {
        // Create the hasher
        let mut only_tag_hasher = DefaultHasher::new();
        tag.name.hash(&mut only_tag_hasher);
        let mut all_hasher = only_tag_hasher.clone();
        let mut without_text_hasher = all_hasher.clone();
        tag.text.hash(&mut all_hasher);
        let mut without_attr_hasher = all_hasher.clone();
        let mut with_reorder_hasher = all_hasher.clone();
        let mut without_subtree_hasher = all_hasher.clone();

        // children without reorder
        for &c in children {
            c.all.hash(&mut all_hasher);
        }

        // Sort the children by "all"
        let mut all_children = children.clone();
        all_children.sort_by(|&a, &b| a.all.cmp(&b.all));
        for c in all_children {
            c.with_reorder.hash(&mut with_reorder_hasher);
            c.without_attr.hash(&mut without_attr_hasher);
            c.without_text.hash(&mut without_text_hasher);
        }

        // Sort the attributes
        let mut attr_keys: Vec<&String> = tag.args.keys().collect();
        attr_keys.sort();

        // Hash the attributes
        for &attr_key in &attr_keys {
            tag.args[attr_key].hash(&mut all_hasher);
            tag.args[attr_key].hash(&mut with_reorder_hasher);
            tag.args[attr_key].hash(&mut without_subtree_hasher);
            tag.args[attr_key].hash(&mut without_text_hasher);
        }

        TreeHash {
            all: all_hasher.finish(),
            without_text: without_text_hasher.finish(),
            without_attr: without_attr_hasher.finish(),
            with_reorder: with_reorder_hasher.finish(),
            without_subtree: without_subtree_hasher.finish(),
            only_tag: only_tag_hasher.finish(),
        }
    }

    pub fn eq_all(&self, o: &TreeHash) -> bool {
        self.all.eq(&o.all)
    }

    pub fn eq_without_text(&self, o: &TreeHash) -> bool {
        self.without_text.eq(&o.without_text)
    }

    pub fn eq_without_attr(&self, o: &TreeHash) -> bool {
        self.without_attr.eq(&o.without_attr)
    }

    pub fn eq_with_reorder(&self, o: &TreeHash) -> bool {
        self.with_reorder.eq(&o.with_reorder)
    }

    pub fn eq_without_subtree(&self, o: &TreeHash) -> bool {
        self.without_subtree.eq(&o.without_subtree)
    }

    pub fn eq_only_tag(&self, o: &TreeHash) -> bool {
        self.only_tag.eq(&o.only_tag)
    }
}

impl Default for TreeHash {
    fn default() -> Self {
        TreeHash::empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;
    use svg::node::Value;
    #[test]
    fn different_tag() {
        let tag_a = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let tag_b = Tag::new("circle".to_string(), "".to_string(), HashMap::new()).unwrap();
        let a = TreeHash::new(&tag_a, &vec![]);
        let b = TreeHash::new(&tag_b, &vec![]);
        assert!(!a.eq_all(&b));
        assert!(!a.eq_with_reorder(&b));
        assert!(!a.eq_without_attr(&b));
    }

    #[test]
    fn different_attr() {
        let tag_a = Tag::new("circle".to_string(), "".to_string(), HashMap::from([("attr".to_string(), Value::from("value1".to_string()))])).unwrap();
        let tag_b = Tag::new("circle".to_string(), "".to_string(), HashMap::from([("attr".to_string(), Value::from("value2".to_string()))])).unwrap();
        let a = TreeHash::new(&tag_a, &vec![]);
        let b = TreeHash::new(&tag_b, &vec![]);
        assert!(!a.eq_all(&b));
        assert!(!a.eq_with_reorder(&b));
        assert!(a.eq_without_attr(&b));
    }

    #[test]
    fn different_children() {
        let tag_a = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let tag_child = Tag::new("circle".to_string(), "".to_string(), HashMap::from([("attr".to_string(), Value::from("value".to_string()))])).unwrap();
        let tag_b = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();

        let a = &TreeHash::new(&tag_a, &vec![]);
        let b = TreeHash::new(
            &tag_b,
            &vec![&TreeHash::new(
                &tag_child,
                &vec![],
            )]
        );
        assert!(!a.eq_all(&b));
        assert!(!a.eq_with_reorder(&b));
        assert!(!a.eq_without_attr(&b));
    }

    #[test]
    fn same_children() {
        let tag_a = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let tag_child = Tag::new("circle".to_string(), "".to_string(), HashMap::from([("attr".to_string(), Value::from("value".to_string()))])).unwrap();
        let tag_b = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let a = TreeHash::new(
            &tag_a,
            &vec![&TreeHash::new(
                &tag_child,
                &vec![],
            )]
        );
        let b = TreeHash::new(
            &tag_b,
            &vec![&TreeHash::new(
                &tag_child,
                &vec![],
            )]
        );
        assert!(a.eq_all(&b));
        assert!(a.eq_with_reorder(&b));
        assert!(a.eq_without_attr(&b));
    }

    #[test]
    fn reorder_children() {
        let tag_a = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let tag_child1 = Tag::new("circle".to_string(), "".to_string(), HashMap::from([("attr".to_string(), Value::from("value".to_string()))])).unwrap();
        let tag_child2 = Tag::new("rect".to_string(), "".to_string(), HashMap::from([("attr".to_string(), Value::from("value".to_string()))])).unwrap();
        let _tag_b = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let a = TreeHash::new(
            &tag_a,
            &vec![
                &TreeHash::new(
                &tag_child1,
                &vec![],
                ),
                &TreeHash::new(
                    &tag_child2,
                    &vec![],
                )
            ]
        );
        let b = TreeHash::new(
            &tag_a,
            &vec![
                &TreeHash::new(
                    &tag_child2,
                    &vec![],
                ),
                &TreeHash::new(
                    &tag_child1,
                    &vec![],
                )
            ]
        );
        assert!(!a.eq_all(&b));
        assert!(a.eq_with_reorder(&b));
        assert!(a.eq_without_attr(&b));
    }
}
