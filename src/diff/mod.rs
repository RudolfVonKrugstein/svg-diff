mod diff_funcs;
mod hashmap_diff;
mod matching_ids;
mod step;

pub use self::diff_funcs::diff;
pub use self::diff_funcs::diff_from_strings;
pub use self::diff_funcs::diffs;
pub use self::step::DiffStep;
pub(crate) use matching_ids::MatchingState;
