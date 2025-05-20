use crate::error::{Error, ErrorType};
use crate::scanner::GenericScanner;
pub use number::Number;
use scanner::Scanner;
use std::collections::VecDeque;
use std::fmt::Display;
use token::Token;
use token::TokenType::*;

mod number;
mod scanner;
mod token;

macro_rules! number {
    ( $token:expr ) => {
        match $token.r#type {
            NUMBER => $token.literal.unwrap().into(),
            PLACE_HOLDER => $token.lexeme.clone().into(),
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidNumber,
                    format!("{:?} is not a valid number", $token.lexeme),
                    $token.line,
                    0, // TODO: column
                ));
            }
        }
    };
}

macro_rules! hex {
    ( $str:expr ) => {
        u8::from_str_radix($str, 16).unwrap().into()
    };
}

#[derive(Clone, Debug)]
pub struct Color(String);

impl Color {
    fn rgb(mut next: impl FnMut() -> Result<Token, Error>, n: u8) -> Result<Color, Error> {
        let token = next()?;

        expect(&token, LEFT_PAREN, ErrorType::UnexpectedToken)?;

        let token = next()?;
        let mut r: Number = Number::U8(0);
        let mut g: Number = Number::U8(0);
        let mut b: Number = Number::U8(0);

        match token.r#type {
            NUMBER | PLACE_HOLDER => {
                r = number!(token);

                let token = next()?;
                expect(&token, COMMA, ErrorType::UnexpectedToken)?;

                let token = next()?;
                g = number!(token);

                let token = next()?;
                expect(&token, COMMA, ErrorType::UnexpectedToken)?;

                let token = next()?;
                b = number!(token);
            }
            HEX => match token.lexeme.len() {
                4 => {
                    r = hex!(&token.lexeme[1..2].repeat(2));
                    g = hex!(&token.lexeme[2..3].repeat(2));
                    b = hex!(&token.lexeme[3..4].repeat(2));
                }
                7 => {
                    r = hex!(&token.lexeme[1..3]);
                    g = hex!(&token.lexeme[3..5]);
                    b = hex!(&token.lexeme[5..7]);
                }
                _ => {}
            },
            _ => {
                return Err(Error::new(
                    ErrorType::InvalidNumber,
                    format!("{:?} is not a valid number", token.lexeme),
                    token.line,
                    0, // TODO: column
                ));
            }
        }

        let token = next()?;

        expect(&token, RIGHT_PAREN, ErrorType::UnexpectedToken)?;

        Ok(Color(format!("{n};2;{r};{g};{b};")))
    }

    fn byte(mut next: impl FnMut() -> Result<Token, Error>, n: u8) -> Result<Color, Error> {
        let token = next()?;
        expect(&token, LEFT_PAREN, ErrorType::UnexpectedToken)?;

        let token = next()?;
        let i: Number = number!(token);

        let token = next()?;
        expect(&token, RIGHT_PAREN, ErrorType::UnexpectedToken)?;

        Ok(Color(format!("{n};5;{i};")))
    }

    pub fn four_bit(n: u8) -> Color {
        Color(format!("{n};"))
    }
}

impl TryFrom<String> for Color {
    type Error = crate::error::Error;

    fn try_from(source: String) -> Result<Self, Self::Error> {
        if source.is_empty() {
            return Ok(Color(source));
        }
        let mut scanner = Scanner::new(source);
        let mut tokens: VecDeque<_> = scanner.scan_tokens().into();
        //println!("{:?}", tokens);
        //let line = tokens[0].line;
        let mut next = || {
            if tokens.is_empty() {
                return Err(Error::new(
                    ErrorType::UnexpectedEof,
                    "Unexpected end of input".to_string(),
                    0, // TODO: line
                    0, // TODO: column
                ));
            }

            Ok(tokens.pop_front().unwrap())
        };

        let token = next()?;
        let color = match token.r#type {
            token::TokenType::FG_BLACK => Color::four_bit(30),
            token::TokenType::FG_RED => Color::four_bit(31),
            token::TokenType::FG_GREEN => Color::four_bit(32),
            token::TokenType::FG_YELLOW => Color::four_bit(33),
            token::TokenType::FG_BLUE => Color::four_bit(34),
            token::TokenType::FG_MAGENTA => Color::four_bit(35),
            token::TokenType::FG_CYAN => Color::four_bit(36),
            token::TokenType::FG_WHITE => Color::four_bit(37),
            token::TokenType::FG_RGB => Color::rgb(next, 38)?,
            token::TokenType::FG_BYTE => Color::byte(next, 38)?,
            token::TokenType::FG_DEFAULT => Color::four_bit(39),

            token::TokenType::BG_BLACK => Color::four_bit(40),
            token::TokenType::BG_RED => Color::four_bit(41),
            token::TokenType::BG_GREEN => Color::four_bit(42),
            token::TokenType::BG_YELLOW => Color::four_bit(43),
            token::TokenType::BG_BLUE => Color::four_bit(44),
            token::TokenType::BG_MAGENTA => Color::four_bit(45),
            token::TokenType::BG_CYAN => Color::four_bit(46),
            token::TokenType::BG_WHITE => Color::four_bit(47),
            token::TokenType::BG_RGB => Color::rgb(next, 48)?,
            token::TokenType::BG_BYTE => Color::byte(next, 38)?,
            token::TokenType::BG_DEFAULT => Color::four_bit(49),

            _ => {
                return Err(Error::new(
                    ErrorType::InvalidColor,
                    format!("{:?} is not a valid color", token.lexeme),
                    token.line,
                    0, // TODO: column
                ));
            }
        };

        Ok(color)
    }
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        color.0
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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
