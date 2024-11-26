use std::io::Write;

use crate::{
    color::{ansi_style::AnsiStyle, channel::Channel, Color},
    value::{B, I, S, U},
};

use super::{inherit, tag::Tag, Parser};

impl<B: AsRef<[u8]>, W: Write> Parser<B, W> {
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
                    if let Some(ref val) = tag.c {
                        if let Some(c) = val {
                            let buf: Vec<u8> =
                                Color::try_from((c.as_str(), Channel::Foreground))?.into();
                            ansi.add_style(buf);
                        }
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
                    if let Some(ref val) = tag.x {
                        if let Some(x) = val {
                            let buf: Vec<u8> =
                                Color::try_from((x.as_str(), Channel::Background))?.into();
                            ansi.add_style(buf);
                        }
                    }
                }

                Attrib::Tab => {
                    if let Some(ref tab) = tag.tab {
                        if let Some(val) = tab {
                            let n: usize = val.parse()?;
                            let _ = write!(self.out, "{}", " ".repeat(n));
                        } else {
                            let _ = write!(self.out, "\t");
                        }
                    }
                }

                Attrib::Val => {
                    if let Some(ref val) = tag.val {
                        if let Some(v) = val {
                            let val = self.bindings.as_ref().unwrap().get(v);
                            if let Some(btag) = val {
                                inherit(&btag, tag);
                            }
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
                let n: usize = val.parse()?;
                let _ = write!(self.out, "{}", "\n".repeat(n));
            }
        } else {
            let _ = write!(self.out, "\n");
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
    Val,
}
