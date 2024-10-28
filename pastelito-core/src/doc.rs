use tracing::debug_span;

use crate::{block::Block, Tagger, Word};

/// A parser that converts a string into a sequence of blocks.
pub trait Parser {
    /// Parse the given data into a list of tokenized blocks.
    fn parse<'input>(&self, input: &'input str) -> Vec<Block<Word<'input>>>;
}

/// A document, containing a sequence of blocks.
#[derive(Clone, Debug)]
pub struct Document<'input> {
    input: &'input str,
    blocks: Vec<Block<Word<'input>>>,
}

impl<'input> Document<'input> {
    /// Create a new document by parsing the input data with the given parser.
    pub fn new(parser: &impl Parser, input: &'input str) -> Self {
        let parse_span = debug_span!("parse");
        let mut blocks = parse_span.in_scope(|| parser.parse(input));

        let tagger = Tagger::default();
        let tag_span = debug_span!("tag");
        tag_span.in_scope(|| {
            for block in &mut blocks {
                tagger.tag(block);
            }
        });

        Document { input, blocks }
    }

    /// Get the input data that this document was created from.
    pub fn input(&self) -> &'input str {
        self.input
    }

    /// Get an iterator over the blocks in this document.
    pub fn iter(&self) -> impl Iterator<Item = &Block<Word<'input>>> {
        self.blocks.iter()
    }

    /// Get a mutable iterator over the blocks in this document.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Block<Word<'input>>> {
        self.blocks.iter_mut()
    }
}

impl<'input> IntoIterator for Document<'input> {
    type Item = Block<Word<'input>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.into_iter()
    }
}

impl<'input> IntoIterator for &'input Document<'_> {
    type Item = &'input Block<Word<'input>>;
    type IntoIter = std::slice::Iter<'input, Block<Word<'input>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.iter()
    }
}

impl<'input> IntoIterator for &'input mut Document<'input> {
    type Item = &'input mut Block<Word<'input>>;
    type IntoIter = std::slice::IterMut<'input, Block<Word<'input>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.iter_mut()
    }
}
