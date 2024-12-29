use std::{collections::HashMap, io::Write};

use crate::{error::FromError, Error};

use super::{Parser, Tag, TagKind, TagType};

impl<T: AsRef<[u8]>> Parser<T> {
    pub(crate) fn parse_open_and_close_tag(&mut self, tag: Tag) -> Result<(), Error> {
        match tag.kind {
            TagKind::Br => {
                if let Some(n) = tag.n {
                    if let Some(val) = n {
                        let n: usize = Error::convert(val.parse(), tag.start, tag.end)?;
                        let _ = self.buf.write("\n".repeat(n).as_bytes());
                    }
                } else {
                    let _ = self.buf.write("\n".as_bytes());
                }
            }

            TagKind::Let => {
                if self.bindings.is_none() {
                    self.bindings = Some(HashMap::with_capacity(1024));
                }

                if let Some(Some(ref name)) = tag.name {
                    self.bindings.as_mut().unwrap().insert(name.clone(), tag);
                }
            }

            TagKind::Text => {
                if let Some(text) = tag.text {
                    let _ = self.buf.write(text.as_bytes());
                    if self.first_print_char {
                        self.first_print_char = false;
                    }
                }
                self.last_written = TagKind::Text;
            }

            TagKind::WhiteSpace => {
                let tag2 = self.parse_next_tag()?;
                if self.first_print_char || tag2.kind == TagKind::P && tag2.r#type == TagType::Close
                {
                } else if tag2.kind == TagKind::Eof && tag2.r#type == TagType::OpenAndClose {
                    if tag.text.unwrap().contains('\n') {
                        let _ = self.buf.write("\n".as_bytes());
                    }
                } else if self.last_written == TagKind::WhiteSpace {
                } else {
                    let _ = self.buf.write(" ".as_bytes());
                    self.last_written = TagKind::WhiteSpace;
                }
            }

            TagKind::Ziyy => {}
            _ => {}
        }
        Ok(())
    }
}
