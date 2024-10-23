use tracing::debug_span;

use crate::{block::Block, Tagger, Word};

/// A parser that converts a string into a sequence of blocks.
pub trait Parser {
    /// Parse the given data into a list of tokenized blocks.
    fn parse<'a>(&self, data: &'a str) -> Vec<Block<Word<'a>>>;
}

/// A document, containing a sequence of blocks.
#[derive(Clone, Debug)]
pub struct Document<'a> {
    blocks: Vec<Block<Word<'a>>>,
}

impl<'a> Document<'a> {
    /// Create a new document by parsing the input data with the given parser.
    pub fn new(parser: &impl Parser, data: &'a str) -> Self {
        let parse_span = debug_span!("parse");
        let mut blocks = parse_span.in_scope(|| parser.parse(data));

        let tagger_span = debug_span!("create POS tagger");
        let tagger = tagger_span.in_scope(Tagger::default);
        let tag_span = debug_span!("run POS tagger");
        tag_span.in_scope(|| {
            for block in &mut blocks {
                tagger.tag(block);
            }
        });

        Document { blocks }
    }

    /// Get an iterator over the blocks in this document.
    pub fn iter(&self) -> impl Iterator<Item = &Block<Word<'a>>> {
        self.blocks.iter()
    }

    /// Get a mutable iterator over the blocks in this document.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Block<Word<'a>>> {
        self.blocks.iter_mut()
    }
}

impl<'a> IntoIterator for Document<'a> {
    type Item = Block<Word<'a>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.into_iter()
    }
}

impl<'a> IntoIterator for &'a Document<'_> {
    type Item = &'a Block<Word<'a>>;
    type IntoIter = std::slice::Iter<'a, Block<Word<'a>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.iter()
    }
}

impl<'a> IntoIterator for &'a mut Document<'a> {
    type Item = &'a mut Block<Word<'a>>;
    type IntoIter = std::slice::IterMut<'a, Block<Word<'a>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.iter_mut()
    }
}
