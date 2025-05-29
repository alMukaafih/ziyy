use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::common::Span;

use super::tag_parer::tag::Tag;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChunkData {
    Tag(Tag),
    WhiteSpace(String),
    Word(String),
}

impl ChunkData {
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
        matches!(self, ChunkData::Tag(_))
    }

    pub fn is_tag_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Tag) -> bool,
    {
        match self {
            ChunkData::Tag(tag) => f(tag),
            ChunkData::WhiteSpace(_) => false,
            ChunkData::Word(_) => false,
        }
    }

    pub fn is_word(&self) -> bool {
        matches!(self, ChunkData::Word(_))
    }

    pub fn is_ws(&self) -> bool {
        matches!(self, ChunkData::WhiteSpace(_))
    }

    pub fn tag(&self) -> Option<&Tag> {
        if let ChunkData::Tag(tag) = self {
            Some(tag)
        } else {
            None
        }
    }

    pub fn word(&self) -> Option<&String> {
        if let ChunkData::Word(word) = self {
            Some(word)
        } else {
            None
        }
    }

    pub fn ws(&self) -> Option<&String> {
        if let ChunkData::WhiteSpace(ws) = self {
            Some(ws)
        } else {
            None
        }
    }

    pub fn tag_mut(&mut self) -> Option<&mut Tag> {
        if let ChunkData::Tag(tag) = self {
            Some(tag)
        } else {
            None
        }
    }
}

impl Display for ChunkData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self {
                ChunkData::Tag(tag) => f.write_fmt(format_args!("{tag:#}")),
                ChunkData::WhiteSpace(ws) => f.write_fmt(format_args!("{ws:?}")),
                ChunkData::Word(word) => word.fmt(f),
            }
        } else {
            match self {
                ChunkData::Tag(tag) => tag.fmt(f),
                ChunkData::WhiteSpace(ws) => ws.fmt(f),
                ChunkData::Word(word) => word.fmt(f),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub data: ChunkData,
    pub span: Span,
}

impl Deref for Chunk {
    type Target = ChunkData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Chunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
