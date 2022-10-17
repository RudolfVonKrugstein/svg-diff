use crate::diff::matching_ids::generator::MatchingIdGenerator;
use crate::svg_data::TreeHash;

/// Stores how a SVG element is matched to
/// another element (in the corresponding diff between the SVGs).
/// For this a `Option<MatchingState>` is stored in very `SVGTag`
/// which is set to `None`, as long as the tag is not matched
/// and to `Some` when the element is matched or can not be matched.
///
/// Besides the matching-ID (which identifies matching Elements by giving
/// them the same ID), it stores how good the match is.
/// In particular:
///
/// * Which between the too elements are the same?
/// * Is there a difference in the subtrees?
/// * Is this a full match and the elements are just identical?
/// * Is this not a match at all, and this element does not have a matching counterpart?
#[derive(Debug, Clone)]
pub(crate) struct MatchingState {
    matching_id: String,
    // The index in the origin svg
    origin_index: Option<usize>,
    // Same for the target
    target_index: Option<usize>,
    // The id of the match (is the same in both matching elements).
    no_changes: bool,
    // Nothing has changed, full subtree match
    subtree_changes: bool,
    // There are changes in the subtree
    internal_changes: bool,
    // Has something changed inside this node
    no_match: bool, // We have no matching partner
}

impl MatchingState {
    /// Returns the ID as it should be stored in the "id" attribute of the element.
    pub fn get_id(&self) -> String {
        self.matching_id.to_string()
    }

    /// Create a new matching State.
    ///
    /// # Arguments
    ///
    /// - g - The generator to generate IDs.
    /// - hash - The `TreeHash` of the first (original) element.
    /// - o_hash - The `TreeHash` of the second (target) element.
    /// - default_id - A default ID, which (if not None) overwrites the ID from the generator. Use
    ///                this if the elements already have IDs and you want to keep them.
    ///
    /// # Result
    ///
    /// The `MatchingState` to be added to the Tags.
    /// Clone it to add it to both tags!
    pub(crate) fn new(
        g: &mut MatchingIdGenerator,
        origin_index: usize,
        target_index: usize,
        hash: &TreeHash,
        o_hash: &TreeHash,
        default_id: Option<String>,
    ) -> MatchingState {
        let no_changes = hash.eq_all(o_hash);
        let subtree_changes = !hash.eq_all_subtrees(o_hash);
        let internal_changes = !hash.eq_all_without_subtrees(o_hash);
        MatchingState {
            matching_id: g.next(default_id),
            origin_index: Some(origin_index),
            target_index: Some(target_index),
            no_changes,
            subtree_changes,
            internal_changes,
            no_match: false,
        }
    }

    /// Create an MatchingState that represents an unmatched tag.
    ///
    /// # Arguments
    ///
    /// - g - The genertor for IDs.
    /// - default_id - A default ID, which (if not None) overwrites the ID from the generator. Use
    ///                this if the elements already have IDs and you want to keep them.
    ///
    /// # Result
    ///
    /// The `MatchingState` to be added to the Tag.
    /// Don't clone it, it should be added to only one tag!
    pub(crate) fn new_unmatched(
        index: usize,
        is_origin: bool,
        g: &mut MatchingIdGenerator,
        default_id: Option<String>,
    ) -> MatchingState {
        MatchingState {
            matching_id: g.next(default_id),
            no_changes: false,
            origin_index: if is_origin { Some(index) } else { None },
            target_index: if is_origin { None } else { Some(index) },
            subtree_changes: false,
            internal_changes: false,
            no_match: true,
        }
    }

    /// If this returns true, the Tags are identiacal including all childs
    pub fn full_match(&self) -> bool {
        self.no_changes
    }

    /// If this is false, the subtrees don't have to be traversed for changes (they are identical).
    pub fn changes_in_subtree(&self) -> bool {
        self.subtree_changes
    }

    /// If this is true, something about the node (an attribute or the text) has changed.
    pub fn changes_in_node(&self) -> bool {
        self.internal_changes
    }

    /// If this is true, it is an unmatched node.
    pub fn is_unmatched(&self) -> bool {
        self.no_match
    }

    // Return the indices
    pub fn get_target_index(&self) -> Option<usize> {
        self.target_index
    }
    pub fn get_origin_index(&self) -> Option<usize> {
        self.origin_index
    }
}
