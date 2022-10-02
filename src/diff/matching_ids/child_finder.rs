use log::debug;
use crate::diff::matching_ids::generator::MatchingIdGenerator;
use crate::SVGTag;

pub fn for_each_unmatched_child<F>(tag: &mut SVGTag, g: &mut MatchingIdGenerator, f: F)
where
    F: Fn(&mut SVGTag, &mut MatchingIdGenerator),
{
    for child in tag.children.iter_mut() {
        if child.matching.is_some() {
            continue;
        }
        f(child, g);
    }
}
pub fn for_each_unmatched_child_pair<F>(
    origin: &mut SVGTag,
    target: &mut SVGTag,
    g: &mut MatchingIdGenerator,
    f: F,
) where
    F: Fn(&mut SVGTag, &mut SVGTag, &mut MatchingIdGenerator),
{
    for o_child in origin.children.iter_mut() {
        if o_child.matching.is_some() {
            continue;
        }
        for t_child in target.children.iter_mut() {
            if t_child.matching.is_some() {
                continue;
            }
            f(o_child, t_child, g);
            // Break out of the outer child has no a matching id
            if o_child.matching.is_some() {
                break;
            }
        }
    }
}
