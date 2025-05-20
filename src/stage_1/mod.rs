use std::mem::take;

pub struct Stage1 {
    source: String,
    start: usize,
    current: usize,
    parts: Vec<String>,
}

impl Default for Stage1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Stage1 {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            start: 0,
            current: 0,
            parts: Vec::new(),
        }
    }

    pub fn parse(&mut self, source: String) -> String {
        self.source = source;
        self.parts = Vec::with_capacity(self.source.len() / 2);

        let mut index = 0;
        while !self.is_at_end() {
            self.start = self.current;

            if self.peek(0) == b'{' && self.peek(1) == b'{' {
                self.advance(2);
                self.add_part();
            } else if self.peek(0) == b'{' && self.peek(1) == b'}' && self.peek(2) != b'}' {
                self.advance(2);
                self.parts.push(format!("{{{index}}}"));
                index += 1;
            } else {
                loop {
                    self.advance(1);

                    if self.is_at_end() {
                        break;
                    }
                    if self.peek(0) == b'{' && self.peek(1) == b'{' {
                        break;
                    }
                    if self.peek(0) == b'{' && self.peek(1) == b'}' && self.peek(2) != b'}' {
                        break;
                    }
                }
                self.add_part();
            }
        }
        take(&mut self.parts).join("")
    }

    fn peek(&self, n: usize) -> u8 {
        if self.current + n >= self.source.len() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + n]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self, n: usize) {
        self.current += n;
    }

    fn add_part(&mut self) {
        let text = self.source[self.start..self.current].to_string();
        self.parts.push(text);
    }
}
