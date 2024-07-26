#![warn(missing_docs)]
#![allow(dead_code)]
//! A Convenient Library for Styling Terminal Output.
//! # Example
//! ```
//! use ziyy::style;
//! let text = style("[b][c:yellow]Hello World!");
//! assert_eq!(text, "\x1b[1m\x1b[33mHello World!\x1b[0m")
//!```
//!

mod compiler;
#[doc(hidden)]
pub mod scanner;
pub mod value;


use std::{collections::HashMap, io::Write};

use value::C;
pub use crate::compiler::Compiler;

#[doc(hidden)]
pub fn compile(source: &str, out: &mut impl Write) {
    let mut vars = HashMap::new();
    vars.insert("green".to_string(), C::rgb(0, 150, 75));
    vars.insert("cyan".to_string(), C::rgb(0, 150, 150));
    let mut compiler = Compiler::new(source, out, vars);
    compiler.compile();
}

/// Styles your text using escape sequence.
///
/// It takes in text that has been styled using recognised tags and returns the equivalent that it styles using escape sequences.
/// It is a one to one relationship
///
/// # Example
/// ```
/// use ziyy::style;
/// let text = style("<s><c.black>Black Text");
/// assert_eq!(text, "\u{1b}[9m\u{1b}[30mBlack Text\u{1b}[0m")
/// ```
///
pub fn style(text: &str) -> String {
    let vars = HashMap::new();
    let mut out: Vec<u8> = vec![];
    let mut compiler = Compiler::new(text, &mut out, vars);
    compiler.compile();

    unsafe {String::from_utf8_unchecked(out)}
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
/// let bred = template("<b><c.red>");
/// let text = bred("Bold Red Text");
/// assert_eq!(text, "\x1b[1m\x1b[31mBold Red Text\x1b[0m")
/// ```
///
pub fn template(save: &str) -> impl for<'a> Fn(&'a str) -> String + '_ {
    move |text: &str| -> String { style(format!("{save}{text}").as_str()) }
}

#[test]
fn print() {
    //let r = template("[c:green] ");
    //let t: String = r("text");
    //assert_eq!("\x1b[32m text\u{1b}[0m", style("[c : green] text"));
    assert_eq!("\x1b[1m text\u{1b}[0m", style("[b] text"))
}
