use pastelito_model::Tag;
#[allow(unused_imports)]
use strum::VariantArray as _;

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
pub struct Word<'input> {
    str: &'input str,
    offset: usize,
    tag: Option<Tag>,
}

impl<'input> Word<'input> {
    /// Create a new word with an unknown part of speech.
    pub fn new(str: &'input str, offset: usize) -> Self {
        Word {
            str,
            offset,
            tag: None,
        }
    }

    /// Create a new word with a known part of speech.
    pub fn new_with_tag(str: &'input str, offset: usize, tag: Tag) -> Self {
        Word {
            str,
            offset,
            tag: Some(tag),
        }
    }

    // Get the offset of this word in the input data.
    pub fn as_offset(&self) -> usize {
        self.offset
    }

    /// Get the word as a string slice from the input data.
    ///
    /// This assumes that `span` is a part of the input data. If this span was
    /// not built from the input data then incorrect results will be returned or
    /// the function may panic.
    pub fn as_str(&self) -> &'input str {
        self.str
    }

    /// Get the byte span of this word in the input data.
    pub fn as_span(&self) -> ByteSpan {
        ByteSpan::new_unchecked(self.offset, self.offset + self.str.len())
    }

    /// Check if the part-of-speech tag of this word is unknown.
    pub fn is_unknown_tag(&self) -> bool {
        self.tag.is_none()
    }

    /// Get the part-of-speech tag of this word.
    pub fn tag(&self) -> Option<Tag> {
        self.tag
    }

    /// Set the part-of-speech tag of this word.
    pub fn set_tag(&mut self, tag: Tag) {
        self.tag = Some(tag);
    }

    /// Clear the part-of-speech tag of this word.
    pub fn clear_tag(&mut self) {
        self.tag = None;
    }
}

impl<'input> From<FullByteSpan<'input>> for Word<'input> {
    fn from(span: FullByteSpan<'input>) -> Self {
        Word::new(span.as_str(), span.as_span().start())
    }
}

#[cfg(test)]
pub(crate) static ARBITRARY_STR: &str = "foo bar baz quux";

#[cfg(test)]
impl quickcheck::Arbitrary for Word<'static> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let tag = *g.choose(Tag::VARIANTS).unwrap();

        let offset_lens = [(0, 3), (4, 3), (8, 3), (12, 4)];
        let (offset, len) = *g.choose(&offset_lens).unwrap();

        Word::new_with_tag(&ARBITRARY_STR[offset..(offset + len)], offset, tag)
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

#[cfg(test)]
pub(crate) mod test {
    use std::ops::Range;

    use pastelito_model::Tag;

    use super::{Block, BlockKind, Word};

    pub(crate) enum TestWord {
        Word(&'static str, Tag),
        Space,
        Newline,
    }

    fn join_words(words: &[TestWord]) -> (String, Vec<(Range<usize>, Tag)>) {
        let mut input = String::new();
        let mut ranges = Vec::new();

        for word in words {
            match word {
                TestWord::Newline => {
                    input.push('\n');
                }
                TestWord::Space => {
                    input.push(' ');
                }
                TestWord::Word(word, tag) => {
                    let start = input.len();
                    input.push_str(word);
                    let end = input.len();

                    ranges.push((start..end, *tag));
                }
            }
        }

        (input, ranges)
    }

    fn with_words<'input>(
        input: &'input str,
        ranges: &[(Range<usize>, Tag)],
        cb: impl Fn(Block<Word<'input>>),
    ) {
        let mut words = Vec::new();
        for (range, tag) in ranges {
            let word = &input[range.clone()];
            words.push(Word::new_with_tag(word, range.start, *tag));
        }

        let block = Block::new(BlockKind::Paragraph, words);
        cb(block);
    }

    /// A helper function to create a `Block` from a list of words and call `cb`
    /// with the result.
    ///
    /// The words are separated by spaces and the correct byte spans are calculated.
    pub(crate) fn with_testing_block(words: &[TestWord], cb: impl Fn(Block<Word>)) {
        let (input, ranges) = join_words(words);
        with_words(input.as_str(), &ranges, cb);
    }
}
