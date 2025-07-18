use crate::error::Result;
use crate::splitter::fragment::{Fragment, FragmentType};
use chunk::{Chunk, ChunkData};
pub use word_parer::WordParser;

pub mod ansi;
pub mod chunk;
pub mod color;
pub mod tag_parer;
pub mod word_parer;

#[doc(hidden)]
pub struct Parser {
    parse_placeholders: bool,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Parser {
    pub fn new(parse_placeholders: bool) -> Self {
        Self { parse_placeholders }
    }

    pub fn parse(&self, frags: Vec<Fragment>) -> Result<Vec<Chunk>> {
        let mut tag_parser = tag_parer::TagParser::new(self.parse_placeholders);
        // let word_parer = WordParser::new();
        let mut chunks = vec![];
        for frag in frags {
            let span = frag.span;
            match frag.r#type {
                FragmentType::Tag => {
                    chunks.push(Chunk {
                        data: ChunkData::Tag(tag_parser.parse(frag)?),
                        span,
                    });
                }

                FragmentType::Whitespace => {
                    // Handle whitespace fragments
                    chunks.push(Chunk {
                        data: ChunkData::WhiteSpace(frag.lexeme),
                        span,
                    });
                }

                FragmentType::Word => {
                    // Handle word fragments
                    // let chs = word_parer.parse(frag)?;
                    // chunks.extend_from_slice(&chs);
                    chunks.push(Chunk {
                        data: ChunkData::Word(frag.lexeme),
                        span,
                    });
                }
            }
        }
        Ok(chunks)
    }
}
