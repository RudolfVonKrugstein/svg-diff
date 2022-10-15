use flange_flat_tree::{Subtree, Tree};

use super::step::DiffStep;
use crate::diff::hashmap_diff::HashMapDiff;
use crate::diff::matching_ids::{get_matching_ids, MatchingIdGenerator};
use crate::errors::*;
use crate::svg_data::SVGWithIDs;
use crate::{print_svg, SVG};

pub fn diff<'a>(origin: &'a SVG, target: &'a SVG) -> (SVGWithIDs, SVGWithIDs, Vec<DiffStep>) {
    // Track the result
    let mut diff = Vec::new();

    // Match using tagging ids
    let mut g = MatchingIdGenerator::new();
    let (origin_states, target_states) = get_matching_ids(origin, target, &mut g);
    let origin_ids: Vec<Option<String>> = origin_states
        .iter()
        .map(|s| s.as_ref().map(|m| m.get_id()))
        .collect();
    let target_ids: Vec<Option<String>> = target_states
        .iter()
        .map(|s| s.as_ref().map(|m| m.get_id()))
        .collect();

    // Build the svg with ids
    // let origin_with_ids = origin.with_ids(&origin_ids);
    let target_with_ids = target.with_ids(target_ids);
    let origin_with_ids = origin.with_ids(origin_ids);

    // 1. Add unmatched tags in the target
    for (index, state) in target_states.iter().enumerate() {
        if state.as_ref().map(|s| s.is_unmatched()).unwrap_or(false) {
            diff.push(DiffStep::add(&target_with_ids.at_pos(index)))
        }
    }

    // 2. remove unmatched tags
    for (index, state) in origin_states.iter().enumerate() {
        if state.as_ref().map(|s| s.is_unmatched()).unwrap_or(false) {
            diff.push(DiffStep::remove(&origin_with_ids.at_pos(index)))
        }
    }

    // 3. reorder items
    for (_target_index, id) in target_states.iter().enumerate() {
        if let Some(id) = id {
            if id.changes_in_subtree() {
                if let Some(origin_index) = id.get_origin_index() {
                    // Get the ids of both origin and target, that have not been removed or added
                    let mut current_childs = Vec::new();
                    for child in origin_with_ids.at_pos(origin_index).children() {
                        if !origin_states[child.get_pos()]
                            .as_ref()
                            .unwrap()
                            .is_unmatched()
                        {
                            current_childs.push(child);
                        }
                    }
                    let mut target_childs = Vec::new();
                    for child in target_with_ids.at_pos(origin_index).children() {
                        if !target_states[child.get_pos()]
                            .as_ref()
                            .unwrap()
                            .is_unmatched()
                        {
                            target_childs.push(child);
                        }
                    }
                    assert!(current_childs.len() == target_childs.len());
                    // Find those, that don't match!
                    let mut unmatched_indices = Vec::new();
                    for (index, target_child) in target_childs.into_iter().enumerate() {
                        if current_childs[index].value().1.as_ref().unwrap()
                            != target_child.value().1.as_ref().unwrap()
                        {
                            // modified the origin child ids to match
                            let swap_index = {
                                current_childs
                                    .iter()
                                    .enumerate()
                                    .find(|(_index, child)| {
                                        child.value().1.as_ref().unwrap()
                                            == target_child.value().1.as_ref().unwrap()
                                    })
                                    .unwrap()
                                    .0
                            };
                            // Swap the indices
                            current_childs.swap(swap_index, index);
                            // Remember the target child
                            unmatched_indices.push(target_child);
                        }
                    }
                    // Push those unmatched indices
                    for target in unmatched_indices {
                        diff.push(DiffStep::move_element(&target));
                    }
                }
            }
        }
    }

    // 4. finally change items
    for (target_index, id) in target_states.iter().enumerate() {
        if let Some(id) = id {
            if id.changes_in_node() {
                if let Some(origin_index) = id.get_origin_index() {
                    let origin_tag = origin.tags.at_pos(origin_index).value();
                    let target_tag = target.tags.at_pos(target_index).value();
                    if origin_tag.text != target_tag.text {
                        diff.push(DiffStep::text_change(id.get_id(), target_tag.text.clone()))
                    }
                    let hash_diff = HashMapDiff::create(&origin_tag.args, &target_tag.args);
                    if !hash_diff.is_empty() {
                        diff.push(DiffStep::change(id.get_id(), hash_diff))
                    }
                }
            }
        }
    }

    // Return the result
    (origin_with_ids, target_with_ids, diff)
}

pub fn diffs(tags: &Vec<SVG>) -> (Vec<SVGWithIDs>, Vec<Vec<DiffStep>>) {
    let mut svgs = Vec::new();
    let mut diffs = Vec::new();

    for index in 0..tags.len() - 1 {
        // We cannot borrow mutable twice, so we do a trick
        let d: (SVGWithIDs, SVGWithIDs, Vec<DiffStep>) = diff(&tags[index], &tags[index + 1]);
        svgs.push(d.0);
        diffs.push(d.2);
    }

    (svgs, diffs)
}

pub fn diff_from_strings(svg_strings: &[String]) -> Result<(Vec<String>, Vec<Vec<DiffStep>>)> {
    // Convert the input
    let svgs: crate::errors::Result<Vec<SVG>> = svg_strings
        .iter()
        .map(|s| match SVG::parse_svg_string(s.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        })
        .collect();
    let svgs = svgs?;

    // Create the diffs!
    let (svg_with_ids, diff) = diffs(&svgs);

    // Create result svgs
    let mut res_svgs = Vec::new();
    for svg in svg_with_ids.into_iter() {
        res_svgs.push(print_svg(&svg));
    }

    Ok((res_svgs, diff))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove() {
        // setup
        let origin = r#"
        <svg>
          <circle id="the_circle" cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
        </svg>
        "#.to_string();
        let target = r#"
        <svg>
        </svg>
        "#
        .to_string();

        // Act
        let (_svgs, diffs) = diff_from_strings(&[origin, target]).unwrap();

        // Test
        assert_eq!(diffs[0].len(), 1);
        assert!(diffs[0][0].is_remove());
    }

    #[test]
    fn test_add() {
        // setup
        let origin = r#"
        <svg>
          <circle cx="20" cy="20" r="10"/>
          <circle cx="40" cy="40" r="10"/>
        </svg>
        "#
        .to_string();
        let target = r#"
        <svg>
          <circle cx="20" cy="20" r="10"/>
          <circle cx="40" cy="40" r="10"/>
          <circle id="the_circle" cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
        </svg>
        "#.to_string();

        // Act
        let (_svgs, diffs) = diff_from_strings(&[origin, target]).unwrap();

        // Test
        assert_eq!(diffs[0].len(), 1);
        assert!(diffs[0][0].is_add());
    }

    #[test]
    fn test_change() {
        // setup
        let origin = r#"
        <svg>
          <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
        </svg>
        "#
        .to_string();
        let target = r#"
        <svg>
          <circle cx="49" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
        </svg>
        "#
        .to_string();

        // Act
        let (_svgs, diffs) = diff_from_strings(&[origin, target]).unwrap();

        // Test
        assert_eq!(diffs[0].len(), 1);
        assert!(diffs[0][0].is_change());
    }

    #[test]
    fn test_text_change() {
        // setup
        let origin = r#"
        <svg>
          <text>Hello</text>
        </svg>
        "#
        .to_string();
        let target = r#"
        <svg>
          <text>Good Bye</text>
        </svg>
        "#
        .to_string();

        // Act
        let (_svgs, diffs) = diff_from_strings(&[origin, target]).unwrap();

        // Test
        assert_eq!(diffs[0].len(), 1);
        assert!(diffs[0][0].is_text_change());
    }

    #[test]
    fn test_text_and_prop_change() {
        // setup
        let origin = r###"
        <svg>
          <text color="#00FF00">Hello</text>
        </svg>
        "###
        .to_string();
        let target = r###"
        <svg>
          <text>Good Bye</text>
        </svg>
        "###
        .to_string();

        // Act
        let (_svgs, diffs) = diff_from_strings(&[origin, target]).unwrap();

        // Test
        assert_eq!(diffs[0].len(), 2);
        assert!(diffs[0][0].is_text_change());
        assert!(diffs[0][1].is_change());
    }

    #[test]
    fn reorder_change() {
        // setup
        let origin = r###"
        <svg>
          <g></g>
          <text color="#00FF00">Hello</text>
        </svg>
        "###
        .to_string();
        let target = r###"
        <svg>
          <text color="#00FF00">Hello</text>
          <g></g>
        </svg>
        "###
        .to_string();

        // Act
        let (_svgs, diffs) = diff_from_strings(&[origin, target]).unwrap();

        // Test
        assert_eq!(diffs[0].len(), 1);
        assert!(diffs[0][0].is_move());
    }
}
