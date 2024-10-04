#![feature(ascii_char)]
#![feature(ascii_char_variants)]
mod block;
pub mod doc;
mod matcher;
mod measures;
pub mod parsers;
mod perceptron;
pub mod rule;
mod rules;
mod span;
mod tagger;
mod tokenize;

pub use block::Block;
pub use block::Word;
pub use doc::Document;
pub use span::ByteSpan;
pub use tagger::Tagger;
