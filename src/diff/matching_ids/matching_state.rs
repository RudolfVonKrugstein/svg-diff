use svg::node::Value;
use crate::diff::matching_ids::generator::MatchingIdGenerator;
use crate::svg_tag::TreeHash;

#[derive(Debug, Clone)]
pub(crate) struct MatchingState {
    matching_id: String,
    // what has changed?
    no_changes: bool, // Nothing has changed, full subtree match
    subtree_changes: bool, // There are changes in the subtree
    internal_changes: bool, // Has something changed inside this node
    no_match: bool, // We have no matching partner
}

impl MatchingState {
    pub fn get_id(&self) -> String {
        format!("{}", self.matching_id)
    }

    pub(crate) fn new(g: &mut MatchingIdGenerator, hash: &TreeHash, o_hash: &TreeHash, default_id: Option<String>) -> MatchingState {
        let no_changes = hash.eq_all(&o_hash);
        let subtree_changes = !no_changes;
        let internal_changes = !hash.eq_without_subtree(&o_hash);
        MatchingState {
            matching_id: g.next(default_id),
            no_changes,
            subtree_changes,
            internal_changes,
            no_match: false,
        }
    }

    pub(crate) fn new_unmatched(g: &mut MatchingIdGenerator, default_id: Option<String>) -> MatchingState {
        MatchingState {
            matching_id: g.next(default_id),
            no_changes: false,
            subtree_changes: false,
            internal_changes: false,
            no_match: true,
        }
    }

    pub fn full_match(&self) -> bool {
        self.no_changes
    }

    pub fn changes_in_subtree(&self) -> bool {
        self.subtree_changes
    }

    pub fn changes_in_node(&self) -> bool {
        self.internal_changes
    }

    pub fn is_none(&self) -> bool {
        self.no_match
    }
}
