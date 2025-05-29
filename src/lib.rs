#![allow(clippy::pedantic)]
pub mod error;
#[macro_use]
pub mod scanner;
mod common;
pub mod indexer;
pub mod parser;
pub mod resolver;
pub mod splitter;
