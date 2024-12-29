use std::io::Write;

use crate::color::rgb::Rgb;
use crate::color::{ansi_style::AnsiStyle, channel::Channel, Color};
use crate::error::ErrorKind;
use crate::value::{B, I, S, U};
use crate::{own, Error};

use super::{inherit, write_attribs::Attrib, Parser, Tag, TagKind};

impl<T: AsRef<[u8]>> Parser<T> {
    pub(crate) fn parse_open_tag(&mut self, mut tag: Tag) -> Result<(), Error> {
        match tag.kind {
            TagKind::A => {
                let tag2 = self.parse_tag()?;
                Self::expect_tag(&tag2, TagKind::Text, ErrorKind::UnexpectedTag)?;

                let _ = self.buf.write("\x1b]8;;".as_bytes());
                let _ = self.buf.write(
                    tag.href
                        .as_ref()
                        .unwrap_or(&Some(String::new()))
                        .as_ref()
                        .unwrap()
                        .as_bytes(),
                );
                let _ = self.buf.write("\x1b\\".as_bytes());
                let _ = self.buf.write(tag2.text.unwrap_or_default().as_bytes());
                let _ = self.buf.write("\x1b]8;;\x1b\\".as_bytes());

                let tag2 = self.parse_tag()?;
                Self::expect_tag(&tag2, TagKind::A, ErrorKind::UnexpectedTag)?;
            }

            TagKind::Any(ref s) => {
                let val = self.bindings.as_ref().unwrap().get(s);
                let mut ansi = AnsiStyle::new();
                if let Some(btag) = val {
                    inherit(btag, &mut tag);
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
                    let color = if color_info.0 == "rgb" {
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

            TagKind::E => {
                if let Some(style) = self.state.previous_save() {
                    let _ = self.buf.write(&style.to_bytes());
                }

                let ansi = AnsiStyle::new();
                self.state.push(tag.kind, ansi);
            }

            TagKind::Eof => {}

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
                if self.first_print_char {
                    self.first_print_char = false;
                } else {
                    let _ = self.buf.write("\n".as_bytes());
                }

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

                loop {
                    let tag2 = self.parse_next_tag()?;
                    if tag2.kind != TagKind::WhiteSpace {
                        break;
                    }
                    self.parse_tag()?;
                }
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

            TagKind::Ziyy => {
                let mut ansi = AnsiStyle::new();

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

                self.write_and_save(tag.kind, ansi);
            }

            _ => {}
        }

        Ok(())
    }
}
