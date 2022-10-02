mod diff;
mod hashmap_diff;
mod matching_ids;
mod step;
mod tree_flatter;

pub use self::diff::diff;
pub use self::diff::diff_from_strings;
pub use self::diff::diffs;

pub(crate) use self::matching_ids::MatchingState;
pub use self::step::DiffStep;
