use crate::{
    block::{Block, BlockKind},
    doc::Parser,
    span::FullByteSpan,
    tokenize::Tokenizer,
    Word,
};

/// A parser for plaintext documents.
///
/// This parser treats two newlines ('\n\n') as a paragraph separator.
pub struct PlaintextParser {
    tokenizer: Tokenizer,
}

impl PlaintextParser {
    /// Create a new plaintext parser with the given tokenizer.
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }
}

impl Default for PlaintextParser {
    fn default() -> Self {
        Self::new(Tokenizer::default())
    }
}

impl Parser for PlaintextParser {
    fn parse<'a>(&self, data: &'a str) -> Vec<Block<Word<'a>>> {
        let mut blocks = Vec::new();
        let mut span = FullByteSpan::of_document(data);

        loop {
            match span.split3("\n\n") {
                Some((prefix, _, suffix)) => {
                    let prefix = prefix.skip_while(|c| c.is_whitespace());
                    if !prefix.is_empty() {
                        let block = Block::singleton(BlockKind::Paragraph, prefix.as_span());
                        blocks.push(self.tokenizer.tokenize(data, block));
                    }
                    span = suffix;
                }
                None => {
                    let prefix = span.skip_while(|c| c.is_whitespace());
                    if !prefix.is_empty() {
                        let block = Block::singleton(BlockKind::Paragraph, prefix.as_span());
                        blocks.push(self.tokenizer.tokenize(data, block));
                    }
                    break;
                }
            }
        }

        blocks
    }
}

#[cfg(test)]
mod tests {
    use crate::{block::BlockKind, doc::Document};

    use super::*;

    fn eq(data: &str, expected: Vec<Vec<&str>>) {
        let parser = PlaintextParser::default();
        let doc = Document::new(&parser, data);
        let blocks: Vec<_> = doc
            .into_iter()
            .map(|block| {
                (
                    block.kind(),
                    block.iter().map(|word| word.as_str()).collect::<Vec<_>>(),
                )
            })
            .collect();
        let expected = expected
            .into_iter()
            .map(|parts| (BlockKind::Paragraph, parts))
            .collect::<Vec<_>>();
        assert_eq!(blocks, expected, "data={:?}", data);
    }

    #[test]
    fn test_empty() {
        eq("", vec![]);
        eq("\n", vec![]);
        eq("\n\n", vec![]);
        eq("\n\n\n", vec![]);
    }

    #[test]
    fn test_single() {
        eq("one", vec![vec!["one"]]);
        eq("one\n", vec![vec!["one"]]);
        eq("one\n\n", vec![vec!["one"]]);
        eq("one\n\n\n", vec![vec!["one"]]);
        eq("\none", vec![vec!["one"]]);
        eq("\none\n", vec![vec!["one"]]);
        eq("\none\n\n", vec![vec!["one"]]);
        eq("\none\n\n\n", vec![vec!["one"]]);
        eq("\n\none", vec![vec!["one"]]);
        eq("\n\none\n", vec![vec!["one"]]);
        eq("\n\none\n\n", vec![vec!["one"]]);
    }

    #[test]
    fn test_multiple() {
        eq("one\n\ntwo", vec![vec!["one"], vec!["two"]]);
        eq("one\n\ntwo\n", vec![vec!["one"], vec!["two"]]);
        eq("one\n\ntwo\n\n", vec![vec!["one"], vec!["two"]]);
        eq("one\n\ntwo\n\n\n", vec![vec!["one"], vec!["two"]]);
        eq("\none\n\ntwo", vec![vec!["one"], vec!["two"]]);
        eq("\none\n\ntwo\n", vec![vec!["one"], vec!["two"]]);
        eq("\none\n\ntwo\n\n", vec![vec!["one"], vec!["two"]]);
        eq("\none\n\ntwo\n\n\n", vec![vec!["one"], vec!["two"]]);
        eq("\n\none\n\ntwo", vec![vec!["one"], vec!["two"]]);
        eq("\n\none\n\ntwo\n", vec![vec!["one"], vec!["two"]]);
        eq("\n\none\n\ntwo\n\n", vec![vec!["one"], vec!["two"]]);
        eq("\n\none\n\ntwo\n\n\n", vec![vec!["one"], vec!["two"]]);
    }
}
