#[repr(u8)]
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum Bit4 {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Header,
    #[default]
    Default,
}

impl From<u8> for Bit4 {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::Red,
            2 => Self::Green,
            3 => Self::Yellow,
            4 => Self::Blue,
            5 => Self::Magenta,
            6 => Self::Cyan,
            7 => Self::White,
            8 => Self::Header,
            _ => Self::Default,
        }
    }
}
