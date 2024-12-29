use crate::{assign_attrib, error::ErrorKind, own, scanner::token::TokenKind, Error};

use super::{
    expect_token,
    tag::{Tag, TagKind, TagType},
    Parser,
};

impl<T: AsRef<[u8]>> Parser<T> {
    pub(super) fn parse_tag(&mut self) -> Result<Tag, Error> {
        if let Some(tag) = self.next_tag.clone() {
            self.next_tag = None;
            return Ok(tag);
        }

        let token = self.scanner.scan_token()?;

        let r#type = match token.kind {
            TokenKind::OpenTag => TagType::Open,
            TokenKind::OpenTagAndSlash => TagType::Close,

            TokenKind::Text => {
                let mut tag = Tag::new(TagKind::Text, TagType::OpenAndClose);
                tag.text = Some(own!(token.content));
                return Ok(tag);
            }

            TokenKind::WhiteSpace => {
                let mut tag = Tag::new(TagKind::WhiteSpace, TagType::OpenAndClose);
                tag.text = Some(own!(token.content));
                return Ok(tag);
            }

            TokenKind::Eof => {
                let tag = Tag::new(TagKind::Eof, TagType::OpenAndClose);
                return Ok(tag);
            }

            /* TokenKind::Esc0 => {
                let oct = u8::from_str_radix(&token.content[2..].repeat(2), 16)?;
                let mut tag = Tag::new(TagKind::Text, TagType::OpenAndClose);
                tag.text = Some(char::from_digit(num, radix));
                return Ok(tag);
            } */
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken {
                        expected: TokenKind::Text,
                        found: token.kind,
                    },
                    token.clone(),
                ));
            }
        };

        let start = token.start_pos;

        let token = self.scanner.scan_token()?;
        expect_token(&token, TokenKind::Identifier)?;
        let end = token.end_pos;

        let kind = match token.content {
            "a" => TagKind::A,
            "b" => TagKind::B,
            "br" => TagKind::Br,
            "c" => TagKind::C,
            "e" => TagKind::E,
            "i" => TagKind::I,
            "let" => TagKind::Let,
            "p" => TagKind::P,
            "s" => TagKind::S,
            "u" => TagKind::U,
            "x" => TagKind::X,
            "ziyy" => TagKind::Ziyy,
            _ => TagKind::Any(String::from(token.content)),
        };

        let mut tag = Tag::new(kind, r#type);
        tag.start = start;
        tag.end = end;

        let mut token = self.scanner.scan_token()?;
        tag.end = token.end_pos.clone();
        while token.kind == TokenKind::Identifier {
            match token.content {
                "b" => assign_attrib!(tag, b, self.scanner, token),
                "c" => assign_attrib!(tag, c, self.scanner, token),
                "i" => assign_attrib!(tag, i, self.scanner, token),
                "n" => assign_attrib!(tag, n, self.scanner, token),
                "s" => assign_attrib!(tag, s, self.scanner, token),
                "u" => assign_attrib!(tag, u, self.scanner, token),
                "x" => assign_attrib!(tag, x, self.scanner, token),

                "href" => assign_attrib!(tag, href, self.scanner, token),
                "name" => assign_attrib!(tag, name, self.scanner, token),
                "tab" => assign_attrib!(tag, tab, self.scanner, token),
                "val" => assign_attrib!(tag, val, self.scanner, token),

                _ => match token.content {
                    "black" | "blue" | "byte" | "cyan" | "green" | "magenta" | "red" | "rgb"
                    | "white" | "yellow" => {
                        tag.color = Some((own!(token.content), None));

                        token = self.scanner.scan_token()?;
                        tag.end = token.end_pos.clone();
                        if token.kind == TokenKind::Equal {
                            token = self.scanner.scan_token()?;
                            tag.end = token.end_pos.clone();
                            expect_token(&token, TokenKind::String)?;

                            let end: usize = token.content.len() - 1;
                            tag.color.as_mut().unwrap().1 = Some(own!(token.content[1..end]));
                            token = self.scanner.scan_token()?;
                            tag.end = token.end_pos.clone();
                        }
                    }

                    _ => {
                        token = self.scanner.scan_token()?;
                        tag.end = token.end_pos.clone();
                        if token.kind == TokenKind::Equal {
                            token = self.scanner.scan_token()?;
                            tag.end = token.end_pos.clone();
                            expect_token(&token, TokenKind::String)?;

                            token = self.scanner.scan_token()?;
                            tag.end = token.end_pos.clone();
                        }
                    }
                },
            }
        }

        match token.kind {
            TokenKind::CloseTag => {}
            TokenKind::SlashAndCloseTag => {
                tag.r#type = TagType::OpenAndClose;
            }

            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken {
                        expected: TokenKind::CloseTag,
                        found: token.kind,
                    },
                    token.clone(),
                ));
            }
        }

        Ok(tag)
    }

    pub(super) fn parse_next_tag(&mut self) -> Result<Tag, Error> {
        let tag = self.parse_tag()?;
        self.next_tag = Some(tag);

        Ok(self.next_tag.clone().unwrap())
    }
}
