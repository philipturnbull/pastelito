use crate::block::{Block, Word};
use crate::span::{ByteSpan, FullByteSpan};
use smallvec::SmallVec;

/// A tokenizer that splits a block of text into words.
#[derive(Default)]
pub struct Tokenizer {}

impl Tokenizer {
    /// Tokenize the given block of `ByteSpan`s into a new block of `Word`s.
    pub fn tokenize<'a>(&self, block: Block<'a, ByteSpan>) -> Block<'a, Word> {
        let mut words = Vec::new();

        let kind = block.kind();
        let data = block.data();
        let tokens = block
            .into_iter()
            .map(|span| FullByteSpan::of_span(data, span));

        for token in tokens {
            for token in token.split_whitespace() {
                words.extend(self.split(token));
            }
        }

        Block::new(kind, data, words)
    }

    /// Split a single span into one or more words.
    fn split(&self, mut span: FullByteSpan) -> impl IntoIterator<Item = Word> {
        // Most words are short, so we use a SmallVec to avoid heap allocations.
        let mut words: SmallVec<[Word; 4]> = SmallVec::new();
        let mut suffixes: SmallVec<[Word; 4]> = SmallVec::new();

        // First, strip off any prefixes from the start of the span.
        while let Some((prefix, suffix)) = Tokenizer::has_prefix(span) {
            words.push(prefix.into());
            span = suffix;
        }

        // Next, split off any contractions from the start of the remaining span.
        while let Some((prefix, contraction, suffix)) = Tokenizer::has_contraction(span) {
            words.push(prefix.into());
            words.push(contraction.into());
            span = suffix;
        }

        // Next, split off any suffixes from the end of the remaining span.
        while let Some((prefix, suffix)) = Tokenizer::has_suffix(span) {
            suffixes.push(suffix.into());
            span = prefix;
        }

        // Finally, add the remaining span as a word if it's not empty.
        if !span.is_empty() {
            words.push(span.into());
        }

        words.into_iter().chain(suffixes.into_iter().rev())
    }

    fn has_prefix(span: FullByteSpan) -> Option<(FullByteSpan, FullByteSpan)> {
        match span.split_first() {
            Some((('$' | '(' | '"' | '\'' | '[', prefix), suffix)) => Some((prefix, suffix)),
            _ => None,
        }
    }

    fn has_contraction(span: FullByteSpan) -> Option<(FullByteSpan, FullByteSpan, FullByteSpan)> {
        let contractions = ["'ll", "'s", "'re", "'m", "n't"];
        contractions
            .iter()
            .filter_map(|contraction| span.split3(contraction))
            .next()
    }

    fn has_suffix(span: FullByteSpan) -> Option<(FullByteSpan, FullByteSpan)> {
        match span.split_last() {
            Some((
                prefix,
                (',' | ')' | '"' | ']' | '!' | ';' | '.' | '?' | ':' | '\'' | '%', suffix),
            )) => Some((prefix, suffix)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::BlockKind;

    fn eq(data: &str, expected: Vec<&str>) {
        let span = FullByteSpan::of_document(data);
        let block = Block::singleton(BlockKind::Paragraph, data, span.as_span());

        let tokenizer = Tokenizer {};
        let words = tokenizer.tokenize(block);

        let words = words
            .iter_with_str()
            .map(|(_, str)| str)
            .collect::<Vec<&str>>();
        assert_eq!(words, expected);
    }

    #[test]
    fn test_quotes() {
        let input = "'one', \"two\", 'they're', \"three\".";
        let expected = vec![
            "'", "one", "'", ",", "\"", "two", "\"", ",", "'", "they", "'re", "'", ",", "\"",
            "three", "\"", ".",
        ];
        eq(input, expected);
    }

    #[test]
    fn test_prefixes() {
        let input = "($1), [\"two\"], 'three'.";
        let expected = vec![
            "(", "$", "1", ")", ",", "[", "\"", "two", "\"", "]", ",", "'", "three", "'", ".",
        ];
        eq(input, expected);
    }

    #[test]
    fn test_contractions() {
        let input = "I'll, you're, he's, I'm, isn't.";
        let expected = vec![
            "I", "'ll", ",", "you", "'re", ",", "he", "'s", ",", "I", "'m", ",", "is", "n't", ".",
        ];
        eq(input, expected);
    }

    #[test]
    fn test_suffixes() {
        let input = "one, two! three? four!?";
        let expected = vec!["one", ",", "two", "!", "three", "?", "four", "!", "?"];
        eq(input, expected);
    }

    #[test]
    fn test_commas() {
        let input = "one, two, 3,000, and four.";
        let expected = vec!["one", ",", "two", ",", "3,000", ",", "and", "four", "."];
        eq(input, expected);
    }

    #[test]
    fn test_numbers() {
        let input = "1, -2, 3.14, -4.2, 5%";
        let expected = vec!["1", ",", "-2", ",", "3.14", ",", "-4.2", ",", "5", "%"];
        eq(input, expected);
    }
}
