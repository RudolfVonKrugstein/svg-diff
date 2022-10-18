use flange_flat_tree::{Subtree, Tree};
use std::cmp::Ordering::Equal;
use std::cmp::{max_by, min_by};
use std::str::FromStr;

use super::step::DiffStep;
use crate::diff::hashmap_diff::HashMapDiff;
use crate::diff::matching_ids::{get_matching_ids, MatchingIdGenerator};
use crate::errors::*;
use crate::svg_data::SVGWithIDs;
use crate::{config, print_svg, SVG};

pub fn diff<'a>(
    origin: &'a SVG,
    target: &'a SVG,
    config: &'a config::Config,
) -> (SVGWithIDs, SVGWithIDs, Vec<DiffStep>) {
    // Track the result
    let mut diff = Vec::new();

    // Match using tagging ids
    let mut g = MatchingIdGenerator::new();
    let (origin_with_states, target_with_states) =
        get_matching_ids(origin, target, &config.matching, &mut g);

    // Build the svg with ids
    // let origin_with_ids = origin.with_ids(&origin_ids);
    let target_with_ids =
        target_with_states.replace_map_flange(|s| s.1.as_ref().map(|s| s.get_id()));
    let origin_with_ids =
        origin_with_states.replace_map_flange(|s| s.1.as_ref().map(|s| s.get_id()));

    // 1. Add unmatched tags in the target
    target_with_states.for_each(|s| {
        if s.value()
            .1
            .as_ref()
            .map(|s| s.is_unmatched())
            .unwrap_or(false)
        {
            diff.push(DiffStep::add(&target_with_ids.at_pos(s.get_pos())))
        };
    });

    // 2. remove unmatched tags
    origin_with_states.for_each(|s| {
        if s.value()
            .1
            .as_ref()
            .map(|s| s.is_unmatched())
            .unwrap_or(false)
        {
            diff.push(DiffStep::remove(&origin_with_ids.at_pos(s.get_pos())))
        }
    });

    // 3. reorder items
    target_with_states.for_each(|s| {
        if let Some(target_state) = s.value().1 {
            if target_state.changes_in_subtree() {
                if let Some(origin_index) = target_state.get_origin_index() {
                    // Get the ids of both origin and target, that have not been removed or added
                    let mut current_childs = Vec::new();
                    for child in origin_with_states.at_pos(origin_index).children() {
                        if !child.value().1.as_ref().unwrap().is_unmatched() {
                            current_childs.push(child);
                        }
                    }
                    let mut target_childs = Vec::new();
                    for child in s.children() {
                        if !child.value().1.as_ref().unwrap().is_unmatched() {
                            target_childs.push(child);
                        }
                    }
                    assert!(current_childs.len() == target_childs.len());
                    // Find those, that don't match (that means must be reordered)!
                    let mut unmatched_indices = Vec::new();
                    for (index, target_child) in target_childs.into_iter().enumerate() {
                        if current_childs[index]
                            .value()
                            .1
                            .as_ref()
                            .unwrap()
                            .get_target_index()
                            .unwrap()
                            != target_child.get_pos()
                        {
                            {
                                // modified the origin child ids to match
                                let swap_index = {
                                    current_childs
                                        .iter()
                                        .enumerate()
                                        .find(|(_index, child)| {
                                            child
                                                .value()
                                                .1
                                                .as_ref()
                                                .unwrap()
                                                .get_target_index()
                                                .unwrap()
                                                == target_child.get_pos()
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
                    }
                    // Push those unmatched indices
                    for target in unmatched_indices {
                        diff.push(DiffStep::move_element(
                            &target_with_ids.at_pos(target.get_pos()),
                        ));
                    }
                }
            }
        }
    });

    // 4. finally change items
    target_with_states.for_each(|s| {
        if let Some(target_state) = s.value().1 {
            if target_state.changes_in_node() {
                if let Some(origin_index) = target_state.get_origin_index() {
                    let origin_tag = origin.tags.at_pos(origin_index).value();
                    let target_tag = s.value().0;
                    if origin_tag.text != target_tag.text {
                        diff.push(DiffStep::text_change(
                            target_state.get_id(),
                            target_tag.text.clone(),
                        ))
                    }
                    let hash_diff = HashMapDiff::create(&origin_tag.args, &target_tag.args);
                    if !hash_diff.is_empty() {
                        diff.push(DiffStep::change(target_state.get_id(), hash_diff))
                    }
                }
            }
        }
    });

    // Return the result
    (origin_with_ids, target_with_ids, diff)
}

pub fn diffs<'a>(
    tags: &'a Vec<SVG>,
    min_view_box: Option<svgtypes::ViewBox>,
    config: &'a config::Config,
) -> (Vec<SVGWithIDs<'a>>, Vec<Vec<DiffStep>>, svgtypes::ViewBox) {
    let mut svgs = Vec::new();
    let mut diffs = Vec::new();

    // Find the biggest all containing viewbox
    let mut all_viewbox =
        min_view_box.unwrap_or_else(|| svgtypes::ViewBox::new(0.0, 0.0, 0.0, 0.0));
    for svg in tags {
        if svg.tags.root().value().args.contains_key("viewBox") {
            let svg_viewbox = svgtypes::ViewBox::from_str(
                svg.tags.root().value().args["viewBox"].to_string().as_str(),
            )
            .unwrap_or(all_viewbox);
            let x_start = min_by(all_viewbox.x, svg_viewbox.x, |a, b| {
                a.partial_cmp(b).unwrap_or(Equal)
            });
            let x_end = max_by(
                all_viewbox.x + all_viewbox.w,
                svg_viewbox.x + svg_viewbox.w,
                |a, b| a.partial_cmp(b).unwrap_or(Equal),
            );
            let y_start = min_by(all_viewbox.y, svg_viewbox.y, |a, b| {
                a.partial_cmp(b).unwrap_or(Equal)
            });
            let y_end = max_by(
                all_viewbox.y + all_viewbox.h,
                svg_viewbox.y + svg_viewbox.h,
                |a, b| a.partial_cmp(b).unwrap_or(Equal),
            );
            all_viewbox =
                svgtypes::ViewBox::new(x_start, y_start, x_end - x_start, y_end - y_start);
        }
    }

    for index in 0..tags.len() - 1 {
        // We cannot borrow mutable twice, so we do a trick
        let d: (SVGWithIDs, SVGWithIDs, Vec<DiffStep>) =
            diff(&tags[index], &tags[index + 1], config);
        svgs.push(d.0);
        diffs.push(d.2);
    }

    (svgs, diffs, all_viewbox)
}

pub fn diff_from_strings(
    svg_strings: &[String],
    config: &config::Config,
) -> Result<(Vec<String>, Vec<Vec<DiffStep>>)> {
    // Convert the input
    let svgs: Result<Vec<SVG>> = svg_strings
        .iter()
        .map(|s| match SVG::parse_svg_string(s.as_str()) {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        })
        .collect();
    let svgs = svgs?;

    // Create the diffs!
    let (svg_with_ids, diff, view_box) = diffs(&svgs, None, config);

    // Create result svgs
    let mut res_svgs = Vec::new();
    for svg in svg_with_ids.into_iter() {
        res_svgs.push(print_svg(&svg, Some(&view_box)));
    }

    Ok((res_svgs, diff))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::Config;

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
        let (_svgs, diffs) = diff_from_strings(&[origin, target], &Config::default()).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&[origin, target], &Config::default()).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&[origin, target], &Config::default()).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&[origin, target], &Config::default()).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&[origin, target], &Config::default()).unwrap();

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
        let (_svgs, diffs) = diff_from_strings(&[origin, target], &Config::default()).unwrap();

        // Test
        assert_eq!(diffs[0].len(), 1);
        assert!(diffs[0][0].is_move());
    }
}
