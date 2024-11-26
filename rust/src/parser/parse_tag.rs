use std::io::Write;

use crate::{assign_attrib, compiler::ErrorKind, own, scanner::token::TokenKind, Error};

use super::{
    tag::{Tag, TagKind, TagType},
    Parser,
};

impl<B: AsRef<[u8]>, W: Write> Parser<B, W> {
    pub(super) fn parse_tag(&mut self) -> Result<Tag, Error> {
        let token = self.scanner.scan_token()?;
        let r#type = if Self::expect_token(&token, TokenKind::OpenTag, ErrorKind::UnexpectedToken)
            .is_ok()
        {
            TagType::Open
        } else if Self::expect_token(
            &token,
            TokenKind::OpenTagAndSlash,
            ErrorKind::UnexpectedToken,
        )
        .is_ok()
        {
            TagType::Closed
        } else if Self::expect_token(&token, TokenKind::Text, ErrorKind::UnexpectedToken).is_ok() {
            let mut tag = Tag::new(TagKind::Text, TagType::OpenAndClose);
            tag.text = Some(own!(token.content));
            return Ok(tag);
        } else {
            return Err(Error::new(ErrorKind::UnexpectedToken, token.clone()));
        };

        let token = self.scanner.scan_token()?;
        Self::expect_token(&token, TokenKind::Identifier, ErrorKind::UnexpectedToken)?;

        let kind = match token.content {
            "a" => TagKind::A,
            "b" => TagKind::B,
            "br" => TagKind::Br,
            "c" => TagKind::C,
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

        let mut token = self.scanner.scan_token()?;
        while token.kind == TokenKind::Identifier {
            match tag.kind {
                _ => match token.content {
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
                        "black" | "blue" | "byte" | "cyan" | "green" | "magenta" | "red"
                        | "rgb" | "white" | "yellow" => {
                            tag.color = Some((own!(token.content), None));
                            token = self.scanner.scan_token()?;
                            if token.kind == TokenKind::Equal {
                                token = self.scanner.scan_token()?;
                                Self::expect_token(
                                    &token,
                                    TokenKind::String,
                                    ErrorKind::UnexpectedToken,
                                )?;
                                let end = token.content.len() - 1;
                                tag.color.as_mut().unwrap().1 = Some(own!(token.content[1..end]));
                                token = self.scanner.scan_token()?;
                            }
                        }

                        _ => {
                            token = self.scanner.scan_token()?;
                            if token.kind == TokenKind::Equal {
                                token = self.scanner.scan_token()?;
                                Self::expect_token(
                                    &token,
                                    TokenKind::String,
                                    ErrorKind::UnexpectedToken,
                                )?;
                                token = self.scanner.scan_token()?;
                            }
                        }
                    },
                },
            }
        }

        if Self::expect_token(&token, TokenKind::CloseTag, ErrorKind::UnexpectedToken).is_ok() {
        } else if Self::expect_token(
            &token,
            TokenKind::SlashAndCloseTag,
            ErrorKind::UnexpectedToken,
        )
        .is_ok()
        {
            tag.r#type = TagType::OpenAndClose;
        } else {
            return Err(Error::new(ErrorKind::UnexpectedToken, token.clone()));
        };

        Ok(tag)
    }
}
