use error_chain::bail;
use flange_flat_tree::Tree;
use regex::RegexBuilder;
use svg::parser::Event;
use svg::Parser;

use super::Tag;
use super::TreeHash;
use crate::diff::MatchingState;
use crate::errors::*;
use flange_flat_tree::Builder;
use flange_flat_tree::VecTree;

pub struct SVG {
    pub tags: VecTree<Tag>,
}

pub type SVGWithIDs<'a> = flange_flat_tree::FlangedTree<&'a VecTree<Tag>, Option<String>>;
pub(crate) type SVGWithMatchingState<'a> =
    flange_flat_tree::FlangedTree<&'a VecTree<Tag>, Option<MatchingState>>;
pub type SVGWithTreeHash<'a> = flange_flat_tree::FlangedTree<&'a VecTree<Tag>, TreeHash>;
pub type SVGWithTreeHashSubtree<'a> =
    <flange_flat_tree::FlangedTree<&'a VecTree<Tag>, TreeHash> as Tree<'a>>::SubtreeType;

impl SVG {
    pub fn with_ids(&self, ids: Vec<Option<String>>) -> SVGWithIDs {
        self.tags.flange(ids)
    }

    pub(crate) fn with_matching_states(
        &self,
        states: Vec<Option<MatchingState>>,
    ) -> SVGWithMatchingState {
        self.tags.flange(states)
    }

    pub fn parse_svg_string(input: &str) -> Result<SVG> {
        // Extract the svg part
        let re = RegexBuilder::new(r"<svg.*</svg>")
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        if let Some(svg_string) = re.find(input) {
            // Parse the svg part
            let mut p = svg::read(svg_string.as_str())?;
            SVG::parse_svg(&mut p)
        } else {
            bail!(format!("{} does not contain an svg", input))
        }
    }

    pub fn parse_svg(events: &mut Parser) -> Result<SVG> {
        let mut tags = Builder::new();
        // Go through svg event stream
        let mut current_index: usize = 0;
        for event in events.by_ref() {
            match event {
                Event::Error(e) => bail!(e),
                Event::Tag(tag_name, tag_type, tag_args) => match tag_type {
                    svg::node::element::tag::Type::Start => {
                        current_index = tags.start_element(Tag::new(
                            tag_name.to_string(),
                            "".to_string(),
                            tag_args,
                        )?);
                    }
                    svg::node::element::tag::Type::End => {
                        tags.end_element();
                    }
                    svg::node::element::tag::Type::Empty => {
                        tags.start_end_element(Tag::new(
                            tag_name.to_string(),
                            "".to_string(),
                            tag_args,
                        )?);
                    }
                },
                Event::Text(t) => {
                    tags.get_mut(current_index).unwrap().text = t.to_string();
                }
                Event::Comment => {
                    log::info!("ignoring comment")
                }
                Event::Declaration(t) => {
                    tags.get_mut(current_index).unwrap().text = t.to_string();
                }
                Event::Instruction => {
                    log::warn!("ignoring instruction")
                }
            }
        }
        Ok(SVG { tags: tags.build() })
    }
}

#[cfg(test)]
mod tests {
    use flange_flat_tree::{Subtree, Tree};

    use super::*;

    #[test]
    fn test_simple() {
        let data = r#"
        <svg height="100" width="100">
          <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red" />
          Sorry, your browser does not support inline SVG.
        </svg>
        "#;
        let mut parser = svg::read(data).unwrap();
        let result = SVG::parse_svg(&mut parser).unwrap();

        assert_eq!(result.tags.root().children().len(), 1);
        assert_eq!(result.tags.root().children()[0].value().name, "circle");
        assert_eq!(
            result.tags.root().value().text,
            "Sorry, your browser does not support inline SVG."
        );
    }
}
