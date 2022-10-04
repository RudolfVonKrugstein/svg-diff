use error_chain::bail;
use std::collections::HashMap;
use log::debug;
use regex::{RegexBuilder};

use crate::errors::*;
use svg::node::element::tag::Type;
use svg::node::Value;
use svg::parser::Event;
use svg::Parser;

use crate::svg_tag::SVGTag;

pub fn parse_svg_string(input: &str) -> Result<SVGTag> {
    // Extract the svg part
    let re = RegexBuilder::new(r"<svg.*</svg>").multi_line(true).dot_matches_new_line(true).build().unwrap();
    if let Some(svg_string) = re.find(input) {
        // Parse the svg part
        let mut p = svg::read(svg_string.as_str())?;
        parse_svg_tag(&mut p)
    } else {
        bail!(format!("{} does not contain an svg", input))
    }
}

pub fn parse_svg_tag(events: &mut Parser) -> Result<SVGTag> {
    let first_node = events.next().ok_or(svg::parser::Error::new(
        (0, 0),
        "unable to parse tag, no new tag",
    ))?;
    if let Event::Tag(name, tag_type, args) = first_node {
        match tag_type {
            Type::Start => Ok(rec_parse_svg_tag(name.to_string(), args, events)?),
            Type::Empty => SVGTag::new(
                name.to_string(),
                "".to_string(),
                Vec::new(),
                args
            ),
            Type::End => {
                bail!(svg::parser::Error::new(
                    (0, 0),
                    "stream starts with end tag"
                ));
            }
        }
    } else {
        bail!(svg::parser::Error::new((0, 0), "not a tag in event stream"));
    }
}

/** This functions is supposed to be called after the "start" tag hs been
    found in the event stream.
*/
fn rec_parse_svg_tag(
    name: String,
    args: HashMap<String, Value>,
    events: &mut Parser,
) -> Result<SVGTag> {
    let mut children = Vec::new();
    let mut text = "".to_string();

    debug!("Starting tag type: {:?}", name);
    // Go through events until we find the close event
    while let Some(event) = events.next() {
        match event {
            Event::Error(e) => bail!(e),
            Event::Tag(tag_name, tag_type, tag_args) => match tag_type {
                svg::node::element::tag::Type::Start => {
                    children.push(rec_parse_svg_tag(tag_name.to_string(), tag_args, events)?)
                },
                svg::node::element::tag::Type::End => {
                    debug!("Ending tag type: {:?}", tag_name);
                    break;
                },
                svg::node::element::tag::Type::Empty => {
                    children.push(SVGTag::new_empty(tag_name.to_string(), tag_args)?);
                    debug!("Empty tag type: {:?}", tag_name);
                }
            },
            Event::Text(t) => {
                text = t.to_string();
            }
            Event::Comment => {}
            Event::Declaration => panic!("1"),
            Event::Instruction => panic!("!"),
        }
    }

    Ok(SVGTag::new(name, text, children, args)?)
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
        let result = parse_svg_tag(&mut parser).unwrap();

        assert_eq!(result.children.len(), 1);
        assert_eq!(result.children[0].name, "circle");
        assert_eq!(
            result.text,
            "Sorry, your browser does not support inline SVG."
        );
    }
}
