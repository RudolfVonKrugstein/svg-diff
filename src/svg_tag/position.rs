#[derive(Debug, Clone)]
pub struct Position {
    pub parent_id: String,
    pub child_index: usize,
    pub prev_child: Option<String>,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.parent_id == other.parent_id && self.prev_child == other.prev_child
    }
}

impl Position {
    pub fn root() -> Position {
        Position {
            parent_id: "".to_string(),
            child_index: 0,
            prev_child: None,
        }
    }
}
