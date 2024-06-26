use crate::value::*;

pub struct State {
    tags: Vec<String>,
    saves: Vec<String>,
}

impl State {
    pub fn new() -> State {
        State {
            tags: vec![String::new()],
            saves: vec![RESET.to_string()],
        }
    }

    pub fn push(&mut self, tag: &str, string: &str) {
        let l = self.saves.len() - 1;
        let mut s = self.saves.get(l).unwrap().clone();
        s.push_str(string);
        self.saves.push(s);
        self.tags.push(String::from(tag));
    }

    pub fn pop(&mut self) -> (Option<String>, Option<String>) {
        let a = self.tags.pop();
        let b = self.saves.pop();
        (a, b)
    }

    pub fn current_tag(&self) -> &str {
        let l = self.tags.len() - 1;
        self.tags.get(l).unwrap()
    }

    pub fn current_save(&self) -> &str {
        let l = self.saves.len() - 1;
        self.saves.get(l).unwrap()
    }
}
