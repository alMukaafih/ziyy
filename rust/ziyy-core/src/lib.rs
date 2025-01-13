#![warn(missing_docs)]
#![warn(rustdoc::private_intra_doc_links)]
#![warn(unconditional_panic)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![forbid(unsafe_code)]
#![doc = include_str!("../../../README.md")]
//! # Examples
//! ```
//! use std::collections::HashMap;
//!
//! use ziyy::Parser;
//!
//! let mut parser = Parser::new("This is Some <c magenta u b>Magenta Underlined Bold Text</c>", None);
//! assert!(parser.parse().is_ok());
//!```
//! # Result
//! <pre>This is Some <span style="color: magenta;"><b><u>Magenta Underlined Bold Text</u></b></span></pre>
//!

pub use crate::error::{Error, ErrorKind};
pub use crate::parser::{Parser, Tag, TagKind, TagType};
pub use crate::scanner::token::TokenKind;

mod color;
mod error;
mod num;
mod parser;
#[doc(hidden)]
pub mod scanner;
pub mod value;

/// Styles your text
///
/// # Example
/// ```
/// use ziyy::style;
/// let text = style("<s c='black'>Black Text</s>");
/// assert!(text.is_ok());
/// println!("{}", text.unwrap());
/// ```
/// # Output
/// <pre style="color: black;"><s>Striked Through Black Text</s></pre>
///
pub fn style<T: AsRef<str>>(text: T) -> String {
    let mut parser = Parser::new(text.as_ref(), None);
    parser.parse().unwrap()
}

/// Creates a new Template for styling text.
///
/// It takes in styling information and returns a
/// Clousue that can be used to style text using
/// the styling information.
///
/// # Example
/// ```
/// use ziyy::prepare;
/// let bred = prepare("<b><c red>");
/// let text = bred("Bold Red Text");
/// assert!(text.is_ok());
/// println!("{}", text.unwrap());
/// ```
/// # Output
/// <pre style="color: red;"><b>Bold Red Text</b></pre>
///
pub fn prepare<T: AsRef<str>>(save: T) -> impl for<'a> FnMut(T) -> String {
    move |text: T| -> String { style(format!("{}{}", save.as_ref(), text.as_ref())) }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
