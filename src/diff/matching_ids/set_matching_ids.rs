use crate::config::MatchingRules;
use crate::diff::matching_ids::{generator::MatchingIdGenerator, matching_state::MatchingState};
use crate::svg_data::{SVGWithMatchingState, SVGWithTreeHashSubtree, TreeHash};
use crate::SVG;
use flange_flat_tree::{Subtree, Tree};
use log::debug;

/// Find tags between `origin` and `target` that match and give them the same Matching ID.
/// These matching IDs can than later be used to find changes between the SVGs and generate the diff.
///
/// Iterates all childrens of both `origin` and `target` and find those which match based on their
/// `TreeHash`.
/// If children match, there are given the same matching ID and the algorithm also matches
/// the children of these found children.
///
/// The algorithm does not find matching between arbitrary Tags in the Tree.
/// Only if Tags match are the children also checked for matches.
///
/// The matches happen by the following priority (this with higher priority are preferred
/// over possible matches with lower priority). See als the Documentation of `TreeHash`.
///
/// * Full match (identical nodes in very aspect). Even children are all in the same order.
///   No matching IDs are given for children of these nodes.
/// * Full match if children are reordered in the tree.
/// * Match all but the text (with reorder).
/// * Match all but the attributes (with reorder).
/// * Match children with the same tag name.
///
/// NOTE: These criteria hold true for all tags in the Subtree.
///       i.E. matching without attributes mean, that the full subtree matches if
///       all attributes are ignored.
///
/// All children that remain are given Matching IDs that indicate that they are unmatched
/// and have to be removed or added.
///
/// NOTE: Not all tags are given MatchingIds. Tags that are below:
/// * Full matched nodes.
/// * Removed or added nodes.
/// Don't need a Matching ID and are not given one.
///
/// # Arguments
///
///  - origin - The first SVG to find matches in.
///  - target - The second SVG to find matches in.
///
/// # Result
///
/// An array for each of the svgs containing the matching IDs.
/// The resulting arrays are indexed the same way as the tags in the SVGs.

pub(crate) fn get_matching_ids<'a>(
    origin: &'a SVG,
    target: &'a SVG,
    rule_set: &MatchingRules,
    g: &mut MatchingIdGenerator,
) -> (SVGWithMatchingState<'a>, SVGWithMatchingState<'a>) {
    // Generate the treehashes
    let origin_with_treehash = TreeHash::build_for_svg(origin, &rule_set.rules);
    let target_with_treehash = TreeHash::build_for_svg(target, &rule_set.rules);

    // Make space for the result
    let mut origin_ids = vec![None; origin.tags.node_count()];
    let mut target_ids = vec![None; target.tags.node_count()];
    set_matching_ids_rec(
        origin_with_treehash.root(),
        target_with_treehash.root(),
        &mut origin_ids,
        &mut target_ids,
        rule_set,
        g,
    );
    (
        origin.with_matching_states(origin_ids),
        target.with_matching_states(target_ids),
    )
}

fn set_matching_ids_rec(
    origin: SVGWithTreeHashSubtree,
    target: SVGWithTreeHashSubtree,
    origin_ids: &mut Vec<Option<MatchingState>>,
    target_ids: &mut Vec<Option<MatchingState>>,
    rule_set: &MatchingRules,
    g: &mut MatchingIdGenerator,
) {
    // Get the origin tag (which we use as a default)
    let origin_id = origin.value().0.args.get("id").map(|a| a.to_string());

    // Create the matching id
    let id = MatchingState::new(
        g,
        origin.get_pos(),
        target.get_pos(),
        origin.value().1,
        target.value().1,
        origin_id,
    );
    if id.is_unmatched() {
        panic!("internal error, don't call set_matching_ids with non matching tags")
    }

    debug!("Setting matching for id {:?}", id);
    assert!(origin_ids[origin.get_pos()].is_none());
    origin_ids[origin.get_pos()] = Some(id.clone());
    assert!(target_ids[target.get_pos()].is_none());
    target_ids[target.get_pos()] = Some(id.clone());

    if id.full_match() {
        // Match 100%
        return;
    }
    // Find the child matches by all hashes
    for rule_name in &rule_set.priorities {
        while let Some((o_child, t_child)) = find_first_unmatched_child_pairs_that_matches(
            &origin, &target, origin_ids, target_ids, rule_name,
        ) {
            set_matching_ids_rec(o_child, t_child, origin_ids, target_ids, rule_set, g);
        }
    }
    // The rest remains unmatched
    for o_child in origin.children() {
        if origin_ids[o_child.get_pos()].is_none() {
            let child_id = o_child.value().0.args.get("id").map(|a| a.to_string());
            origin_ids[o_child.get_pos()] = Some(MatchingState::new_unmatched(
                o_child.get_pos(),
                true,
                g,
                child_id,
            ))
        }
    }
    for t_child in target.children() {
        if target_ids[t_child.get_pos()].is_none() {
            let child_id = t_child.value().0.args.get("id").map(|a| a.to_string());
            target_ids[t_child.get_pos()] = Some(MatchingState::new_unmatched(
                t_child.get_pos(),
                false,
                g,
                child_id,
            ))
        }
    }
}

fn find_first_unmatched_child_pairs_that_matches<'a>(
    a: &'a SVGWithTreeHashSubtree<'a>,
    b: &'a SVGWithTreeHashSubtree<'a>,
    origin_ids: &mut [Option<MatchingState>],
    target_ids: &mut [Option<MatchingState>],
    rule_name: &str,
) -> Option<(SVGWithTreeHashSubtree<'a>, SVGWithTreeHashSubtree<'a>)> {
    for a_child in a.children() {
        if origin_ids[a_child.get_pos()].is_some() {
            continue;
        };
        for b_child in b.children() {
            if target_ids[b_child.get_pos()].is_some() {
                continue;
            };
            if a_child.value().1.eq_rule(rule_name, b_child.value().1) {
                return Some((a_child, b_child));
            }
        }
    }
    None
}
