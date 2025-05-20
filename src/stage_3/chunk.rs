use std::fmt::Display;

use super::tag_parer::tag::Tag;

#[derive(Debug, Clone)]
pub enum Chunk {
    Tag(Tag),
    WhiteSpace(String),
    Word(String),
}

impl Chunk {
    pub fn new_tag(tag: Tag) -> Self {
        Self::Tag(tag)
    }

    pub fn new_word(word: String) -> Self {
        Self::Word(word)
    }

    pub fn new_ws(ws: String) -> Self {
        Self::WhiteSpace(ws)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chunk::Tag(tag) => tag.fmt(f),
            Chunk::WhiteSpace(ws) => ws.fmt(f),
            Chunk::Word(word) => word.fmt(f),
        }
    }
}
