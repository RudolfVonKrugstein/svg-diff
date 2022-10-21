use serde::{Deserialize, Serialize};

mod matching_rules;
pub use matching_rules::MatchingRule;
pub use matching_rules::MatchingRules;

// Get all and all subtrees hashes
#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct Config {
    pub matching: MatchingRules,
}
