#![allow(missing_docs)]

use crate::scanner::position::Position;
pub type Value<T> = Option<Option<T>>;

#[derive(PartialEq, Debug, Clone)]
pub struct Tag {
    pub r#type: TagType,
    pub kind: TagKind,
    pub text: Option<String>,

    /* Attributes */
    pub b: Value<String>,
    pub c: Value<String>,
    pub i: Value<String>,
    pub n: Value<String>,
    pub s: Value<String>,
    pub u: Value<String>,
    pub x: Value<String>,

    /* Link */
    pub href: Value<String>,

    /* Color */
    pub color: Option<(String, Option<String>)>,

    /* Binding */
    pub name: Value<String>,

    /* Tab */
    pub tab: Value<String>,
    /* Value */
    pub val: Value<String>,

    pub(crate) start: Position,
    pub(crate) end: Position,
}

impl Tag {
    /// Creates new Tag
    pub fn new(kind: TagKind, r#type: TagType) -> Self {
        Self {
            r#type,
            kind,
            text: None,
            b: None,
            c: None,
            i: None,
            n: None,
            s: None,
            u: None,
            x: None,
            href: None,
            color: None,
            name: None,
            tab: None,
            val: None,

            start: Position::new(0, 0),
            end: Position::new(0, 0),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TagType {
    Open,
    Close,
    OpenAndClose,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TagKind {
    A,
    Any(String),
    B,
    Br,
    C,
    E,
    Eof,
    I,
    Let,
    None,
    P,
    S,
    Text,
    U,
    X,
    WhiteSpace,
    Ziyy,
}
