use bit_4::Bit4;
use channel::Channel;
use rgb::Rgb;

use crate::{
    error::{ErrorKind, FromError},
    scanner::{
        token::{Token, TokenKind},
        Scanner,
    },
    Error,
};

pub mod ansi_style;
pub mod bit_4;
pub mod channel;
pub mod rgb;

#[derive(PartialEq, Debug)]
pub enum ColorKind {
    Bit4(Bit4),
    Byte(u8),
    Rgb(Rgb),
}

#[derive(PartialEq, Debug)]
pub struct Color {
    kind: ColorKind,
    channel: Channel,
}

impl Color {
    pub fn new(kind: ColorKind, channel: Channel) -> Self {
        Self { kind, channel }
    }
}

impl From<Color> for Vec<u8> {
    fn from(val: Color) -> Self {
        match val.kind {
            ColorKind::Bit4(bit4) => vec![val.channel as u8 + bit4 as u8],
            ColorKind::Byte(bit256) => vec![val.channel as u8 + 8, 5, bit256],
            ColorKind::Rgb(rgb) => vec![val.channel as u8 + 8, 2, rgb.0, rgb.1, rgb.2],
        }
    }
}

impl TryFrom<(&str, Channel)> for Color {
    type Error = Error;

    fn try_from(value: (&str, Channel)) -> Result<Self, Self::Error> {
        let mut scanner = Scanner::new(value.0);
        scanner.text_mode = false;
        scanner.parse_colors = true;

        let token = scanner.scan_token()?;
        let kind = match token.kind {
            TokenKind::Black => ColorKind::Bit4(Bit4::Black),
            TokenKind::Red => ColorKind::Bit4(Bit4::Red),
            TokenKind::Green => ColorKind::Bit4(Bit4::Green),
            TokenKind::Yellow => ColorKind::Bit4(Bit4::Yellow),
            TokenKind::Blue => ColorKind::Bit4(Bit4::Blue),
            TokenKind::Magenta => ColorKind::Bit4(Bit4::Magenta),
            TokenKind::Cyan => ColorKind::Bit4(Bit4::Cyan),
            TokenKind::White => ColorKind::Bit4(Bit4::White),
            TokenKind::Byte => {
                let token = scanner.scan_token()?;
                expect(&token, TokenKind::LeftParen)?;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::Number)?;
                let i: u8 =
                    Error::convert(token.content.parse::<f64>(), token.start_pos, token.end_pos)?
                        .round() as u8;

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::RightParen)?;

                ColorKind::Byte(i)
            }
            TokenKind::Rgb => {
                let mut rgb_string = String::with_capacity(32);

                let token = scanner.scan_token()?;
                expect(&token, TokenKind::LeftParen)?;

                let mut token = scanner.scan_token()?;
                while token.kind != TokenKind::RightParen {
                    if token.kind == TokenKind::Eof {
                        return Err(Error::new(ErrorKind::UnexpectedEof, token.clone()));
                    }
                    rgb_string.push_str(token.content);

                    token = scanner.scan_token()?;
                }

                ColorKind::Rgb(Rgb::try_from(rgb_string.as_str())?)
            }
            _ => ColorKind::Bit4(Bit4::default()),
        };

        Ok(Color::new(kind, value.1))
    }
}

impl From<(Rgb, Channel)> for Color {
    fn from(value: (Rgb, Channel)) -> Self {
        Self {
            kind: ColorKind::Rgb(value.0),
            channel: value.1,
        }
    }
}

fn expect(token: &Token, tt: TokenKind) -> Result<(), Error> {
    if token.kind != tt {
        return Err(Error::new(
            ErrorKind::UnexpectedToken {
                expected: tt,
                found: token.kind,
            },
            token.clone(),
        ));
    }
    Ok(())
}

#[test]
fn test_color_from_str() {
    let color = Color::try_from(("red", Channel::Foreground));
    assert!(color.is_ok());
    assert_eq!(
        color.unwrap(),
        Color::new(ColorKind::Bit4(Bit4::Red), Channel::Foreground)
    );

    let color = Color::try_from(("rgb(0,10,100)", Channel::Background));
    assert!(color.is_ok());
    assert_eq!(
        color.unwrap(),
        Color::new(ColorKind::Rgb(Rgb(0, 10, 100)), Channel::Background)
    );

    let color = Color::try_from(("rgb(#B0B0B0)", Channel::Background));
    assert!(color.is_ok());
    assert_eq!(
        color.unwrap(),
        Color::new(ColorKind::Rgb(Rgb(176, 176, 176)), Channel::Background)
    );

    let color = Color::try_from(("byte(1)", Channel::Background));
    assert!(color.is_ok());
    assert_eq!(
        color.unwrap(),
        Color::new(ColorKind::Byte(1), Channel::Background)
    );
}
