use serde::Deserialize;

mod matching_rules;
pub use matching_rules::MatchingRule;
pub use matching_rules::MatchingRules;

// Get all and all subtrees hashes
#[derive(Deserialize, Default)]
pub struct Config {
    pub matching: MatchingRules,
}
