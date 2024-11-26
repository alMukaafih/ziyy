#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![allow(dead_code)]
#![doc = include_str!("../../README.md")]
//! # Examples
//! ```
//! use std::collections::HashMap;
//! use std::io::stdout;
//!
//! use ziyy::Compiler;
//! use ziyy::value::C;
//!
//! let mut out = stdout();
//! let mut vars = HashMap::new();
//! vars.insert("green".to_string(), C::rgb(0, 150, 75));
//! vars.insert("cyan".to_string(), C::rgb(0, 150, 150));
//!
//! let mut compiler = Compiler::new(include_str!("../../help.zi"), &mut out, Some(vars));
//! assert!(compiler.compile().is_ok());
//!```
//! # Output


mod color;
mod compiler;
#[doc(hidden)]
pub mod scanner;
mod parser;
pub mod value;
#[cfg(feature = "source")]
#[cfg_attr(docsrs, doc(cfg(feature = "source")))]
pub use ziyy_proc::source;

pub use crate::parser::{Tag, TagKind, TagType};
pub use crate::compiler::{Compiler, Error};

/// Styles your text using escape sequence.
///
/// It takes in text that has been styled using recognised tags and returns the equivalent that it styles using escape sequences.
///
/// # Example
/// ```
/// use ziyy::style;
/// let text = style("<s><c.black>Black Text</c></s>");
/// assert!(text.is_ok());
/// println!("{}", text.unwrap());
/// ```
/// # Output
/// <pre style="color: black;"><del>Black Text</del></pre>
///
pub fn style<R: AsRef<str>>(text: R) -> Result<String> {
    let mut out: Vec<u8> = vec![];
    let mut compiler = Compiler::new(text.as_ref(), &mut out, None);
    compiler.compile()?;

    Ok(String::from_utf8(out)?)
}

/// Creates a new Template for styling text.
///
/// It takes in styling information and returns a
/// Clousue that can be used to style text using
/// the styling information.
///
/// # Example
/// ```
/// use ziyy::template;
/// let mut bred = template("<b><c.red>");
/// let text = bred("Bold Red Text");
/// assert!(text.is_ok());
/// println!("{}", text.unwrap());
/// ```
/// # Output
/// <pre style="color: red;"><del>Bold Red Text</del></pre>
///
pub fn template<R: AsRef<str>>(save: &str) -> impl for<'a> FnMut(R) -> Result<String> + '_ {
    let out: Vec<u8> = vec![];
    let mut compiler = Compiler::new(save.to_owned(), out, None);
    compiler.compile_template().unwrap();

    move |text: R| -> Result<String> {
        compiler.compile_rest(text)?;
        let out_copy = compiler.parser.out.clone();
        Ok(String::from_utf8(out_copy)?)
    }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
