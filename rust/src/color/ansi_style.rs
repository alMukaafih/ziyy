use std::io::Write as _;

#[cfg(test)]
use std::fmt::{Display, Write};

use super::Color;

#[derive(Clone, Debug)]
pub struct AnsiStyle {
    pub inner: Vec<u8>,
}

impl AnsiStyle {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn add_style<T: AsRef<[u8]>>(&mut self, buf: T) {
        self.inner.extend_from_slice(buf.as_ref());
    }

    pub fn push(&mut self, style: &AnsiStyle) {
        self.inner.extend_from_slice(&style.inner);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        if self.inner.len() == 0 {
            return bytes;
        }

        let _ = bytes.write("\x1b[".as_bytes());
        let len = self.inner.len();
        let mut i = 0;
        while i < len {
            let _ = bytes.write(self.inner[i].to_string().as_bytes());

            if i == len - 1 {
                break;
            }
            let _ = bytes.push(';' as u8);
            i += 1;
        }
        let _ = bytes.push('m' as u8);

        bytes
    }
}

impl From<Color> for AnsiStyle {
    fn from(value: Color) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

#[cfg(test)]
impl Display for AnsiStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.inner.len() == 0 {
            return Ok(());
        }

        f.write_str("\x1b[")?;
        let len = self.inner.len();
        let mut i = 0;
        while i < len {
            f.write_str(&self.inner[i].to_string())?;
            if i == len - 1 {
                break;
            }
            f.write_char(';')?;
            i += 1;
        }
        f.write_char('m')?;

        Ok(())
    }
}

#[test]
fn test_ansi_style() {
    let mut ansi = AnsiStyle::new();
    ansi.add_style([1]);
    assert_eq!("\x1b[1m", ansi.to_string());

    let mut ansi = AnsiStyle::new();
    ansi.add_style([38, 2, 10, 20, 30]);
    ansi.add_style([1]);
    assert_eq!("\x1b[38;2;10;20;30;1m", ansi.to_string())
}
