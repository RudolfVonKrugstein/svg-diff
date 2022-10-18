use serde::Deserialize;

mod rule;
pub use rule::MatchingRule;

#[derive(Deserialize)]
pub struct MatchingRules {
    pub rules: Vec<MatchingRule>,
    pub priorities: Vec<String>,
}

impl Default for MatchingRules {
    fn default() -> MatchingRules {
        MatchingRules {
            rules: MatchingRule::default_rules(),
            priorities: vec![
                "next_is_same_text".to_string(),
                "all".to_string(),
                "with_reorder".to_string(),
                "without_attr".to_string(),
                "without_text".to_string(),
                "only_tag".to_string(),
            ],
        }
    }
}
