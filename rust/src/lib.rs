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

mod scanner;

use scanner::Scanner;
use crate::scanner::token::*;

#[allow(dead_code)]
struct Color {
    first_digit: u8,
    color: String
}
#[allow(dead_code)]
impl Color {
    fn new(first_digit: u8) -> Color {
        Color { first_digit: first_digit, color: String::new() }
    }
    fn escape(&self, second_digit: u8) -> String {
        let first_digit = self.first_digit;
        format!("\x1b[{first_digit}{second_digit}m").to_string()
    }
    fn escape_str(&self, second_digit: String) -> String {
        let first_digit = self.first_digit;
        format!("\x1b[{first_digit}8;2;{second_digit}m").to_string()
    }
    fn color_value(&mut self, color: &str) -> String {
        if color.starts_with("rgb(") && color.ends_with(")") {
            let rgb = color.get(4..(color.len()-1)).unwrap().to_string();
            let rgb = rgb.replace(",", ";");
            self.color = self.escape_str(rgb);
            return self.color.clone()
        }
        match color {
            "black" => {
                self.color = self.escape(0)
            }
            "red" => {
                self.color = self.escape(1)
            }
            "green" => {
                self.color = self.escape(2)
            }
            "yellow" => {
                self.color = self.escape(3)
            }
            "blue" => {
                self.color = self.escape(4)
            }
            "magenta" => {
                self.color = self.escape(5)
            }
            "cyan" => {
                self.color = self.escape(6)
            }
            "white" => {
                self.color = self.escape(7)
            } 
            value => {
                panic!("Unrecognised color: {value}")
            }
        }
        self.color.clone()
    }
    fn substitute(&self, text: &mut String, tag: String) -> String {
        text.replace(&tag, &self.color)
    }
}

struct Parser {
    result: String,
    tags: Vec<String>,
    open: bool,
    esc: bool,
}
impl Parser {
    fn parse(&mut self, text: String) {
        let mut tag = String::new();
        let _: Vec<_> = text.chars().map(|x| {
            if x == '\\' && !self.esc {
                self.esc = true;
            }
            else if self.esc {
                self.result.push(x);
                self.esc = false
            }
            else if x == '[' {
                self.open = true;
                self.result.push(x);
                tag.push(x);
            }
            else if x == ']' {
                self.open = false;
                self.result.push(x);
                tag.push(x);
                if !self.tags.contains(&tag) {
                    self.tags.push(tag.clone());
                }
                tag = String::new();
            }
            else if self.open && !x.is_whitespace()  {
                self.result.push(x);
                tag.push(x);
            }
            else if self.open && x.is_whitespace() {}
            else {
                self.result.push(x);
            }
            x
        }).collect();
    }
    fn new() -> Parser {
        Parser {
            result: String::new(),
            tags: Vec::new(),
            open: false,
            esc: false,
        }
    }
}

#[doc(hidden)]
pub fn compile(source: String) -> String {
    let mut scanner = Scanner::new(source);
    let mut line = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        print!("{:2} '{}'\n", token.kind.as_u8(), token.content);
        if token.kind == TokenKind::Eof { break };
    }
    
    String::new()
}

/// Styles your text using escape sequence.
/// 
/// It takes in text that has been styled using recognised tags and returns the equivalent that it styles using escape sequences.
/// It is a one to one relationship
/// 
/// # Example
/// ```
/// use ziyy::style;
/// let text = style("[s][c:black]Black Text");
/// assert_eq!(text, "\x1b[9m\x1b[30mBlack Text\x1b[0m")
/// ```
/// 
pub fn style(text: &str) -> String {
    // initialize fg and bg
    let text = String::from(text);
    let mut fg = Color::new(3);
    let mut bg = Color::new(4);
    const RESET: &str = "\x1b[0m";
    
    let mut p = Parser::new();
    p.parse(text);
    let mut text = String::from(p.result);
    for mut tag in p.tags {
        let len = tag.len();
        if tag.starts_with("[c:") {
            let value = tag.get_mut(3..(len-1)).unwrap();
            fg.color_value(value);
            text = fg.substitute(&mut text, tag);
        }
        else if tag.starts_with("[x:") {
            let value = tag.get_mut(3..(len-1)).unwrap();
            bg.color_value(value);
            text = bg.substitute(&mut text, tag);
        }
        // Bold
        else if text.contains("[b]") {
            text = text.replace("[b]", "\x1b[1m");
        }
        // Remove Bold
        else if tag == "[/b]" {
            text = text.replace("[/b]", "\x1b[22m");
        }
        
        // Italics
        else if tag == "[i]" {
            text = text.replace("[i]", "\x1b[3m");
        }
        // Remove italics
        else if tag == "[/i]" {
            text = text.replace("[/i]", "\x1b[23m");
        }
        
        // Remove colors
        else if tag == "[/c]" {
            text = text.replace("[/c]", "\x1b[39m");
        }
        else if tag == "[/x]" {
            text = text.replace("[/x]", "\x1b[49m");
        }
        // Underline
        else if tag == "[u]" {
            text = text.replace("[u]", "\x1b[4m");
        }
        else if tag == "[/u]" {
            text = text.replace("[/u]", "\x1b[24m");
        }
    
        // Strike through
        else if tag == "[s]" {
            text = text.replace("[s]", "\x1b[9m");
    
        }
        else if tag == "[/s]" {
            text = text.replace("[/s]/", "\x1b[29m");
        }
        else if tag == "[/0]" {
            text = text.replace("[/0]", RESET);
        }
    }
    format!("{text}{RESET}")
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
/// let bred = template("[b][c:red]");
/// let text = bred("Bold Red Text");
/// assert_eq!(text, "\x1b[1m\x1b[31mBold Red Text\x1b[0m")
/// ```
/// 
pub fn template(save: &str) -> impl for<'a> Fn(&'a str) -> String + '_ {
    move |text: &str| -> String {
        style(format!("{save}{text}").as_str())
    }
}

#[test]
fn print() {
    //let r = template("[c:green] ");
    //let t: String = r("text");
    //assert_eq!("\x1b[32m text\u{1b}[0m", style("[c : green] text"));
    assert_eq!("\x1b[1m text\u{1b}[0m", style("[b] text"))
    
}