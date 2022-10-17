use flange_flat_tree::{Subtree, Tree};

use crate::config::MatchingRule;
use crate::svg_data::Tag;
use crate::SVG;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::svg::SVGWithTreeHash;

/** different hashes for comparing SVG tags.
*  This allows fast comparision by tags and tag trees
*  with different criteria.
*/
#[derive(Debug, Clone)]
pub struct TreeHash {
    all: u64,
    all_subtrees: u64,
    all_without_subtrees: u64,
    rules: HashMap<String, u64>,
}

impl TreeHash {
    pub fn build_for_svg<'a>(svg: &'a SVG, rules: &Vec<MatchingRule>) -> SVGWithTreeHash<'a> {
        let mut res = svg
            .tags
            .depth_first_flange(|t, children| TreeHash::new(t, &children));
        for rule in rules {
            svg.tags.get_nav().for_each_depth_first(|i, _a| {
                let tag = svg.tags.at_pos(i).value();
                let prev_sibling = res.get_nav().prev_sibling(i).map(|s| res.get_flange(s));
                let next_sibling = res.get_nav().next_sibling(i).map(|s| res.get_flange(s));
                let children = res
                    .get_nav()
                    .children(i)
                    .iter()
                    .map(|s| res.get_flange(*s))
                    .collect();
                let val = TreeHash::calc_hash(rule, tag, &children, prev_sibling, next_sibling);
                if let Some(v) = val {
                    res.get_flange_mut(i).rules.insert(rule.name.clone(), v);
                }
            });
        }
        res
    }

    fn get_sibling_value(sibling: &Option<&TreeHash>, rule: &str) -> Option<u64> {
        sibling.and_then(|s| s.rules.get(rule)).cloned()
    }

    pub fn calc_hash(
        rule: &MatchingRule,
        tag: &Tag,
        children: &Vec<&TreeHash>,
        prev_sibling: Option<&TreeHash>,
        next_sibling: Option<&TreeHash>,
    ) -> Option<u64> {
        // Check if the rule should be applied to us
        if !rule.applies_to_tag(tag) {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        // Check if the rule does not apply to use because we don't have the siblings
        if let Some(prev_rule) = &rule.prev_sibling_rule {
            if let Some(value) = Self::get_sibling_value(&prev_sibling, prev_rule) {
                value.hash(&mut hasher);
            } else {
                return None;
            }
        };
        if let Some(next_rule) = &rule.next_sibling_rule {
            if let Some(value) = Self::get_sibling_value(&next_sibling, next_rule) {
                value.hash(&mut hasher);
            } else {
                return None;
            }
        };
        // Now apply the tag itself
        tag.name.hash(&mut hasher);
        // Text?
        if rule.include_text {
            tag.text.hash(&mut hasher);
        }
        // Children?
        if rule.recursive {
            // Which rule for children? Default to myself!
            let child_rule = rule.childrens_rule.as_ref().unwrap_or(&rule.name);
            let mut child_values = Vec::with_capacity(children.len());
            for child in children {
                child_values.push(*child.rules.get(child_rule).unwrap());
            }
            // Should the children be sorted?
            if rule.sort_children {
                child_values.sort();
            }
            for v in child_values {
                v.hash(&mut hasher);
            }
        }
        // Sort the attributes
        let attritbutes = rule.included_sorted_attr(tag);
        for attribute in attritbutes {
            tag.args.get(attribute).unwrap().hash_with_modifier(
                rule.attr.with_pos,
                rule.attr.with_style,
                &mut hasher,
            );
            attribute.hash(&mut hasher);
        }
        Some(hasher.finish())
    }

    fn empty() -> TreeHash {
        TreeHash {
            all: 0,
            all_subtrees: 0,
            all_without_subtrees: 0,
            rules: HashMap::new(),
        }
    }

    fn new(tag: &Tag, children: &Vec<&TreeHash>) -> TreeHash {
        // Create the hasher
        let all =
            Self::calc_hash(&MatchingRule::new_all_rule(), tag, children, None, None).unwrap();
        let all_subtrees = Self::calc_hash(
            &MatchingRule::new_all_subtrees_rule(),
            tag,
            children,
            None,
            None,
        )
        .unwrap();
        let all_without_subtrees = Self::calc_hash(
            &MatchingRule::new_all_without_subtrees_rule(),
            tag,
            children,
            None,
            None,
        )
        .unwrap();
        let mut rules = HashMap::new();
        rules.insert("all".to_string(), all);
        rules.insert("all_subtrees".to_string(), all_subtrees);
        rules.insert("all_without_subtrees".to_string(), all_without_subtrees);

        TreeHash {
            all,
            all_subtrees,
            all_without_subtrees,
            rules,
        }
    }

    pub fn eq_all(&self, o: &TreeHash) -> bool {
        self.all.eq(&o.all)
    }

    pub fn eq_all_subtrees(&self, o: &TreeHash) -> bool {
        self.all_subtrees.eq(&o.all_subtrees)
    }

    pub fn eq_all_without_subtrees(&self, o: &TreeHash) -> bool {
        self.all_without_subtrees.eq(&o.all_without_subtrees)
    }

    pub fn eq_rule(&self, name: &str, o: &TreeHash) -> bool {
        if let Some(my_value) = self.rules.get(name) {
            if let Some(o_value) = o.rules.get(name) {
                return my_value.eq(o_value);
            }
        }
        false
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
        assert!(!a.eq_rule("with_reorder", &b));
        assert!(!a.eq_rule("without_attr", &b));
    }

    #[test]
    fn different_attr() {
        let tag_a = Tag::new(
            "circle".to_string(),
            "".to_string(),
            HashMap::from([("attr".to_string(), Value::from("value1".to_string()))]),
        )
        .unwrap();
        let tag_b = Tag::new(
            "circle".to_string(),
            "".to_string(),
            HashMap::from([("attr".to_string(), Value::from("value2".to_string()))]),
        )
        .unwrap();
        let a = TreeHash::new(&tag_a, &vec![]);
        let b = TreeHash::new(&tag_b, &vec![]);
        assert!(!a.eq_all(&b));
        assert!(!a.eq_rule("with_reorder", &b));
        assert!(a.eq_rule("without_attr", &b));
    }

    #[test]
    fn different_children() {
        let tag_a = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let tag_child = Tag::new(
            "circle".to_string(),
            "".to_string(),
            HashMap::from([("attr".to_string(), Value::from("value".to_string()))]),
        )
        .unwrap();
        let tag_b = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();

        let a = &TreeHash::new(&tag_a, &vec![]);
        let b = TreeHash::new(&tag_b, &vec![&TreeHash::new(&tag_child, &vec![])]);
        assert!(!a.eq_all(&b));
        assert!(!a.eq_rule("with_reorder", &b));
        assert!(!a.eq_rule("without_attr", &b));
    }

    #[test]
    fn same_children() {
        let tag_a = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let tag_child = Tag::new(
            "circle".to_string(),
            "".to_string(),
            HashMap::from([("attr".to_string(), Value::from("value".to_string()))]),
        )
        .unwrap();
        let tag_b = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let a = TreeHash::new(&tag_a, &vec![&TreeHash::new(&tag_child, &vec![])]);
        let b = TreeHash::new(&tag_b, &vec![&TreeHash::new(&tag_child, &vec![])]);
        assert!(a.eq_all(&b));
        assert!(a.eq_rule("with_reorder", &b));
        assert!(a.eq_rule("without_attr", &b));
    }

    #[test]
    fn reorder_children() {
        let tag_a = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let tag_child1 = Tag::new(
            "circle".to_string(),
            "".to_string(),
            HashMap::from([("attr".to_string(), Value::from("value".to_string()))]),
        )
        .unwrap();
        let tag_child2 = Tag::new(
            "rect".to_string(),
            "".to_string(),
            HashMap::from([("attr".to_string(), Value::from("value".to_string()))]),
        )
        .unwrap();
        let _tag_b = Tag::new("g".to_string(), "".to_string(), HashMap::new()).unwrap();
        let a = TreeHash::new(
            &tag_a,
            &vec![
                &TreeHash::new(&tag_child1, &vec![]),
                &TreeHash::new(&tag_child2, &vec![]),
            ],
        );
        let b = TreeHash::new(
            &tag_a,
            &vec![
                &TreeHash::new(&tag_child2, &vec![]),
                &TreeHash::new(&tag_child1, &vec![]),
            ],
        );
        assert!(!a.eq_all(&b));
        assert!(a.eq_rule("with_reorder", &b));
        assert!(a.eq_rule("without_attr", &b));
    }
}
