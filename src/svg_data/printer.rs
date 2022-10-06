
use svg::node::Text;
use svg::{Node};

use crate::svg_data::svg::SVGWithIDs;

fn build_element(svg: &SVGWithIDs) -> svg::node::element::Element {
    let tag = svg.get_main();
    let mut el = svg::node::element::Element::new(&tag.name);
    for (name, value) in &tag.args {
        el.assign(name, svg::node::Value::from(value.to_string()))
    }
    if let Some(id) = svg.get_extra() {
        el.assign("id", id.clone());
    }
    for child in &svg.children() {
        el.append(build_element(child));
    }
    if tag.text.len() > 0 {
        el.append(Text::new(&tag.text))
    }
    el
}

fn build_doc(svg: &SVGWithIDs) -> svg::Document {
    let mut doc = svg::Document::new();
    let root_tag = svg.get_main();
    for (name, value) in &root_tag.args {
        doc.assign(name, svg::node::Value::from(value.to_string()))
    }
    if let Some(id) = svg.get_extra() {
        doc.assign("id", id.clone());
    }
    for child in &svg.children() {
        doc.append(build_element(child));
    }
    if root_tag.text.len() > 0 {
        doc.append(Text::new(&root_tag.text))
    }
    doc
}

pub fn print_svg(svg: &SVGWithIDs) -> String {
    let doc = build_doc(svg);
    doc.to_string()
}

pub fn print_svg_element(svg: &SVGWithIDs) -> String {
    let doc = build_element(svg);
    doc.to_string()
}
