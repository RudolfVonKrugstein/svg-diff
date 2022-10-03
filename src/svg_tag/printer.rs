
use svg::node::element::{Element, SVG};
use svg::node::Text;
use svg::{Node};
use crate::SVGTag;

fn build_element(tag: &SVGTag) -> Element {
    let mut el = svg::node::element::Element::new(&tag.name);
    for (name, value) in &tag.args {
        el.assign(name, svg::node::Value::from(value.to_string()))
    }
    for child in &tag.children {
        el.append(build_element(child));
    }
    if tag.text.len() > 0 {
        el.append(Text::new(&tag.text))
    }
    el
}

fn build_doc(tag: &SVGTag) -> SVG {
    let mut doc = svg::Document::new();
    for (name, value) in &tag.args {
        doc.assign(name, svg::node::Value::from(value.to_string()))
    }
    for child in &tag.children {
        doc.append(build_element(child));
    }
    doc
}

pub fn print_svg(tag: &SVGTag) -> String {
    let doc = build_doc(tag);
    doc.to_string()
}

pub fn print_svg_element(tag: &SVGTag) -> String {
    let doc = build_element(tag);
    doc.to_string()
}
