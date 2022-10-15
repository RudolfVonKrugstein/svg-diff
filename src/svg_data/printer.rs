use flange_flat_tree::{Subtree, Tree};
use svg::node::Text;
use svg::Node;

use crate::svg_data::svg::SVGWithIDs;

use super::Tag;

fn build_element<'a, ST: Subtree<Node = (&'a Tag, &'a Option<String>)>>(
    svg: &ST,
) -> svg::node::element::Element {
    let tag = svg.value().0;
    let id = svg.value().1;
    let mut el = svg::node::element::Element::new(&tag.name);
    for (name, value) in &tag.args {
        el.assign(name, svg::node::Value::from(value.to_string()))
    }
    if let Some(id) = id {
        el.assign("id", id.clone());
    }
    for child in &svg.children() {
        el.append(build_element(child));
    }
    if !tag.text.is_empty() {
        el.append(Text::new(&tag.text))
    }
    el
}

fn build_doc(svg: &SVGWithIDs) -> svg::Document {
    let mut doc = svg::Document::new();
    let (root_tag, root_id) = svg.root().value();
    for (name, value) in &root_tag.args {
        doc.assign(name, svg::node::Value::from(value.to_string()))
    }
    if let Some(id) = root_id {
        doc.assign("id", id.clone());
    }
    for child in &svg.root().children() {
        doc.append(build_element(child));
    }
    if !root_tag.text.is_empty() {
        doc.append(Text::new(&root_tag.text))
    }
    doc
}

pub fn print_svg(svg: &SVGWithIDs) -> String {
    let doc = build_doc(svg);
    doc.to_string()
}

pub fn print_svg_element<'a, ST: Subtree<Node = (&'a Tag, &'a Option<String>)>>(
    svg: &ST,
) -> String {
    let doc = build_element(svg);
    doc.to_string()
}
