use state::State;
pub use tag::{Tag, TagKind, TagType};
use write_attribs::Attrib;

use crate::color::ansi_style::AnsiStyle;
use crate::color::channel::Channel;
use crate::color::rgb::Rgb;
use crate::color::Color;
use crate::compiler::ErrorKind;
use crate::scanner::token::{Token, TokenKind};
use crate::scanner::Scanner;
use crate::value::{B, I, S, U};
use crate::{own, Error};
use std::collections::HashMap;
use std::io::Write;

mod macros;
mod parse_tag;
mod state;
mod tag;
mod write_attribs;

macro_rules! ok {
    ( $x:expr ) => {{
        if $x.is_some() {
            $x.unwrap()
        } else {
            Default::default()
        }
    }};
}

pub struct Parser<B: AsRef<[u8]>, W: Write> {
    pub(super) scanner: Scanner<B>,
    pub(crate) out: W,
    pub(crate) bindings: Option<HashMap<String, Tag>>,
    state: State,
    trim_start: bool,
    first_text: bool,
}

impl<B: AsRef<[u8]>, W: Write> Parser<B, W> {
    pub fn new(source: B, out: W, bindings: Option<HashMap<String, Tag>>) -> Parser<B, W> {
        Parser {
            scanner: Scanner::new(source),
            out,
            bindings,
            state: State::new(),
            trim_start: false,
            first_text: true,
        }
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        let tag = self.parse_tag()?;
        Self::expect_tag(&tag, TagKind::Ziyy, ErrorKind::ExpectedZiyyTag)?;
        //let _ = write!(self.out, "\x1b[m");

        loop {
            let mut tag = self.parse_tag()?;
            match tag.r#type {
                TagType::Open => match tag.kind {
                    TagKind::A => {
                        let tag2 = self.parse_tag()?;
                        Self::expect_tag(&tag2, TagKind::Text, ErrorKind::UnexpectedTag)?;

                        let _ = write!(
                            self.out,
                            "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
                            ok!(ok!(tag.href.clone())),
                            ok!(tag2.text)
                        );

                        let tag2 = self.parse_tag()?;
                        Self::expect_tag(&tag2, TagKind::A, ErrorKind::UnexpectedTag)?;
                    }

                    TagKind::Any(ref s) => {
                        let val = self.bindings.as_ref().unwrap().get(s);
                        let mut ansi = AnsiStyle::new();
                        if let Some(btag) = val {
                            inherit(&btag, &mut tag);
                            self.write_attribs(
                                &mut tag,
                                &mut ansi,
                                &[
                                    Attrib::B,
                                    Attrib::C,
                                    Attrib::I,
                                    Attrib::S,
                                    Attrib::U,
                                    Attrib::X,
                                ],
                            )?;
                        }

                        self.write_and_save(tag.kind, ansi);
                    }

                    TagKind::B => {
                        let mut ansi = AnsiStyle::new();
                        ansi.add_style(B);

                        self.write_attribs(
                            &mut tag,
                            &mut ansi,
                            &[
                                Attrib::Val,
                                Attrib::C,
                                Attrib::I,
                                Attrib::S,
                                Attrib::U,
                                Attrib::X,
                            ],
                        )?;

                        self.write_and_save(tag.kind, ansi);
                    }

                    TagKind::Br => {
                        self.write_nl(tag)?;
                    }

                    TagKind::C => {
                        let mut ansi = AnsiStyle::new();
                        if let Some(ref color_info) = tag.color {
                            let color = if color_info.0 != "rgb" {
                                let rgb = Rgb::try_from(color_info.0.as_str())?;
                                Color::from((rgb, Channel::Foreground))
                            } else {
                                if color_info.1 == Some(own!("light")) {
                                    ansi.add_style(B);
                                }
                                Color::try_from((color_info.0.as_str(), Channel::Foreground))?
                            };
                            let buf: Vec<u8> = color.into();
                            ansi.add_style(buf);
                        }

                        self.write_attribs(
                            &mut tag,
                            &mut ansi,
                            &[Attrib::B, Attrib::I, Attrib::S, Attrib::U, Attrib::X],
                        )?;

                        self.write_and_save(tag.kind, ansi);
                    }

                    TagKind::I => {
                        let mut ansi = AnsiStyle::new();
                        ansi.add_style(I);

                        self.write_attribs(
                            &mut tag,
                            &mut ansi,
                            &[
                                Attrib::Val,
                                Attrib::B,
                                Attrib::C,
                                Attrib::S,
                                Attrib::U,
                                Attrib::X,
                            ],
                        )?;

                        self.write_and_save(tag.kind, ansi);
                    }

                    TagKind::Let => {}

                    TagKind::P => {
                        let _ = write!(self.out, "\n");
                        self.trim_start = true;
                        let mut ansi = AnsiStyle::new();

                        self.write_attribs(
                            &mut tag,
                            &mut ansi,
                            &[
                                Attrib::Val,
                                Attrib::Tab,
                                Attrib::B,
                                Attrib::C,
                                Attrib::I,
                                Attrib::S,
                                Attrib::U,
                                Attrib::X,
                            ],
                        )?;

                        self.write_and_save(tag.kind, ansi);
                    }

                    TagKind::S => {
                        let mut ansi = AnsiStyle::new();
                        ansi.add_style(S);

                        self.write_attribs(
                            &mut tag,
                            &mut ansi,
                            &[
                                Attrib::Val,
                                Attrib::B,
                                Attrib::C,
                                Attrib::I,
                                Attrib::U,
                                Attrib::X,
                            ],
                        )?;

                        self.write_and_save(tag.kind, ansi);
                    }

                    TagKind::Text => {}

                    TagKind::U => {
                        let mut ansi = AnsiStyle::new();
                        ansi.add_style(U);

                        self.write_attribs(
                            &mut tag,
                            &mut ansi,
                            &[
                                Attrib::Val,
                                Attrib::B,
                                Attrib::C,
                                Attrib::I,
                                Attrib::S,
                                Attrib::X,
                            ],
                        )?;

                        self.write_and_save(tag.kind, ansi);
                    }

                    TagKind::X => {
                        let mut ansi = AnsiStyle::new();
                        if let Some(ref color_info) = tag.color {
                            let color = if color_info.0 != "rgb" {
                                let rgb = Rgb::try_from(color_info.0.as_str())?;
                                Color::from((rgb, Channel::Background))
                            } else {
                                /* if color_info.1 == Some(own!("light")) {
                                    ansi.add_style(B);
                                } */
                                Color::try_from((color_info.0.as_str(), Channel::Background))?
                            };
                            let buf: Vec<u8> = color.into();
                            ansi.add_style(buf);
                        }

                        self.write_attribs(
                            &mut tag,
                            &mut ansi,
                            &[Attrib::C, Attrib::I, Attrib::S, Attrib::U],
                        )?;

                        self.write_and_save(tag.kind, ansi);
                    }

                    TagKind::Ziyy => todo!(),
                },

                TagType::Closed => match tag.kind {
                    TagKind::Text => {}
                    TagKind::Ziyy => {
                        return Ok(());
                    }
                    _ => {
                        let ctag = self.state.current_tag();
                        if ctag.is_none() {}
                        Self::expect_tag(
                            &tag,
                            ctag.clone().unwrap().clone(),
                            ErrorKind::WrongClosingTag,
                        )?;
                        self.state.pop();

                        if let Some(style) = self.state.current_save() {
                            let _ = write!(self.out, "{style}");
                        }
                    }
                },

                TagType::OpenAndClose => match tag.kind {
                    TagKind::Br => {
                        if let Some(n) = tag.n {
                            if let Some(val) = n {
                                let n: usize = val.parse()?;
                                let _ = write!(self.out, "{}", "\n".repeat(n));
                            }
                        } else {
                            let _ = write!(self.out, "\n");
                        }
                    }

                    TagKind::Let => {
                        if self.bindings.is_none() {
                            self.bindings = Some(HashMap::with_capacity(1024));
                        }

                        if let Some(ref val) = tag.name {
                            if let Some(name) = val {
                                self.bindings.as_mut().unwrap().insert(name.clone(), tag);
                            }
                        }
                    }

                    TagKind::Text => {
                        let text = ok!(tag.text);
                        if self.trim_start || self.first_text {
                            let _ = write!(self.out, "{}", dedupe_whitespace(text.trim_start()));
                            self.trim_start = false;
                            self.first_text = false;
                        } else {
                            let _ = write!(self.out, "{}", dedupe_whitespace(&text));
                        }
                    }

                    /* TODO: return error for <ziyy /> tag */
                    TagKind::Ziyy => {}
                    _ => {}
                },
            }
        }
    }

    fn expect_token(token: &Token, tt: TokenKind, err: ErrorKind) -> Result<(), Error> {
        if token.kind != tt {
            return Err(Error::new(err, token.clone()));
        }
        Ok(())
    }

    fn expect_tag(tag: &Tag, to_be: TagKind, err: ErrorKind) -> Result<(), Error> {
        if tag.kind != to_be {
            return Err(Error::from(err));
        }
        Ok(())
    }

    fn write_and_save(&mut self, tag: TagKind, style: AnsiStyle) {
        let _ = write!(self.out, "{}", &style);
        self.state.push(tag, style);
    }
}

fn dedupe_whitespace(text: &str) -> String {
    let iter = text.split(|c: char| c.is_whitespace());
    let mut check_duplicate = false;
    let mut collect = vec![];
    for word in iter {
        if check_duplicate {
            if word.is_empty() {
                continue;
            } else {
                check_duplicate = false;
            }
        }

        if word.is_empty() {
            check_duplicate = true;
        }
        collect.push(word);
    }

    if collect.len() == 1 && collect[0].is_empty() && !text.is_empty() {
        String::from(" ")
    } else {
        collect.join(" ")
    }
}

fn inherit(src: &Tag, dst: &mut Tag) {
    if src.b.is_some() && dst.b.is_none() {
        dst.b = Some(None)
    }

    if let Some(ref val) = src.c {
        if dst.c.is_some() {
        } else if let Some(c) = val {
            if !dst.c.as_ref().is_some_and(|x| x.is_some()) {
                dst.c = Some(Some(c.clone()))
            }
        }
    }

    if src.i.is_some() && dst.i.is_none() {
        dst.i = Some(None)
    }

    if src.n.is_some() && dst.n.is_none() {
        dst.n = Some(None)
    }

    if src.s.is_some() && dst.s.is_none() {
        dst.s = Some(None)
    }

    if src.u.is_some() && dst.u.is_none() {
        dst.u = Some(None)
    }

    if let Some(ref val) = src.x {
        if dst.x.is_some() {
        } else if let Some(c) = val {
            if !dst.x.as_ref().is_some_and(|x| x.is_some()) {
                dst.x = Some(Some(c.clone()))
            }
        }
    }

    if let Some(ref tab) = src.tab {
        if dst.tab.is_some() {
        } else if let Some(val) = tab {
            if !dst.tab.as_ref().is_some_and(|x| x.is_some()) {
                dst.tab = Some(Some(val.clone()))
            }
        } else {
            dst.tab = Some(None)
        }
    }
}

#[test]
fn test_parse_tag() {
    let mut parser = Parser::new("<ziyy>", vec![], None);
    let tag = parser.parse_tag();
    assert_eq!(tag.unwrap(), Tag::new(TagKind::Ziyy, TagType::Open));

    parser.scanner.set_source("</ziyy>");
    let tag = parser.parse_tag();
    assert!(tag.is_ok());
    assert_eq!(tag.unwrap(), Tag::new(TagKind::Ziyy, TagType::Closed));

    parser.scanner.set_source("<ziyy />");
    let tag = parser.parse_tag();
    assert!(tag.is_ok());
    assert_eq!(tag.unwrap(), Tag::new(TagKind::Ziyy, TagType::OpenAndClose));

    parser.scanner.set_source("<c red='dark' b>");
    let tag = parser.parse_tag();
    let mut other_tag = Tag::new(TagKind::C, TagType::Open);
    other_tag.color = Some((own!("red"), Some(own!("dark"))));
    other_tag.b = Some(None);
    assert_eq!(tag.unwrap(), other_tag);

    parser.scanner.set_source("<b i/>");
    let tag = parser.parse_tag();
    let mut other_tag = Tag::new(TagKind::B, TagType::OpenAndClose);
    other_tag.i = Some(None);
    assert_eq!(tag.unwrap(), other_tag);

    parser.scanner.set_source("yes");
    let tag = parser.parse_tag();
    let mut other_tag = Tag::new(TagKind::Text, TagType::OpenAndClose);
    other_tag.text = Some(own!("yes"));
    assert_eq!(tag.unwrap(), other_tag);

    parser.scanner.set_source("<cyan b></cyan>");
    let open_tag = parser.parse_tag().unwrap();
    let close_tag = parser.parse_tag().unwrap();
    //other_tag.text = Some(own!("yes"));
    assert_eq!(open_tag.kind, close_tag.kind);

    parser
        .scanner
        .set_source("<let cyan=\"rgb(0,150,150)\" c />");
    let tag = parser.parse_tag();
    let mut other_tag = Tag::new(TagKind::Let, TagType::OpenAndClose);
    other_tag.text = Some(own!("yes"));
    assert_eq!(tag.unwrap(), other_tag);
}
