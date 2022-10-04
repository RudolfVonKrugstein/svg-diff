use super::step::DiffStep;
use crate::diff::matching_ids::{MatchingIdGenerator, set_matching_ids};
use crate::diff::tree_flatter::flat_tree_of_matched_tags;
use crate::svg_tag::{Position, print_svg};
use crate::{parse_svg_string, SVGTag};
use crate::diff::hashmap_diff::HashMapDiff;

pub fn diff(origin: &mut SVGTag, target: &mut SVGTag) -> (SVGTag, SVGTag, Vec<DiffStep>) {
    // Track the result
    let mut diff = Vec::new();

    // Match using tagging ids
    let mut g = MatchingIdGenerator::new();
    set_matching_ids(origin, target, &mut g);

    // Set the base svg to the original svg with ids set!
    origin.copy_matching_ids_to_args();
    target.copy_matching_ids_to_args();

    // Flatten the trees
    let origin_flat = flat_tree_of_matched_tags(Position::root(), origin);
    let target_flat = flat_tree_of_matched_tags(Position::root(), target);

    // 1. Add unmatched tags in the target
    for (_id, item) in target_flat.iter() {
        if item.1.matching.as_ref().unwrap().is_none() {
            diff.push(DiffStep::add(item.1.clone(), item.0.clone()))
        }
    }

    // 2. remove unmatched tags
    for (_id, item) in origin_flat.iter() {
        if item.1.matching.as_ref().unwrap().is_none() {
            diff.push(DiffStep::remove(item.1));
        }
    }

    // 3. reorder items
    for (id, (pos, t_child)) in &target_flat {
        if !t_child.matching.as_ref().unwrap().is_none() {
            if let Some((or_pos, _)) = &origin_flat.get(id) {
                if or_pos != pos {
                    diff.push(DiffStep::move_element(id.clone(), pos.clone()));
                }
            } else {
                panic!("Unmatched id: {}", id);
            }
        }
    }

    // 4. finally change items
    for (id, (_, child)) in target_flat {
        if child.matching.as_ref().unwrap().changes_in_node() {
            let (_, original_child) = origin_flat.get(&id).unwrap();
            if child.text != original_child.text {
                diff.push(DiffStep::text_change(id.clone(), child.text.clone()))
            }
            let hash_diff = HashMapDiff::create(&original_child.args, &child.args);
            if !hash_diff.is_empty() {
                diff.push(DiffStep::change(id.clone(), hash_diff))
            }
        }
    }

    // Clear all matching ids!
    origin.reset_matching();
    target.reset_matching();

    // Return the result
    (origin.clone(), target.clone(), diff)
}

pub fn diffs(tags: &mut Vec<SVGTag>) -> (Vec<SVGTag>, Vec<Vec<DiffStep>>) {
    let mut svgs = Vec::new();
    let mut diffs = Vec::new();

    for index in 0..tags.len() - 1 {
        // We cannot borrow mutable twice, so we do a trick
        let (before, after) = tags.split_at_mut(index + 1);
        let d: (SVGTag, SVGTag, Vec<DiffStep>) = diff(
            &mut before.last_mut().unwrap(),
            &mut after.first_mut().unwrap(),
        );
        svgs.push(d.0);
        diffs.push(d.2);
    }

    (svgs, diffs)
}

pub fn diff_from_strings(
    svg_strings: &Vec<String>,
) -> crate::errors::Result<(Vec<String>, Vec<Vec<DiffStep>>)> {
    // Convert the input
    let svgs: crate::errors::Result<Vec<SVGTag>> = svg_strings
        .iter()
        .map(|s| {
            parse_svg_string(s.as_str())
        })
        .collect();
    let mut svgs = svgs?;

    // Create the diffs!
    let diff = diffs(&mut svgs);

    // Create result svgs
    let res_svgs = diff.0.iter().map(|t| print_svg(t)).collect();

    Ok((res_svgs, diff.1))
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
        let (_svgs, diffs) = diff_from_strings(&vec![origin, target]).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&vec![origin, target]).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&vec![origin, target]).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&vec![origin, target]).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&vec![origin, target]).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&vec![origin, target]).unwrap();

        // Test
        assert_eq!(diffs[0].len(), 2);
        assert!(diffs[0][0].is_move());
        assert!(diffs[0][0].is_move());
    }
}
