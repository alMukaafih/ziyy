//! Values
mod c;
mod x;

pub use c::C;
pub use x::X;

/// Bold Value.
pub const B: &'static str = "\x1b[1m";
/// Italics Value.
pub const I: &'static str = "\x1b[3m";
/// Underline Value.
pub const U: &'static str = "\x1b[4m";
/// Strike through Value.
pub const S: &'static str = "\x1b[9m";

pub(crate) const RESET: &'static str   = "\x1b[0m";
pub(crate) const RESET_B: &'static str = "\x1b[22m";
pub(crate) const RESET_I: &'static str = "\x1b[23m";
pub(crate) const RESET_U: &'static str = "\x1b[24m";
pub(crate) const RESET_S: &'static str = "\x1b[29m";
pub(crate) const RESET_C: &'static str = "\x1b[39m";
pub(crate) const RESET_X: &'static str = "\x1b[49m";
