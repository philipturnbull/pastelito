use pastelito_data::POS;

use crate::{span::FullByteSpan, ByteSpan};

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
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub struct Word<'a> {
    str: &'a str,
    offset: usize,
    pos: Option<POS>,
}

impl<'a> Word<'a> {
    /// Create a new word with an unknown part of speech.
    pub fn new(str: &'a str, offset: usize) -> Self {
        Word {
            str,
            offset,
            pos: None,
        }
    }

    /// Create a new word with a known part of speech.
    pub fn new_with_pos(str: &'a str, offset: usize, pos: POS) -> Self {
        Word {
            str,
            offset,
            pos: Some(pos),
        }
    }

    // Get the offset of this word in the input data.
    pub fn as_offset(&self) -> usize {
        self.offset
    }

    /// Get the word as a string slice from `data`.
    ///
    /// This assumes that `span` is a valid span in `data`. If this span was not
    /// built from `span` then incorrect results will be returned or the
    /// function may panic.
    pub fn as_str(&self) -> &'a str {
        self.str
    }

    /// Get the byte span of this word in the input data.
    pub fn as_span(&self) -> ByteSpan {
        ByteSpan::new_unchecked(self.offset, self.offset + self.str.len())
    }

    /// Check if the part-of-speech tag of this word is unknown.
    pub fn is_unknown_pos(&self) -> bool {
        self.pos.is_none()
    }

    /// Get the part-of-speech tag of this word.
    pub fn pos(&self) -> Option<POS> {
        self.pos
    }

    /// Set the part-of-speech tag of this word.
    pub fn set_pos(&mut self, pos: POS) {
        self.pos = Some(pos);
    }

    /// Clear the part-of-speech tag of this word.
    pub fn clear_pos(&mut self) {
        self.pos = None;
    }

    /// Checks that two words are equal, ignoring case.
    pub fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.str.eq_ignore_ascii_case(other.str)
    }
}

impl<'a> From<FullByteSpan<'a>> for Word<'a> {
    fn from(span: FullByteSpan<'a>) -> Self {
        Word::new(span.as_str(), span.as_span().start())
    }
}

/// A block of text, containing `T` elements.
///
/// Each block has a kind and holds a reference to the original input data.
#[derive(Clone, Debug)]
pub struct Block<T> {
    kind: BlockKind,
    contents: Vec<T>,
}

impl<T> Block<T> {
    /// Create a new block from multiple elements.
    pub fn new(kind: BlockKind, contents: Vec<T>) -> Self {
        Block { kind, contents }
    }

    /// Create a new block from a single element.
    pub fn singleton(kind: BlockKind, contents: T) -> Self {
        Block::new(kind, vec![contents])
    }

    /// Get the kind of this block.
    pub fn kind(&self) -> BlockKind {
        self.kind
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

impl<T> IntoIterator for Block<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.contents.into_iter()
    }
}

impl Block<Word<'_>> {
    /// A helper function to create a `Block` from a list of words and call `cb`
    /// with the result.
    ///
    /// The words are separated by spaces and the correct byte spans are calculated.
    #[cfg(test)]
    pub(crate) fn with_testing_block(words: &[(&str, POS)], cb: impl Fn(Block<Word<'static>>)) {
        let mut data = String::new();

        let mut ranges = Vec::new();

        for (word, pos) in words {
            let start = data.len();
            data.push_str(word);
            let end = data.len();
            data.push(' ');
            ranges.push((start..end, *pos));
        }

        // FIXME: We have to leak the data here to make the lifetimes work. This
        // function is only available during testing, so this isn't an issue.
        let data = Box::leak(data.into_boxed_str());

        let mut words = Vec::new();
        for (range, pos) in ranges {
            let word = &data[range.clone()];
            words.push(Word::new_with_pos(word, range.start, pos));
        }

        let block = Block::new(BlockKind::Paragraph, words);
        cb(block);
    }
}
