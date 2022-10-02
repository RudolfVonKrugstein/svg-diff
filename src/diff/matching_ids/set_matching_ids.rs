use log::{debug, info};
use crate::diff::matching_ids::child_finder::*;
use crate::diff::matching_ids::generator::MatchingIdGenerator;
use crate::diff::matching_ids::matching_state::MatchingState;
use crate::SVGTag;

pub fn set_matching_ids(origin: &mut SVGTag, target: &mut SVGTag, g: &mut MatchingIdGenerator) {
    let id = MatchingState::new(
        g, &origin.hash, &target.hash, origin.args.get("id").map(|v| v.to_string())); // Next matching id
    if id.is_none() {
        panic!("internal error, don't call set_matching_ids with non matching tags")
    }
    assert!(origin.matching.is_none());
    assert!(target.matching.is_none());
    debug!("Setting matching for id {:?}", id);
    origin.matching = Some(id.clone());
    target.matching = Some(id.clone());
    if id.full_match() {
        // Match 100%
        return;
    }
    // Find the child matches by all hashes
    for_each_unmatched_child_pair(origin, target, g, |o, t, g| {
        if o.hash.eq_all(&t.hash) {
            set_matching_ids(o, t, g);
        }
    });
    for_each_unmatched_child_pair(origin, target, g, |o, t, g| {
        if o.hash.eq_with_reorder(&t.hash) {
            set_matching_ids(o, t, g);
        }
    });
    for_each_unmatched_child_pair(origin, target, g, |o, t, g| {
        if o.hash.eq_without_text(&t.hash) {
            set_matching_ids(o, t, g);
        }
    });
    for_each_unmatched_child_pair(origin, target, g, |o, t, g| {
        if o.hash.eq_without_attr(&t.hash) {
            set_matching_ids(o, t, g);
        }
    });
    for_each_unmatched_child_pair(origin, target, g, |o, t, g| {
        if o.name == t.name {
            set_matching_ids(o, t, g);
        }
    });
    // Set the rest unmatched!
    for_each_unmatched_child(origin, g, |t, g| {
        t.matching = Some(
            MatchingState::new_unmatched(g, t.args.get("id").map(|v| v.to_string()))
        );
        info!("Unmatched matching id: {:?}\n", t.matching)
    });
    for_each_unmatched_child(target, g, |t, g| {
        t.matching = Some(
            MatchingState::new_unmatched(g, None)
        );
        info!("Unmatched matching id: {:?}\n", t.matching)
    });
}

pub fn print_matching_id_tree(tag: &SVGTag, level: usize) {
    for i in 0..level {
        print!(" ")
    }
    print!("|{}\n", tag.matching.as_ref().unwrap().get_id());
    if tag.matching.as_ref().unwrap().changes_in_subtree() {
        for c in &tag.children {
            print_matching_id_tree(c, level + 1);
        }
    }
}
