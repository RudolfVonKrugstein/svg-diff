use random_string::generate;

pub struct MatchingIdGenerator {
    prefix: String,
    next_index: u64,
}

impl MatchingIdGenerator {
    pub fn new() -> MatchingIdGenerator {
        MatchingIdGenerator {
            prefix: generate(8, "abcdefghijklmnopqrstuvwxyz"),
            next_index: 0
        }
    }

    pub fn next(&mut self, default_id: Option<String>) -> String {
        if let Some(pre_id) = default_id {
           pre_id
        }
        else {
            let res = format!("{}-{}", self.prefix, self.next_index);
            self.next_index = self.next_index + 1;
            res
        }
    }
}
