use crate::{
    error::ErrorKind,
    num::str_to_i32,
    scanner::{token::TokenKind, Scanner},
    Error,
};

use super::expect;

#[derive(PartialEq, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl TryFrom<&str> for Rgb {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut scanner = Scanner::new(value);
        scanner.text_mode = false;
        scanner.parse_colors = true;

        let token = scanner.scan_token()?;
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;

        match token.kind {
            TokenKind::Number => {
                r = str_to_i32(token.content, 10) as u8;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Comma)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Number)?;
                g = str_to_i32(token.content, 10) as u8;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Comma)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Number)?;
                b = str_to_i32(token.content, 10) as u8;
            }

            TokenKind::Hex => match token.content.len() {
                4 => {
                    r = str_to_i32(&token.content[1..2].repeat(2), 16) as u8;
                    g = str_to_i32(&token.content[2..3].repeat(2), 16) as u8;
                    b = str_to_i32(&token.content[3..4].repeat(2), 16) as u8;
                }

                7 => {
                    r = str_to_i32(&token.content[1..3], 16) as u8;
                    g = str_to_i32(&token.content[3..5], 16) as u8;
                    b = str_to_i32(&token.content[5..7], 16) as u8;
                }

                _ => {}
            },

            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken {
                        expected: TokenKind::Number,
                        found: token.kind,
                    },
                    token.clone(),
                ))
            }
        }

        Ok(Rgb(r, g, b))
    }
}

#[test]
fn test_rgb_from_str() {
    let rgb = Rgb::try_from("0 , 50 , 10");
    assert!(rgb.is_ok());
    assert_eq!(rgb.unwrap(), Rgb(0, 50, 10));
}

#[test]
fn test_rgb_from_str_hex() {
    let rgb = Rgb::try_from("#0fffff");
    assert!(rgb.is_ok());
    assert_eq!(rgb.unwrap(), Rgb(15, 255, 255));

    let rgb = Rgb::try_from("#fff");
    assert!(rgb.is_ok());
    assert_eq!(rgb.unwrap(), Rgb(255, 255, 255));
}
