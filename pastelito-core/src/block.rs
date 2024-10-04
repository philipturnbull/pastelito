use pastelito_data::POS;

use crate::span::{ByteSpan, FullByteSpan};

/// The "kind" of a block. This allows rules to change their behavior based on
/// the kind of block.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockKind {
    /// A regular text paragraph
    Paragraph,
    /// A heading or title
    Heading,
}

/// A word in a block of text.
#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    /// The byte span for this word.
    pub span: ByteSpan,
    /// The part of speech for this word.
    pub pos: Option<POS>,
}

impl Word {
    /// Create a new word with an unknown part of speech.
    pub fn new(span: ByteSpan) -> Self {
        Word { span, pos: None }
    }

    /// Create a new word with a known part of speech.
    pub fn new_with_pos(span: ByteSpan, pos: POS) -> Self {
        Word {
            span,
            pos: Some(pos),
        }
    }

    /// Get the word as a string slice from `data`.
    ///
    /// This assumes that `span` is a valid span in `data`. If this span was not
    /// built from `span` then incorrect results will be returned or the
    /// function may panic.
    fn as_str<'a>(&self, data: &'a str) -> &'a str {
        self.span.as_str(data)
    }
}

impl From<ByteSpan> for Word {
    fn from(span: ByteSpan) -> Self {
        Word::new(span)
    }
}

impl<'a> From<FullByteSpan<'a>> for Word {
    fn from(span: FullByteSpan<'a>) -> Self {
        Word::new(span.as_span())
    }
}

/// A block of text, containing `T` elements.
///
/// Each block has a kind and holds a reference to the original input data.
#[derive(Clone, Debug)]
pub struct Block<'a, T> {
    kind: BlockKind,
    data: &'a str,
    contents: Vec<T>,
}

impl<'a, T> Block<'a, T> {
    /// Create a new block from multiple elements.
    pub fn new(kind: BlockKind, string: &'a str, contents: Vec<T>) -> Self {
        Block {
            kind,
            data: string,
            contents,
        }
    }

    /// Create a new block from a single element.
    pub fn singleton(kind: BlockKind, data: &'a str, contents: T) -> Self {
        Block::new(kind, data, vec![contents])
    }

    /// Get the kind of this block.
    pub fn kind(&self) -> BlockKind {
        self.kind
    }

    /// Get the underlying data for this block.
    pub(crate) fn data(&self) -> &'a str {
        self.data
    }

    /// Get the contents of this block as a slice.
    pub fn as_slice(&self) -> &[T] {
        &self.contents
    }

    /// Get the contents of this block as an iterator.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.contents.iter()
    }

    /// Get the contents of this block as a mutable iterator.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.contents.iter_mut()
    }
}

impl<'a> Block<'a, Word> {
    pub fn iter_with_str(&self) -> impl Iterator<Item = (&Word, &'a str)> {
        self.contents
            .iter()
            .map(move |word| (word, word.as_str(self.data)))
    }

    pub fn iter_mut_with_str(&mut self) -> impl Iterator<Item = (&mut Word, &'a str)> {
        self.contents.iter_mut().map(|word| {
            let str = word.as_str(self.data);
            (word, str)
        })
    }
}

impl<T> IntoIterator for Block<'_, T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

impl Block<'_, Word> {
    /// A helper function to create a `Block` from a list of words and call `cb`
    /// with the result.
    ///
    /// The words are separated by spaces and the correct byte spans are calculated.
    #[cfg(test)]
    pub(crate) fn with_testing_block(words: &[(&str, POS)], cb: impl FnOnce(Block<'_, Word>)) {
        let mut contents = Vec::new();
        let mut data = String::new();

        for (word, pos) in words {
            let start = data.len();
            data.push_str(word);
            let end = data.len();
            data.push(' ');
            contents.push(Word::new_with_pos(
                ByteSpan::new_unchecked(start, end),
                *pos,
            ));
        }

        let block = Block::new(BlockKind::Paragraph, &data, contents);
        cb(block);
    }
}
