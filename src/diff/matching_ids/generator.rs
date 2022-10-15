//! A generator for matching IDs in the SVG.
//! Whenever a pair of SVG elements is matched
//! they get a new ID generate by this generator.

use random_string::generate;

/// Generates IDs for matching SVG elements.
///
/// # How an ID looks
///
/// Every ID has this format: "<prefix>-<number>".
/// The prefix is generated for every instance
/// of the `MatchingIdGenerator`. This ensures
/// that IDs from different generators do not
/// overlap.
/// The number is just an integer number, that is
/// increased by one with for every ID.
pub struct MatchingIdGenerator {
    /// A common prefix for all generated ID.
    prefix: String,
    /// The next index to use for generating an ID.
    next_index: u64,
}

impl MatchingIdGenerator {
    /// New generator, sets the prefix to something random and the id to 0.
    pub fn new() -> MatchingIdGenerator {
        MatchingIdGenerator {
            prefix: generate(8, "abcdefghijklmnopqrstuvwxyz"),
            next_index: 0,
        }
    }

    /// Generates a new ID.
    /// If default_id is set to something but none, it is simple
    /// returned.
    /// The Idea is, that if one of the elements has already an ID attribute,
    /// that can be used for the ID.
    pub fn next(&mut self, default_id: Option<String>) -> String {
        if let Some(pre_id) = default_id {
            pre_id
        } else {
            let res = format!("{}-{}", self.prefix, self.next_index);
            self.next_index += 1;
            res
        }
    }
}
