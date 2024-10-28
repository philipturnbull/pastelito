use crate::block::{Block, Word};
use crate::span::{ByteSpan, FullByteSpan};
use smallvec::SmallVec;

/// A tokenizer that splits a block of text into words.
#[derive(Default)]
pub struct Tokenizer {}

impl Tokenizer {
    /// Tokenize the given block of `ByteSpan`s into a new block of `Word`s.
    pub fn tokenize<'input>(
        &self,
        input: &'input str,
        block: Block<ByteSpan>,
    ) -> Block<Word<'input>> {
        let mut words: Vec<Word<'input>> = Vec::new();

        let kind = block.kind();

        for token in block
            .into_iter()
            .map(|span| FullByteSpan::<'input>::of_span(input, span))
        {
            for token in token.split_whitespace() {
                words.extend(self.split(token));
            }
        }

        Block::new(kind, words)
    }

    /// Split a single span into one or more words.
    fn split<'input>(
        &self,
        mut span: FullByteSpan<'input>,
    ) -> impl IntoIterator<Item = Word<'input>> {
        // Most words are short, so we use a SmallVec to avoid heap allocations.
        let mut words: SmallVec<[Word<'input>; 4]> = SmallVec::new();
        let mut suffixes: SmallVec<[Word<'input>; 4]> = SmallVec::new();

        // First, strip off any prefixes from the start of the span.
        while let Some((prefix, suffix)) = Tokenizer::has_prefix(span) {
            words.push(prefix.into());
            span = suffix;
        }

        // Next, split off any contractions from the start of the remaining span.
        while let Some((prefix, contraction, suffix)) = Tokenizer::has_contraction(span) {
            if !prefix.is_empty() {
                words.push(prefix.into());
            }
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

    fn eq(input: &str, expected: Vec<&str>) {
        let span = FullByteSpan::of_document(input);
        let block = Block::singleton(BlockKind::Paragraph, span.as_span());

        let tokenizer = Tokenizer {};
        let words = tokenizer.tokenize(input, block);

        let words = words
            .iter()
            .map(|word| word.as_str())
            .collect::<Vec<&str>>();
        assert_eq!(words, expected);
    }

    #[test]
    fn test_just_contraction() {
        let input = "n't";
        let expected = vec!["n't"];
        eq(input, expected);
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
