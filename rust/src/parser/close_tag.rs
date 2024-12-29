use std::io::Write;

use crate::{error::ErrorKind, Error};

use super::{Parser, Tag, TagKind};

impl<B: AsRef<[u8]>> Parser<B> {
    pub(crate) fn parse_close_tag(&mut self, tag: Tag) -> Result<(), Error> {
        match tag.kind {
            TagKind::Text => {}
            _ => {
                let ctag = self.state.current_tag();
                //if ctag.is_none() {}
                Self::expect_tag(
                    &tag,
                    ctag.unwrap().clone(),
                    ErrorKind::WrongClosingTag {
                        expected: ctag.unwrap().clone(),
                        found: tag.kind.clone(),
                    },
                )?;

                self.state.pop();

                if let Some(style) = self.state.current_save() {
                    let _ = self.buf.write(&style.to_bytes());
                }
            }
        }
        Ok(())
    }
}
