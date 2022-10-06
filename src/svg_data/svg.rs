use error_chain::bail;
use regex::RegexBuilder;
use svg::Parser;
use svg::parser::Event;

use crate::flat_tree::{FlatTree, FlatTreeBuilder, Navigator, NavigatorWithValues};
use super::Tag;
use crate::errors::*;
use crate::flat_tree;

#[derive(Debug)]
pub struct SVG {
    pub tags: FlatTree<Tag>
}

pub type SVGWithIDs<'a> = flat_tree::NavigatorWithValues<'a, Tag, Option<String>>;

impl SVG {
    pub fn with_ids<'a>(&'a self, ids: &'a Vec<Option<String>>) -> SVGWithIDs<'a> {
        self.with_values(ids)
    }

    pub fn with_values<'a, A>(&'a self, values: &'a Vec<A>) -> NavigatorWithValues<'a, Tag,A> {
        NavigatorWithValues::from_iterator(
            Navigator::new(&self.tags, 0),
            values,
        )
    }

    pub fn parse_svg_string(input: &str) -> Result<SVG> {
        // Extract the svg part
        let re = RegexBuilder::new(r"<svg.*</svg>").multi_line(true).dot_matches_new_line(true).build().unwrap();
        if let Some(svg_string) = re.find(input) {
            // Parse the svg part
            let mut p = svg::read(svg_string.as_str())?;
            SVG::parse_svg(&mut p)
        } else {
            bail!(format!("{} does not contain an svg", input))
        }
    }

    pub fn parse_svg(events: &mut Parser) -> Result<SVG> {
        let mut tags = FlatTreeBuilder::new();
        // Go through svg event stream
        let mut current_index :usize = 0;
        while let Some(event) = events.next() {
            match event {
                Event::Error(e) => bail!(e),
                Event::Tag(tag_name, tag_type, tag_args) => match tag_type {
                    svg::node::element::tag::Type::Start => {
                        current_index = tags.start_element(
                            Tag::new(tag_name.to_string(), "".to_string(), tag_args)?
                        );
                    },
                    svg::node::element::tag::Type::End => {
                        tags.end_element();
                    },
                    svg::node::element::tag::Type::Empty => {
                        tags.start_end_element(Tag::new(tag_name.to_string(), "".to_string(), tag_args)?);
                    }
                },
                Event::Text(t) => {
                    tags.get_mut(current_index).unwrap().text = t.to_string();
                }
                Event::Comment => {}
                Event::Declaration => panic!("1"),
                Event::Instruction => panic!("!"),
            }
        };
        Ok(
            SVG {
                tags: tags.build()
            }
        )
    }
}

#[cfg(test)]
mod tests {
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

        assert_eq!(result.tags.children(0).len(), 1);
        assert_eq!(result.tags.children(0)[0].name, "circle");
        assert_eq!(
            result.tags.get(0).unwrap().text,
            "Sorry, your browser does not support inline SVG."
        );
    }
}
