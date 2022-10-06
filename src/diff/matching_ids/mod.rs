//! This modules is for creation and handling of Mathcing IDs.
//!
//! The purpose of Matching IDs is to store, which SVG Element
//! in the original SVG has been matched to Elements in the target
//! SVG.
//!
//! Matching Elements are found by different criteria (see `set_matching_ids`).

mod generator;
mod matching_state;
mod set_matching_ids;

pub(crate) use self::generator::MatchingIdGenerator;
pub(crate) use self::matching_state::MatchingState;
pub(crate) use self::set_matching_ids::get_matching_ids;

