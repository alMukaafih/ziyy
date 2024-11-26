//! Values
mod c;
mod x;

pub use c::C;
pub use x::X;

/// Bold Value.
pub const B: [u8; 1] = [1];
/// Italics Value.
pub const I: [u8; 1] = [3];
/// Underline Value.
pub const U: [u8; 1] = [4];
/// Strike through Value.
pub const S: [u8; 1] = [9];

pub(crate) const RESET: [u8; 1] = [0];
pub(crate) const RESET_B: &str = "\x1b[22m";
pub(crate) const RESET_I: &str = "\x1b[23m";
pub(crate) const RESET_U: &str = "\x1b[24m";
pub(crate) const RESET_S: &str = "\x1b[29m";
