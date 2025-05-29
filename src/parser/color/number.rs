use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Number {
    U8(u8),
    PlaceHolder(String),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::U8(u) => u.fmt(f),
            Number::PlaceHolder(s) => s.fmt(f),
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::U8(value)
    }
}

impl From<String> for Number {
    fn from(value: String) -> Self {
        Number::PlaceHolder(value)
    }
}
