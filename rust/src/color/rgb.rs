use crate::{
    error::{ErrorKind, FromError},
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
                r = Error::convert(token.content.parse::<f64>(), token.start_pos, token.end_pos)?
                    .round() as u8;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Comma)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Number)?;
                g = Error::convert(token.content.parse::<f64>(), token.start_pos, token.end_pos)?
                    .round() as u8;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Comma)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Number)?;
                b = Error::convert(token.content.parse::<f64>(), token.start_pos, token.end_pos)?
                    .round() as u8;
            }

            TokenKind::Hex => match token.content.len() {
                4 => {
                    r = Error::convert(
                        u8::from_str_radix(&token.content[1..2].repeat(2), 16),
                        token.start_pos.clone(),
                        token.end_pos.clone(),
                    )?;
                    g = Error::convert(
                        u8::from_str_radix(&token.content[2..3].repeat(2), 16),
                        token.start_pos.clone(),
                        token.end_pos.clone(),
                    )?;
                    b = Error::convert(
                        u8::from_str_radix(&token.content[3..4].repeat(2), 16),
                        token.start_pos,
                        token.end_pos,
                    )?;
                }

                7 => {
                    r = Error::convert(
                        u8::from_str_radix(&token.content[1..3], 16),
                        token.start_pos.clone(),
                        token.end_pos.clone(),
                    )?;
                    g = Error::convert(
                        u8::from_str_radix(&token.content[3..5], 16),
                        token.start_pos.clone(),
                        token.end_pos.clone(),
                    )?;
                    b = Error::convert(
                        u8::from_str_radix(&token.content[5..7], 16),
                        token.start_pos,
                        token.end_pos,
                    )?;
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
