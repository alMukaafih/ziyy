use crate::color::ansi_style::AnsiStyle;

use super::tag::TagKind;

pub struct State {
    tags: Vec<TagKind>,
    saves: Vec<AnsiStyle>,
}

impl State {
    pub fn new() -> State {
        State {
            tags: vec![TagKind::Ziyy],
            saves: vec![AnsiStyle { inner: vec![0] }],
        }
    }

    pub fn push(&mut self, tag: TagKind, style: AnsiStyle) {
        let l = self.saves.len() - 1;
        let mut ansi = self.saves.get(l).unwrap().clone();
        ansi.push(&style);
        self.saves.push(ansi);
        self.tags.push(tag);
    }

    pub fn pop(&mut self) -> (Option<TagKind>, Option<AnsiStyle>) {
        let a = self.tags.pop();
        let b = self.saves.pop();
        (a, b)
    }

    pub fn current_tag(&self) -> Option<&TagKind> {
        let l = self.tags.len() - 1;
        self.tags.get(l)
    }

    pub fn current_save(&self) -> Option<&AnsiStyle> {
        let l = self.saves.len() - 1;
        self.saves.get(l)
    }

    pub fn previous_save(&self) -> Option<&AnsiStyle> {
        let l = self.saves.len() - 2;
        self.saves.get(l)
    }
}
