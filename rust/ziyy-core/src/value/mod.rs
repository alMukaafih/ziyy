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
