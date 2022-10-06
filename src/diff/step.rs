use serde::{Serialize, Deserialize};
use crate::diff::hashmap_diff::HashMapDiff;
use crate::flat_tree::FlatTreeNeighbors;
use crate::svg_data::{print_svg_element, SVGWithIDs};

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
    #[serde(rename="remove")]
    Remove(RemoveDiff),
    // Add
    #[serde(rename="add")]
    Add(AddDiff),
    // Change the properties of a tag
    #[serde(rename="change")]
    ChangeProperties(ChangePropertiesDiff),
    #[serde(rename="change_text")]
    ChangeText(ChangeTextDiff),
    #[serde(rename="move")]
    Move(MoveDiff),
}

impl DiffStep {
    pub fn remove(position: FlatTreeNeighbors<String>) -> DiffStep {
        DiffStep::Remove(RemoveDiff {
            id: position.me.unwrap(),
            parent_id: position.parent.unwrap(),
            prev_child_id: position.prev_sibling,
            next_child_id: position.next_sibling
        })
    }

    pub fn add(svg: &SVGWithIDs, position: FlatTreeNeighbors<String>) -> DiffStep {
        DiffStep::Add(AddDiff {
            svg: print_svg_element(svg),
            id: position.me.unwrap(),
            parent_id: position.parent.unwrap(),
            prev_child_id: position.prev_sibling,
            next_child_id: position.next_sibling
        })
    }

    pub fn change(id: String, change: HashMapDiff<String>) -> DiffStep {
        let adds = change.adds.iter().map(
            |(prop, val)| Property {
                prop: prop.clone(),
                value: val.to_string(),
            }
        ).collect();
        let removes = change.deletes.iter().map(
            |(prop, val)| Property {
                prop: prop.clone(),
                value: val.to_string(),
            }
        ).collect();
        let changes = change.changes.iter().map(
            |(prop, (from, to))| ChangedProperty {
                prop: prop.clone(),
                start: from.to_string(),
                end: to.to_string(),
            }
        ).collect();
        DiffStep::ChangeProperties(
            ChangePropertiesDiff {
                id,
                adds,
                removes,
                changes
            })
    }

    pub fn text_change(id: String, new_text: String) -> DiffStep {
        DiffStep::ChangeText(ChangeTextDiff{id, new_text})
    }

    pub fn move_element(id: String, new_position: FlatTreeNeighbors<String>) -> DiffStep {
        DiffStep::Move(
            MoveDiff {
                id,
                new_parent_id: new_position.parent.unwrap(),
                new_next_child_id: new_position.next_sibling,
                new_prev_child_id: new_position.prev_sibling,
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
