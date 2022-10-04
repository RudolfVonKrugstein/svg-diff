
use crate::diff::matching_ids::generator::MatchingIdGenerator;
use crate::SVGTag;

/** Run a function for all children (only direct childs, not recursive)
 * that don't have a matching id (yet?).
 *
 * # Arguments
 *
 * - tag - The `SVGTag` to find the children from.
*  - f - The function to call for evey child.
*/
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

/** Run a function for all pairs of children (only direct childs, not recursive)
 * that don't have a matching id (yet?).
 * If during the process a child gets a matching id, it is not passed again.
 *
 * # Arguments
 *
 * - tag - The `SVGTag` to find the children from.
*  - f - The function to call for evey pair of childs.
 */
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
