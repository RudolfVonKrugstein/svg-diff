use crate::svg_data::Tag;
use serde::Deserialize;
use std::collections::HashSet;

fn default_true() -> bool {
    true
}

#[derive(Deserialize)]
pub struct MatchingAttrRule {
    pub included_attr: Option<HashSet<String>>,
    pub exclude_attr: HashSet<String>,
    #[serde(default = "default_true")]
    pub with_pos: bool,
    #[serde(default = "default_true")]
    pub with_style: bool,
}

#[derive(Deserialize)]
pub struct MatchingRule {
    pub name: String,
    pub apply_to_tags: Option<HashSet<String>>,
    #[serde(default = "HashSet::new")]
    pub dont_apply_to_tags: HashSet<String>,
    pub attr: MatchingAttrRule,
    #[serde(default = "default_true")]
    pub include_text: bool,
    #[serde(default = "default_true")]
    pub recursive: bool,
    pub childrens_rule: Option<String>,
    #[serde(default = "default_true")]
    pub sort_children: bool,
    pub prev_sibling_rule: Option<String>,
    pub next_sibling_rule: Option<String>,
}

impl MatchingRule {
    pub fn new_all_rule() -> MatchingRule {
        MatchingRule {
            name: "all".to_string(),
            apply_to_tags: None,
            dont_apply_to_tags: HashSet::new(),
            attr: MatchingAttrRule {
                included_attr: None,
                exclude_attr: HashSet::new(),
                with_pos: true,
                with_style: true,
            },
            include_text: true,
            recursive: true,
            childrens_rule: None,
            sort_children: false,
            prev_sibling_rule: None,
            next_sibling_rule: None,
        }
    }

    pub fn new_all_without_subtrees_rule() -> MatchingRule {
        MatchingRule {
            name: "all".to_string(),
            apply_to_tags: None,
            dont_apply_to_tags: HashSet::new(),
            attr: MatchingAttrRule {
                included_attr: None,
                exclude_attr: HashSet::new(),
                with_pos: true,
                with_style: true,
            },
            include_text: true,
            recursive: false,
            childrens_rule: None,
            sort_children: false,
            prev_sibling_rule: None,
            next_sibling_rule: None,
        }
    }

    pub fn new_all_subtrees_rule() -> MatchingRule {
        MatchingRule {
            name: "all_children".to_string(),
            apply_to_tags: None,
            dont_apply_to_tags: HashSet::new(),
            attr: MatchingAttrRule {
                included_attr: Some(HashSet::new()),
                exclude_attr: HashSet::new(),
                with_pos: false,
                with_style: false,
            },
            include_text: false,
            recursive: true,
            childrens_rule: Some("all".to_string()),
            sort_children: false,
            prev_sibling_rule: None,
            next_sibling_rule: None,
        }
    }

    pub fn default_rules() -> Vec<MatchingRule> {
        vec![
            MatchingRule {
                name: "with_reorder".to_string(),
                apply_to_tags: None,
                dont_apply_to_tags: HashSet::new(),
                attr: MatchingAttrRule {
                    included_attr: None,
                    exclude_attr: HashSet::new(),
                    with_pos: true,
                    with_style: true,
                },
                include_text: true,
                recursive: true,
                childrens_rule: None,
                sort_children: true,
                prev_sibling_rule: None,
                next_sibling_rule: None,
            },
            MatchingRule {
                name: "without_attr".to_string(),
                apply_to_tags: None,
                dont_apply_to_tags: HashSet::new(),
                attr: MatchingAttrRule {
                    included_attr: Some(HashSet::new()),
                    exclude_attr: HashSet::new(),
                    with_pos: false,
                    with_style: false,
                },
                include_text: true,
                recursive: true,
                childrens_rule: None,
                sort_children: true,
                prev_sibling_rule: None,
                next_sibling_rule: None,
            },
            MatchingRule {
                name: "without_text".to_string(),
                apply_to_tags: None,
                dont_apply_to_tags: HashSet::new(),
                attr: MatchingAttrRule {
                    included_attr: None,
                    exclude_attr: HashSet::new(),
                    with_pos: true,
                    with_style: true,
                },
                include_text: false,
                recursive: true,
                childrens_rule: None,
                sort_children: true,
                prev_sibling_rule: None,
                next_sibling_rule: None,
            },
            MatchingRule {
                name: "only_tag".to_string(),
                apply_to_tags: None,
                dont_apply_to_tags: HashSet::new(),
                attr: MatchingAttrRule {
                    included_attr: Some(HashSet::new()),
                    exclude_attr: HashSet::new(),
                    with_pos: true,
                    with_style: true,
                },
                include_text: false,
                recursive: true,
                childrens_rule: None,
                sort_children: true,
                prev_sibling_rule: None,
                next_sibling_rule: None,
            },
        ]
    }

    pub fn applies_to_tag(&self, tag: &Tag) -> bool {
        if let Some(includes) = &self.apply_to_tags {
            if !includes.contains(&tag.name.to_string()) {
                return false;
            }
        }
        return !self.dont_apply_to_tags.contains(&tag.name.to_string());
    }

    fn attr_is_excluded(&self, attr: &String) -> bool {
        if self.attr.exclude_attr.contains(attr) {
            return true;
        }
        if !self.attr.with_pos {
            if [
                "x".to_string(),
                "y".to_string(),
                "cx".to_string(),
                "cy".to_string(),
            ]
            .contains(attr)
            {
                return true;
            }
        }
        if !self.attr.with_style {
            if ["fill".to_string(), "stroke".to_string()].contains(attr) {
                return true;
            }
        }
        return false;
    }

    pub fn included_sorted_attr<'a>(&self, tag: &'a Tag) -> Vec<&'a String> {
        let mut res = Vec::new();
        if let Some(inc_attr) = &self.attr.included_attr {
            for (attr, _) in &tag.args {
                if inc_attr.contains(attr) && !self.attr_is_excluded(attr) {
                    res.push(attr);
                }
            }
        } else {
            for (attr, _) in &tag.args {
                if !self.attr_is_excluded(attr) {
                    res.push(attr);
                }
            }
        }
        res.sort();
        res
    }
}
