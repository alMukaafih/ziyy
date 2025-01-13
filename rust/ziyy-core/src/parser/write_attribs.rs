use std::io::Write;

use crate::{
    color::{ansi_style::AnsiStyle, channel::Channel, Color},
    error::FromError,
    value::{B, I, S, U},
    Error,
};

use super::{inherit, tag::Tag, Parser};

impl<T: AsRef<str>> Parser<T> {
    pub(super) fn write_attribs(
        &mut self,
        tag: &mut Tag,
        ansi: &mut AnsiStyle,
        attribs: &[Attrib],
    ) -> crate::Result<()> {
        for attrib in attribs {
            match attrib {
                Attrib::B => {
                    if tag.b.is_some() {
                        ansi.add_style(B);
                    }
                }

                Attrib::C => {
                    if let Some(Some(ref c)) = tag.c {
                        let buf: Vec<u8> =
                            Color::try_from((c.as_str(), Channel::Foreground))?.into();
                        ansi.add_style(buf);
                    }
                }

                Attrib::I => {
                    if tag.i.is_some() {
                        ansi.add_style(I);
                    }
                }

                Attrib::S => {
                    if tag.s.is_some() {
                        ansi.add_style(S);
                    }
                }

                Attrib::U => {
                    if tag.u.is_some() {
                        ansi.add_style(U);
                    }
                }

                Attrib::X => {
                    if let Some(Some(ref x)) = tag.x {
                        let buf: Vec<u8> =
                            Color::try_from((x.as_str(), Channel::Background))?.into();
                        ansi.add_style(buf);
                    }
                }

                Attrib::Tab => {
                    if let Some(ref tab) = tag.tab {
                        if let Some(val) = tab {
                            let n: usize =
                                Error::convert(val.parse(), tag.start.clone(), tag.end.clone())?;
                            let _ = self.buf.write(" ".repeat(n).as_bytes());
                        } else {
                            let _ = self.buf.write("\t".as_bytes());
                        }
                    }
                }

                Attrib::Src => {
                    if let Some(Some(ref v)) = tag.src {
                        let val = self.bindings.as_ref().unwrap().get(v);
                        if let Some(btag) = val {
                            inherit(btag, tag);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub(super) fn write_nl(&mut self, tag: Tag) -> crate::Result<()> {
        if let Some(ref n) = tag.n {
            if let Some(val) = n {
                let n: usize = Error::convert(val.parse(), tag.start, tag.end)?;
                let _ = self.buf.write("\n".repeat(n).as_bytes());
            }
        } else {
            let _ = self.buf.write("\n".as_bytes());
        }

        Ok(())
    }
}

pub enum Attrib {
    B,
    C,
    I,
    S,
    U,
    X,
    Tab,
    Src,
}
