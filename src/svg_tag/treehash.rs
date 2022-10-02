use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use svg::node::Attributes;

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
}

impl TreeHash {
    pub(crate) fn new(
        tag: &String,
        text: &String,
        children: &Vec<&TreeHash>,
        attr: &Attributes,
    ) -> TreeHash {
        // Create the hasher
        let mut all_hasher = DefaultHasher::new();
        tag.hash(&mut all_hasher);
        let mut without_text_hasher = all_hasher.clone();
        text.hash(&mut all_hasher);
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
        let mut attr_keys: Vec<&String> = attr.keys().collect();
        attr_keys.sort();

        // Hash the attributes
        for &attr_key in &attr_keys {
            attr[attr_key].hash(&mut all_hasher);
            attr[attr_key].hash(&mut with_reorder_hasher);
            attr[attr_key].hash(&mut without_subtree_hasher);
            attr[attr_key].hash(&mut without_text_hasher);
        }

        TreeHash {
            all: all_hasher.finish(),
            without_text: without_text_hasher.finish(),
            without_attr: without_attr_hasher.finish(),
            with_reorder: with_reorder_hasher.finish(),
            without_subtree: without_subtree_hasher.finish(),
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
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;
    use svg::node::Value;

    #[test]
    fn different_tag() {
        let a = TreeHash::new(&"g".to_string(), &"".to_string(), &vec![], &HashMap::new());
        let b = TreeHash::new(
            &"circle".to_string(),
            &"".to_string(),
            &vec![],
            &HashMap::new(),
        );
        assert!(!a.eq_all(&b));
        assert!(!a.eq_with_reorder(&b));
        assert!(!a.eq_without_attr(&b));
    }

    #[test]
    fn different_attr() {
        let a = TreeHash::new(&"g".to_string(), &"".to_string(), &vec![], &HashMap::new());
        let b = TreeHash::new(
            &"g".to_string(),
            &"".to_string(),
            &vec![],
            &HashMap::from([("attr".to_string(), Value::from("value".to_string()))]),
        );
        assert!(!a.eq_all(&b));
        assert!(!a.eq_with_reorder(&b));
        assert!(a.eq_without_attr(&b));
    }

    #[test]
    fn different_children() {
        let a = &TreeHash::new(&"g".to_string(), &"".to_string(), &vec![], &HashMap::new());
        let b = TreeHash::new(
            &"g".to_string(),
            &"".to_string(),
            &vec![&TreeHash::new(
                &"g".to_string(),
                &"".to_string(),
                &vec![],
                &HashMap::new(),
            )],
            &HashMap::new(),
        );
        assert!(!a.eq_all(&b));
        assert!(!a.eq_with_reorder(&b));
        assert!(!a.eq_without_attr(&b));
    }

    #[test]
    fn same_children() {
        let a = TreeHash::new(
            &"g".to_string(),
            &"".to_string(),
            &vec![&TreeHash::new(
                &"g".to_string(),
                &"".to_string(),
                &vec![],
                &HashMap::new(),
            )],
            &HashMap::new(),
        );
        let b = TreeHash::new(
            &"g".to_string(),
            &"".to_string(),
            &vec![&TreeHash::new(
                &"g".to_string(),
                &"".to_string(),
                &vec![],
                &HashMap::new(),
            )],
            &HashMap::new(),
        );
        assert!(a.eq_all(&b));
        assert!(a.eq_with_reorder(&b));
        assert!(a.eq_without_attr(&b));
    }

    #[test]
    fn reorder_children() {
        let a = TreeHash::new(
            &"g".to_string(),
            &"".to_string(),
            &vec![
                &TreeHash::new(&"g".to_string(), &"".to_string(), &vec![], &HashMap::new()),
                &TreeHash::new(&"c".to_string(), &"".to_string(), &vec![], &HashMap::new()),
            ],
            &HashMap::new(),
        );
        let b = TreeHash::new(
            &"g".to_string(),
            &"".to_string(),
            &vec![
                &TreeHash::new(&"c".to_string(), &"".to_string(), &vec![], &HashMap::new()),
                &TreeHash::new(&"g".to_string(), &"".to_string(), &vec![], &HashMap::new()),
            ],
            &HashMap::new(),
        );
        assert!(!a.eq_all(&b));
        assert!(a.eq_with_reorder(&b));
        assert!(a.eq_without_attr(&b));
    }
}
