use std::fmt::Display;

use crate::{
    error::ErrorKind,
    get_num,
    num::str_to_u32,
    scanner::{span::Span, token::TokenKind, Scanner},
    Error,
};

use super::expect;

#[derive(PartialEq, Debug, Clone)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl TryFrom<(&str, Span)> for Rgb {
    type Error = Error;

    fn try_from(value: (&str, Span)) -> Result<Self, Self::Error> {
        let mut scanner = Scanner::new(value.0);
        scanner.text_mode = false;
        scanner.parse_colors = true;
        scanner.span = value.1;

        let token = scanner.scan_token()?;
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;

        match token.kind {
            TokenKind::Number => {
                r = get_num!(str_to_u32(token.content, 10), &token) as u8;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Comma)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Number)?;
                g = get_num!(str_to_u32(token.content, 10), &token) as u8;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Comma)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Number)?;
                b = get_num!(str_to_u32(token.content, 10), &token) as u8;
            }

            TokenKind::Hex => match token.content.len() {
                4 => {
                    r = get_num!(str_to_u32(&token.content[1..2].repeat(2), 16), &token) as u8;
                    g = get_num!(str_to_u32(&token.content[2..3].repeat(2), 16), &token) as u8;
                    b = get_num!(str_to_u32(&token.content[3..4].repeat(2), 16), &token) as u8;
                }

                7 => {
                    r = get_num!(str_to_u32(&token.content[1..3], 16), &token) as u8;
                    g = get_num!(str_to_u32(&token.content[3..5], 16), &token) as u8;
                    b = get_num!(str_to_u32(&token.content[5..7], 16), &token) as u8;
                }

                _ => {}
            },

            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken(TokenKind::Number, Some(token.kind)),
                    &token,
                ))
            }
        }

        Ok(Rgb(r, g, b))
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{};{};{}", self.0, self.1, self.2))
    }
}

#[test]
fn test_rgb_from_str() {
    let rgb = Rgb::try_from(("0 , 50 , 10", Span::default()));
    assert!(rgb.is_ok());
    assert_eq!(rgb.unwrap(), Rgb(0, 50, 10));
}

#[test]
fn test_rgb_from_str_hex() {
    let rgb = Rgb::try_from(("#0fffff", Span::default()));
    assert!(rgb.is_ok());
    assert_eq!(rgb.unwrap(), Rgb(15, 255, 255));

    let rgb = Rgb::try_from(("#fff", Span::default()));
    assert!(rgb.is_ok());
    assert_eq!(rgb.unwrap(), Rgb(255, 255, 255));
}
