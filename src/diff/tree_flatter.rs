use std::collections::HashMap;
use crate::svg_tag::Position;
use crate::SVGTag;

pub fn flat_tree_of_matched_tags(
    position: Position,
    input: &SVGTag,
) -> HashMap<String, (Position, &SVGTag)> {
    let mut result = HashMap::new();

    let my_id = input.matching.as_ref().unwrap().get_id();

    if input.matching.as_ref().unwrap().changes_in_subtree() {
        let mut prev_child_id = None;
        for (index, child) in input.children.iter().enumerate() {
            let next_child_id = input.children.get(index+1).map(
                |c| c.matching.as_ref().unwrap().get_id()
            );
            let n = flat_tree_of_matched_tags(
                Position {
                    parent_id: my_id.clone(),
                    child_index: index,
                    prev_child: prev_child_id,
                    next_child: next_child_id,
                },
                child,
            );
            for (key, value) in n {
                result.insert(key, value);
            }
            prev_child_id = Some(child.matching.as_ref().unwrap().get_id());
        }
    }

    // Append ourself
    result.insert(my_id, (position, input));

    result
}
