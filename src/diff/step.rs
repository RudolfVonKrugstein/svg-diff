

use crate::diff::hashmap_diff::HashMapDiff;
use crate::svg_tag::{print_svg_element, Position, SVGTag};
use crate::errors::*;

#[derive(Debug)]
pub struct RemoveDiff {
    id: String,
}

#[derive(Debug)]
pub struct AddDiff {
    id: String,
    tag: SVGTag,
    position: Position,
}

#[derive(Debug)]
pub struct MoveDiff {
    id: String,
    new_position: Position,
}

#[derive(Debug)]
pub struct ChangePropertiesDiff {
    id: String,
    change: HashMapDiff<String>,
}

#[derive(Debug)]
pub struct ChangeTextDiff {
    id: String,
    new_text: String,
}

#[derive(Debug)]
pub enum DiffStep {
    // Remove a node
    Remove(RemoveDiff),
    // Add
    Add(AddDiff),
    // Change the properties of a tag
    ChangeProperties(ChangePropertiesDiff),
    ChangeText(ChangeTextDiff),
    Move(MoveDiff),
}

impl DiffStep {
    pub fn remove(tag: &SVGTag) -> DiffStep {
        DiffStep::Remove(RemoveDiff {
            id: tag.matching.as_ref().unwrap().get_id(),
        })
    }

    pub fn add(tag: SVGTag, position: Position) -> DiffStep {
        DiffStep::Add(AddDiff {
            id: tag.matching.as_ref().unwrap().get_id(),
            tag,
            position,
        })
    }

    pub fn change(id: String, change: HashMapDiff<String>) -> DiffStep {
        DiffStep::ChangeProperties(ChangePropertiesDiff { id, change })
    }

    pub fn text_change(id: String, new_text: String) -> DiffStep {
        DiffStep::ChangeText(ChangeTextDiff{id, new_text})
    }

    pub fn move_element(id: String, new_position: Position) -> DiffStep {
        DiffStep::Move(MoveDiff{id, new_position})
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

    fn json_diff_from_step(s: &DiffStep) -> Result<JsonDiff> {
        Ok(match s {
            DiffStep::Remove(rdiff) => JsonDiff::Remove(JsonRemoveDiff {
                id: rdiff.id.clone(),
            }),
            DiffStep::Add(adiff) => JsonDiff::Add(
                JsonAddDiff {
                    svg: print_svg_element(&adiff.tag),
                    id: adiff.tag.matching.as_ref().unwrap().get_id(),
                    parent_id: adiff.position.parent_id.clone(),
                    prev_child_id: adiff.position.prev_child.clone(),
                    next_child_id: adiff.position.next_child.clone(),
                }
            ),
            DiffStep::ChangeProperties(cdiff) => JsonDiff::Change(
                JsonChangeDiff {
                    id: cdiff.id.clone(),
                    adds: cdiff.change.adds.iter().map(
                        |(prop, value)| JsonPropWithValue {
                            prop: prop.clone(),
                            value: value.clone()
                        }
                    ).collect(),
                    changes: cdiff.change.changes.iter().map(
                        |(prop, value)| JsonPropWithValue {
                            prop: prop.clone(),
                            value: value.1.clone()
                        }
                    ).collect(),
                    removes: cdiff.change.deletes.iter().map(
                        |(prop, _value)| prop.clone()
                    ).collect(),
                }
            ),
            DiffStep::ChangeText(tdiff) => JsonDiff::ChangeText(
                JsonChangeTextDiff {
                    id: tdiff.id.clone(),
                    new_text: tdiff.new_text.clone(),
                }
            ),
            DiffStep::Move(mdiff) => JsonDiff::Move(
                JsonMoveDiff {
                    id: mdiff.id.clone(),
                    new_parent_id: mdiff.new_position.parent_id.clone(),
                    new_prev_child_id: mdiff.new_position.prev_child.clone(),
                    new_next_child_id: mdiff.new_position.next_child.clone(),
                }
            )
        })
    }

    pub fn write_json(data: &Vec<DiffStep>) -> Result<Vec<JsonDiff>> {
        // Create the json structure
        let json_structure: Vec<JsonDiff> = data.iter().map(
            |s| DiffStep::json_diff_from_step(s)
        ).collect::<Result<Vec<JsonDiff>>>()?;

        // And print it!
        Ok(json_structure)
    }
}

use serde::{Serialize, Deserialize};



use crate::svg_tag::attributes::SVGAttValue;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonAddDiff {
    svg: String,
    id: String,
    parent_id: String,
    prev_child_id: Option<String>,
    next_child_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRemoveDiff {
    id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonMoveDiff {
    id: String,
    new_parent_id: String,
    new_prev_child_id: Option<String>,
    new_next_child_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonPropWithValue {
    prop: String,
    value: SVGAttValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonChangeDiff {
    id: String,
    adds: Vec<JsonPropWithValue>,
    changes: Vec<JsonPropWithValue>,
    removes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonChangeTextDiff {
    id: String,
    new_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum JsonDiff {
    #[serde(rename="add")]
    Add(JsonAddDiff),
    #[serde(rename="remove")]
    Remove(JsonRemoveDiff),
    #[serde(rename="move")]
    Move(JsonMoveDiff),
    #[serde(rename="change")]
    Change(JsonChangeDiff),
    #[serde(rename="change_text")]
    ChangeText(JsonChangeTextDiff),
}
