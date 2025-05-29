use std::fmt::Display;

use super::tag_parer::tag::Tag;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn is_tag(&self) -> bool {
        matches!(self, Chunk::Tag(_))
    }

    pub fn is_tag_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Tag) -> bool,
    {
        match self {
            Chunk::Tag(tag) => f(tag),
            Chunk::WhiteSpace(_) => false,
            Chunk::Word(_) => false,
        }
    }

    pub fn is_word(&self) -> bool {
        matches!(self, Chunk::Word(_))
    }

    pub fn is_ws(&self) -> bool {
        matches!(self, Chunk::WhiteSpace(_))
    }

    pub fn tag(&self) -> Option<&Tag> {
        if let Chunk::Tag(tag) = self {
            Some(tag)
        } else {
            None
        }
    }

    pub fn word(&self) -> Option<&String> {
        if let Chunk::Word(word) = self {
            Some(word)
        } else {
            None
        }
    }

    pub fn ws(&self) -> Option<&String> {
        if let Chunk::WhiteSpace(ws) = self {
            Some(ws)
        } else {
            None
        }
    }

    pub fn tag_mut(&mut self) -> Option<&mut Tag> {
        if let Chunk::Tag(tag) = self {
            Some(tag)
        } else {
            None
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                Chunk::Tag(tag) => f.write_fmt(format_args!("{tag:#}")),
                Chunk::WhiteSpace(ws) => f.write_fmt(format_args!("{ws:?}")),
                Chunk::Word(word) => word.fmt(f),
            }
        } else {
            match self {
                Chunk::Tag(tag) => tag.fmt(f),
                Chunk::WhiteSpace(ws) => ws.fmt(f),
                Chunk::Word(word) => word.fmt(f),
            }
        }
    }
}
