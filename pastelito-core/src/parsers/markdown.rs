use core::panic;

use crate::{
    block::{Block, BlockKind},
    doc::Parser,
    span::{ByteSpan, FullByteSpan},
    tokenize::Tokenizer,
    Word,
};
use pulldown_cmark::{CowStr, Event, Options, Parser as CmarkParser, Tag, TagEnd};

#[derive(Debug)]
struct BlockBuilder {
    kind: BlockKind,
    spans: Vec<ByteSpan>,
}

impl BlockBuilder {
    fn new_paragraph() -> Self {
        BlockBuilder {
            kind: BlockKind::Paragraph,
            spans: Vec::new(),
        }
    }

    fn new_heading() -> Self {
        BlockBuilder {
            kind: BlockKind::Heading,
            spans: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    fn build(self, data: &str) -> Block<'_, ByteSpan> {
        Block::new(self.kind, data, self.spans)
    }
}

struct ParseState<'a, 't> {
    data: &'a str,
    blocks: Vec<Block<'a, Word>>,
    stack: Vec<BlockBuilder>,
    tokenizer: &'t Tokenizer,
}

impl<'a, 't> ParseState<'a, 't> {
    fn new(data: &'a str, tokenizer: &'t Tokenizer) -> Self {
        ParseState {
            data,
            blocks: Vec::new(),
            stack: Vec::new(),
            tokenizer,
        }
    }

    fn start_paragraph(&mut self) {
        self.stack.push(BlockBuilder::new_paragraph());
    }

    fn end_paragraph(&mut self) {
        self.pop_current_block();
    }

    fn start_heading(&mut self) {
        self.stack.push(BlockBuilder::new_heading());
    }

    fn end_heading(&mut self) {
        self.pop_current_block();
    }

    fn pop_current_block(&mut self) {
        let block = self.stack.pop().expect("stack should not be empty");
        if !block.is_empty() {
            self.blocks
                .push(self.tokenizer.tokenize(block.build(self.data)));
        }
    }

    fn push_span(&mut self, span: ByteSpan) {
        let block = self.stack.last_mut().expect("stack should not be empty");
        block.spans.push(span);
    }

    fn finish(self) -> Vec<Block<'a, Word>> {
        if self.stack.is_empty() {
            self.blocks
        } else {
            panic!("unbalanced blocks: {:?}", self.stack);
        }
    }
}

/// A parser for Markdown documents.
pub struct MarkdownParser {
    tokenizer: Tokenizer,
}

impl MarkdownParser {
    /// Create a new Markdown parser with the given tokenizer.
    pub fn new(tokenizer: Tokenizer) -> Self {
        MarkdownParser { tokenizer }
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        MarkdownParser::new(Tokenizer::default())
    }
}

impl Parser for MarkdownParser {
    fn parse<'a>(&self, data: &'a str) -> Vec<Block<'a, Word>> {
        let parser = CmarkParser::new_ext(
            data,
            Options::ENABLE_TABLES
                | Options::ENABLE_FOOTNOTES
                | Options::ENABLE_STRIKETHROUGH
                | Options::ENABLE_TASKLISTS
                | Options::ENABLE_HEADING_ATTRIBUTES
                | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
                | Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
                | Options::ENABLE_MATH
                | Options::ENABLE_GFM
                | Options::ENABLE_DEFINITION_LIST,
        );
        let mut state = ParseState::new(data, &self.tokenizer);

        // Should we ignore any text events? This is used to skip some sections
        // of the document that aren't prose. For example, code blocks.
        let mut ignore_text = false;

        for (event, range) in parser.into_offset_iter() {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Paragraph
                    | Tag::Item
                    | Tag::HtmlBlock
                    | Tag::TableCell
                    | Tag::DefinitionListTitle
                    | Tag::DefinitionListDefinition
                    | Tag::BlockQuote(_) => {
                        state.start_paragraph();
                    }
                    Tag::Heading { .. } => {
                        state.start_heading();
                    }
                    Tag::CodeBlock(_) | Tag::MetadataBlock(_) => {
                        ignore_text = true;
                    }
                    _ => {}
                },

                Event::End(tag) => match tag {
                    TagEnd::Paragraph
                    | TagEnd::Item
                    | TagEnd::HtmlBlock
                    | TagEnd::TableCell
                    | TagEnd::DefinitionListTitle
                    | TagEnd::DefinitionListDefinition
                    | TagEnd::BlockQuote(_) => {
                        state.end_paragraph();
                    }
                    TagEnd::Heading { .. } => {
                        state.end_heading();
                    }
                    TagEnd::CodeBlock | TagEnd::MetadataBlock(_) => {
                        ignore_text = false;
                    }
                    _ => {}
                },

                Event::Text(text) => {
                    if !ignore_text {
                        match text {
                            CowStr::Borrowed(_) => {
                                state.push_span(
                                    FullByteSpan::of_range(data, range).trim().as_span(),
                                );
                            }
                            CowStr::Inlined(_) => {
                                // This happens when the text contains escape
                                // sequences. Just use the escaped text.
                                state.push_span(
                                    FullByteSpan::of_range(data, range).trim().as_span(),
                                );
                            }
                            _ => panic!("text range is not borrowed"),
                        }
                    }
                }
                _ => {}
            }
        }

        state.finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::doc::Document;

    use super::*;

    fn eq(data: &str, expected: Vec<(BlockKind, Vec<&str>)>) {
        let parser = MarkdownParser::default();
        let doc = Document::new(&parser, data);

        let blocks: Vec<_> = doc
            .into_iter()
            .map(|block| {
                (
                    block.kind(),
                    block
                        .iter_with_str()
                        .map(|(_, str)| str)
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        assert_eq!(blocks, expected, "data={:?}", data);
    }

    fn p(strs: Vec<&str>) -> (BlockKind, Vec<&str>) {
        (BlockKind::Paragraph, strs.to_vec())
    }

    fn h(strs: Vec<&str>) -> (BlockKind, Vec<&str>) {
        (BlockKind::Heading, strs.to_vec())
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
        eq("one", vec![p(vec!["one"])]);
        eq("one\n", vec![p(vec!["one"])]);
        eq("one\n\n", vec![p(vec!["one"])]);
        eq("one\n\n\n", vec![p(vec!["one"])]);
        eq("\none", vec![p(vec!["one"])]);
        eq("\none\n", vec![p(vec!["one"])]);
        eq("\none\n\n", vec![p(vec!["one"])]);
        eq("\none\n\n\n", vec![p(vec!["one"])]);
        eq("\n\none", vec![p(vec!["one"])]);
        eq("\n\none\n", vec![p(vec!["one"])]);
        eq("\n\none\n\n", vec![p(vec!["one"])]);
    }

    #[test]
    fn test_multiple() {
        eq("one\n\ntwo", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("one\n\ntwo\n", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("one\n\ntwo\n\n", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("one\n\ntwo\n\n\n", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("\none\n\ntwo", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("\none\n\ntwo\n", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("\none\n\ntwo\n\n", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("\none\n\ntwo\n\n\n", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("\n\none\n\ntwo", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("\n\none\n\ntwo\n", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("\n\none\n\ntwo\n\n", vec![p(vec!["one"]), p(vec!["two"])]);
        eq("\n\none\n\ntwo\n\n\n", vec![p(vec!["one"]), p(vec!["two"])]);
    }

    #[test]
    fn test_paragraph() {
        eq("aaa\n\nbbb", vec![p(vec!["aaa"]), p(vec!["bbb"])]);
        eq(
            "aaa bbb\n\nccc",
            vec![p(vec!["aaa", "bbb"]), p(vec!["ccc"])],
        );
        eq(
            "aaa\nbbb\n\nccc",
            vec![p(vec!["aaa", "bbb"]), p(vec!["ccc"])],
        );
    }

    #[test]
    fn test_code_blocks() {
        eq(
            "aaa\n```\ncode1\n```\n\n```\ncode2\n```\nbbb",
            vec![p(vec!["aaa"]), p(vec!["bbb"])],
        );
    }

    #[test]
    fn test_formatting() {
        eq("aaa *bbb* ccc", vec![p(vec!["aaa", "bbb", "ccc"])]);
        eq("aaa _bbb_ ccc", vec![p(vec!["aaa", "bbb", "ccc"])]);
        eq(
            "aaa ~bbb ccc~ ddd",
            vec![p(vec!["aaa", "bbb", "ccc", "ddd"])],
        );
    }

    #[test]
    fn test_heading() {
        eq("# aaa", vec![h(vec!["aaa"])]);
        eq("# aaa\n## bbb", vec![h(vec!["aaa"]), h(vec!["bbb"])]);
        eq("# aaa\n\nbbb", vec![h(vec!["aaa"]), p(vec!["bbb"])]);
        eq(
            "# aaa\nbbb\n\nccc",
            vec![h(vec!["aaa"]), p(vec!["bbb"]), p(vec!["ccc"])],
        );
        eq("aaa\n# bbb", vec![p(vec!["aaa"]), h(vec!["bbb"])]);
    }

    #[test]
    fn test_list() {
        eq("* aaa\n* bbb", vec![p(vec!["aaa"]), p(vec!["bbb"])]);
        eq("1. aaa\n1. bbb", vec![p(vec!["aaa"]), p(vec!["bbb"])]);
    }

    #[test]
    fn test_html_comment() {
        eq("aaa <!-- bbb --> ccc", vec![p(vec!["aaa", "ccc"])]);
    }

    #[test]
    fn test_html() {
        eq("<div>aaa</div>", vec![]);
        eq("aaa<br>bbb", vec![p(vec!["aaa", "bbb"])]);
        eq("aaa<div>bbb</div>ccc", vec![p(vec!["aaa", "bbb", "ccc"])]);
        eq("aaa<div> bbb </div>ccc", vec![p(vec!["aaa", "bbb", "ccc"])]);
        eq("aaa<div>\n  bbb\n</div>ccc", vec![p(vec!["aaa", "bbb"])]);
        eq("aaa<div>\n  bbb\n</div>\nccc", vec![p(vec!["aaa", "bbb"])]);
        eq(
            "aaa<div>\n  bbb\n</div>\n\nccc",
            vec![p(vec!["aaa", "bbb"]), p(vec!["ccc"])],
        );
    }

    #[test]
    fn test_escape() {
        eq("\\\\", vec![p(vec!["\\"])]);
        eq("*\\\n\x0c", vec![p(vec!["*", "\\"])]);
    }

    #[test]
    fn test_regressions() {
        eq(",]", vec![p(vec![",", "]"])]);
    }

    #[test]
    fn test_tables() {
        eq(
            "| t0 | t1 |\n| :-- | --: |\n| v0 | v1 |",
            vec![p(vec!["t0"]), p(vec!["t1"]), p(vec!["v0"]), p(vec!["v1"])],
        );
    }

    #[test]
    fn test_quotes() {
        eq("> aaa", vec![p(vec!["aaa"])]);
    }

    #[test]
    fn test_footnotes() {
        eq("aaa[^1]\n\n[^1]: bbb", vec![p(vec!["aaa"]), p(vec!["bbb"])]);
    }

    #[test]
    fn test_definition_list() {
        eq(
            "aaa\n: bbb\n: ccc",
            vec![p(vec!["aaa"]), p(vec!["bbb"]), p(vec!["ccc"])],
        );
    }

    #[test]
    fn test_task_lists() {
        eq("- [x] aaa\n- [ ] bbb", vec![p(vec!["aaa"]), p(vec!["bbb"])]);
    }

    #[test]
    fn test_math() {
        eq(
            "aaa $`\n\\sum_{i=1}^{n} i\n`$ bbb",
            vec![p(vec!["aaa", "bbb"])],
        );
    }

    #[test]
    fn test_heading_attributes() {
        eq("# aaa {#id .class key=value}", vec![h(vec!["aaa"])]);
    }

    #[test]
    fn test_yaml_metadata() {
        eq("---\nkey: value\n---\n\naaa", vec![p(vec!["aaa"])]);
        eq("+++\nkey: value\n+++\n\naaa", vec![p(vec!["aaa"])]);
    }
}
