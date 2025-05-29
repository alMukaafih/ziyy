use crate::error::{Error, ErrorType};
use crate::{scanner::GenericScanner, splitter::fragment::Fragment};
use scanner::Scanner;
use std::collections::VecDeque;
use tag::{Tag, TagType};
use token::{Token, TokenType::*};

use super::color::Color;

mod scanner;
pub mod tag;
mod token;

macro_rules! consume_declaration {
    ( $tag:expr, $next:expr, $token:expr ) => {{
        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagPropertyValue)?;
            $token = $next()?;
        }
    }};
}

macro_rules! assign_prop {
    ( $tag:expr, $set_prop:tt, $next:expr, $token:expr ) => {{
        $tag.$set_prop(String::new());

        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagPropertyValue)?;
            $tag.$set_prop($token.literal.unwrap());
            $token = $next()?;
        }
    }};
}

macro_rules! assign_prop_color {
    ( $tag:expr, $set_prop:tt, $next:expr, $token:expr, $pre:expr ) => {{
        $tag.$set_prop(String::new());

        $token = $next()?;
        if $token.r#type == EQUAL {
            $token = $next()?;
            expect(&$token, STRING, ErrorType::InvalidTagPropertyValue)?;
            let color: Color = format!("{}{}", $pre, $token.literal.unwrap()).try_into()?;
            $tag.$set_prop(color.into());
            $token = $next()?;
        }
    }};
}

macro_rules! assign_prop_switch {
    ( $tag:expr, $set_prop:tt, $next:expr, $token:expr ) => {{
        $tag.$set_prop(true);

        consume_declaration!($tag, $next, $token);
    }};
}

pub struct TagParser {
    stack: Vec<Tag>,
}

impl Default for TagParser {
    fn default() -> Self {
        Self::new()
    }
}

impl TagParser {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(8),
        }
    }

    pub fn parse(&mut self, source: Fragment) -> Result<Tag, Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let open = &tokens[0].r#type;
        let close = &tokens[tokens.len() - 1].r#type;

        let tag_type = match (open, close) {
            (LESS, GREATER) => TagType::Open,
            (LESS, SLASH_GREATER) => TagType::SelfClose,
            (LESS_SLASH, GREATER) => TagType::Close,
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidTag,
                    format!("Invalid tag: {:?} {:?}", open, close),
                    tokens[0].line,
                    0, // TODO: column
                ));
            }
        };
        let line = tokens[0].line;
        let mut tokens: VecDeque<_> = tokens[1..tokens.len()].to_vec().into();

        let mut next = || {
            if tokens.is_empty() {
                return Err(Error::new(
                    ErrorType::InvalidTag,
                    "Unexpected end of input".to_string(),
                    line, // TODO: line
                    0,    // TODO: column
                ));
            }

            Ok(tokens.pop_front().unwrap())
        };

        let token = next()?;
        expect(&token, IDENTIFIER, ErrorType::InvalidTagName)?;

        let mut tag = Tag::default();
        tag.r#type = tag_type;
        tag.set_name(token.lexeme.clone());

        match tag.name().as_str() {
            "b" | "strong" => {
                tag.set_bold(true);
            }
            "d" | "dim" => {
                tag.set_dim(true);
            }
            "i" => {
                tag.set_italics(true);
            }
            "u" | "ins" => {
                tag.set_under(true);
            }
            "k" | "blink" => {
                tag.set_blink(true);
            }
            "r" => {
                tag.set_negative(true);
            }
            "h" => {
                tag.set_hidden(true);
            }
            "s" | "del" => {
                tag.set_strike(true);
            }
            _ => {}
        }

        let mut token = next()?;
        while token.r#type == IDENTIFIER {
            match token.lexeme.as_str() {
                "b" | "bold" => assign_prop_switch!(tag, set_bold, next, token),
                "d" | "dim" => assign_prop_switch!(tag, set_dim, next, token),
                "k" | "blink" => assign_prop_switch!(tag, set_blink, next, token),
                "h" | "hidden" | "hide" | "invisible" => {
                    assign_prop_switch!(tag, set_hidden, next, token)
                }
                "s" | "strike" => assign_prop_switch!(tag, set_strike, next, token),
                "i" | "italics" => assign_prop_switch!(tag, set_italics, next, token),
                "invert" => assign_prop_switch!(tag, set_negative, next, token),
                "u" | "underline" => assign_prop_switch!(tag, set_under, next, token),
                "uu" | "double-underline" => {
                    assign_prop_switch!(tag, set_double_under, next, token)
                }

                "c" | "fg" => assign_prop_color!(tag, set_fg_color, next, token, "fg_"),

                "x" | "bg" => assign_prop_color!(tag, set_bg_color, next, token, "bg_"),

                "black" | "blue" | "cyan" | "default" | "green" | "magenta" | "red" | "white"
                | "yellow" => {
                    if tag.name() == "c" {
                        let color: Color = format!("fg_{}", token.lexeme).try_into()?;
                        tag.set_fg_color(color.into());
                    } else if tag.name() == "x" {
                        let color: Color = format!("bg_{}", token.lexeme).try_into()?;
                        tag.set_bg_color(color.into());
                    }

                    consume_declaration!(tag, next, token);
                }
                "byte" => {
                    token = next()?;
                    if token.r#type == EQUAL {
                        token = next()?;
                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                        token = next()?;
                    }

                    let color = format!("byte({})", token.literal.unwrap());

                    if tag.name() == "c" {
                        let color: Color = format!("fg_{color}").try_into()?;
                        tag.set_fg_color(color.into());
                    } else if tag.name() == "x" {
                        let color: Color = format!("bg_{color}").try_into()?;
                        tag.set_bg_color(color.into());
                    }
                }
                "rgb" => {
                    token = next()?;
                    if token.r#type == EQUAL {
                        token = next()?;
                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                        token = next()?;
                    }

                    let color = format!("rgb({})", token.literal.unwrap());

                    if tag.name() == "c" {
                        let color: Color = format!("fg_{color}").try_into()?;
                        tag.set_fg_color(color.into());
                    } else if tag.name() == "x" {
                        let color: Color = format!("bg_{color}").try_into()?;
                        tag.set_bg_color(color.into());
                    }
                }
                "n" => {
                    if tag.name() == "br" {
                        assign_prop!(tag, set_custom, next, token);
                    }
                }
                "href" => {
                    if tag.name() == "a" {
                        assign_prop!(tag, set_custom, next, token);
                    }
                }
                "name" => {
                    if tag.name() == "let" {
                        assign_prop!(tag, set_custom, next, token);
                    }
                }
                "tab" => {
                    if tag.name() == "p" {
                        assign_prop!(tag, set_custom, next, token);
                    }
                }

                "src" => assign_prop!(tag, set_src, next, token),

                _ => {
                    consume_declaration!(tag, next, token);
                }
            }
        }

        match tag.r#type {
            TagType::Open => {
                self.stack.push(tag.clone());
            }
            TagType::SelfClose => {}
            TagType::Close => {
                if let Some(last) = self.stack.pop() {
                    if last.name() != tag.name() {
                        return Err(Error::new(
                            ErrorType::InvalidTag,
                            format!("Mismatched tag: {:?} {:?}", last.name(), tag.name()),
                            token.line,
                            0, // TODO: column
                        ));
                    }
                }
            }
        }

        Ok(tag)
    }
}

fn expect(token: &Token, expected: token::TokenType, error: ErrorType) -> Result<(), Error> {
    if token.r#type == expected {
        Ok(())
    } else {
        Err(Error::new(
            error,
            format!("Expected {:?}, but found {:?}", expected, token.r#type),
            token.line,
            0, // TODO: column
        ))
    }
}
