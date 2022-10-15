use crate::diff::hashmap_diff::HashMapDiff;
use crate::svg_data::{print_svg_element, SVGWithIDsSubtree};
use flange_flat_tree::{Subtree, Tree};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveDiff {
    id: String,
    parent_id: String,
    prev_child_id: Option<String>,
    next_child_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddDiff {
    svg: String,
    id: String,
    parent_id: String,
    prev_child_id: Option<String>,
    next_child_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoveDiff {
    id: String,
    new_parent_id: String,
    new_prev_child_id: Option<String>,
    new_next_child_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangedProperty {
    prop: String,
    start: String,
    end: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Property {
    prop: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangePropertiesDiff {
    id: String,
    adds: Vec<Property>,
    removes: Vec<Property>,
    changes: Vec<ChangedProperty>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangeTextDiff {
    id: String,
    new_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum DiffStep {
    // Remove a node
    #[serde(rename = "remove")]
    Remove(RemoveDiff),
    // Add
    #[serde(rename = "add")]
    Add(AddDiff),
    // Change the properties of a tag
    #[serde(rename = "change")]
    ChangeProperties(ChangePropertiesDiff),
    #[serde(rename = "change_text")]
    ChangeText(ChangeTextDiff),
    #[serde(rename = "move")]
    Move(MoveDiff),
}

impl DiffStep {
    pub fn remove(svg: &SVGWithIDsSubtree) -> DiffStep {
        DiffStep::Remove(RemoveDiff {
            id: svg.value().1.clone().unwrap(),
            parent_id: svg.parent().and_then(|s| s.value().1.clone()).unwrap(),
            prev_child_id: svg.prev_sibling().and_then(|s| s.value().1.clone()),
            next_child_id: svg.next_sibling().and_then(|s| s.value().1.clone()),
        })
    }

    pub fn add(svg: &SVGWithIDsSubtree) -> DiffStep {
        DiffStep::Add(AddDiff {
            svg: print_svg_element(svg),
            id: svg.value().1.clone().unwrap(),
            parent_id: svg.parent().and_then(|s| s.value().1.clone()).unwrap(),
            prev_child_id: svg.prev_sibling().and_then(|s| s.value().1.clone()),
            next_child_id: svg.next_sibling().and_then(|s| s.value().1.clone()),
        })
    }

    pub fn change(id: String, change: HashMapDiff<String>) -> DiffStep {
        let adds = change
            .adds
            .iter()
            .map(|(prop, val)| Property {
                prop: prop.clone(),
                value: val.to_string(),
            })
            .collect();
        let removes = change
            .deletes
            .iter()
            .map(|(prop, val)| Property {
                prop: prop.clone(),
                value: val.to_string(),
            })
            .collect();
        let changes = change
            .changes
            .iter()
            .map(|(prop, (from, to))| ChangedProperty {
                prop: prop.clone(),
                start: from.to_string(),
                end: to.to_string(),
            })
            .collect();
        DiffStep::ChangeProperties(ChangePropertiesDiff {
            id,
            adds,
            removes,
            changes,
        })
    }

    pub fn text_change(id: String, new_text: String) -> DiffStep {
        DiffStep::ChangeText(ChangeTextDiff { id, new_text })
    }

    pub fn move_element(svg: &SVGWithIDsSubtree) -> DiffStep {
        DiffStep::Move(MoveDiff {
            id: svg.value().1.clone().unwrap(),
            new_parent_id: svg.parent().and_then(|s| s.value().1.clone()).unwrap(),
            new_prev_child_id: svg.prev_sibling().and_then(|s| s.value().1.clone()),
            new_next_child_id: svg.next_sibling().and_then(|s| s.value().1.clone()),
        })
    }

    pub fn is_add(&self) -> bool {
        match *self {
            DiffStep::Add(_) => true,
            _ => false,
        }
    }

    pub fn is_remove(&self) -> bool {
        match *self {
            DiffStep::Remove(_) => true,
            _ => false,
        }
    }

    pub fn is_change(&self) -> bool {
        match *self {
            DiffStep::ChangeProperties(_) => true,
            _ => false,
        }
    }

    pub fn is_text_change(&self) -> bool {
        match *self {
            DiffStep::ChangeText(_) => true,
            _ => false,
        }
    }

    pub fn is_move(&self) -> bool {
        match *self {
            DiffStep::Move(_) => true,
            _ => false,
        }
    }
}
