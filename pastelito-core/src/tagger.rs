use crate::{
    block::{Block, Word},
    perceptron::Perceptron,
};
use pastelito_model::{Model, Tag};

/// A part-of-speech tagger.
pub struct Tagger {
    model: &'static Model,
    perceptron: Perceptron,
}

impl Default for Tagger {
    fn default() -> Self {
        let model = pastelito_model::get();
        Self {
            model,
            perceptron: Perceptron::new(model),
        }
    }
}

impl Tagger {
    /// Tag the words in the given block.
    ///
    /// The words are modified in place.
    pub fn tag(&self, block: &mut Block<Word>) {
        // First, add any "static" tags based solely on the token. Some simple
        // words -- such as `on`, `whose`, `after`, etc -- always have the same
        // tag, so we can tag them immediately without looking at any
        // surrounding context. This is quick and helps the following perceptron
        // step
        self.add_static_tags(block);

        // Next, predict the tags for the remaining words using the perceptron.
        self.perceptron.predict(block);
    }

    fn add_static_tags(&self, block: &mut Block<Word>) {
        for word in block.iter_mut() {
            if let Some(tag) = self.static_tag(word.as_str()) {
                word.set_tag(tag);
            }
        }
    }

    fn static_tag(&self, token: &str) -> Option<Tag> {
        if let Some(tag) = self.model.get_static_tag(token) {
            return Some(tag);
        }

        if token.len() == 1 && matches!(token.chars().next(), Some('"' | '\'')) {
            return Some(Tag::TwoQuotes);
        }

        if token.chars().any(|c: char| c.is_numeric())
            && token.chars().all(|c| {
                c.is_numeric()
                    || c == ','
                    || c == '.'
                    || c == '-'
                    || c == '+'
                    || c == '_'
                    || c == '/'
            })
        {
            return Some(Tag::CardinalNumber);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        block::{
            test::{with_testing_block, TestWord},
            BlockKind,
        },
        span::{ByteSpan, FullByteSpan},
    };
    use serde_json::Value;
    use std::{fs::File, str::FromStr as _};

    fn eq(words: &[TestWord]) {
        with_testing_block(words, |_, block| {
            let mut unknown_block = Block::new(
                block.kind(),
                block
                    .iter()
                    .map(|word| {
                        let mut word = *word;
                        word.clear_tag();
                        word
                    })
                    .collect::<Vec<_>>(),
            );

            let tagger = Tagger::default();
            tagger.tag(&mut unknown_block);

            assert_eq!(unknown_block.as_slice(), block.as_slice());
        });
    }

    #[test]
    fn test_numbers() {
        eq(&[TestWord::Word("1", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("20", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("3.3", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("-4", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("-5.5", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("+6", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("+7.7", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("8,000", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("9_000", Tag::CardinalNumber)]);
        eq(&[TestWord::Word("10/100", Tag::CardinalNumber)]);
    }

    #[test]
    fn test_static() {
        eq(&[
            TestWord::Word("The", Tag::Determiner),
            TestWord::Space,
            TestWord::Word("cat", Tag::NounSingularOrMass),
            TestWord::Space,
            TestWord::Word("sat", Tag::VerbPastTense),
            TestWord::Space,
            TestWord::Word("on", Tag::PrepositionOrSubordinatingConjunction),
            TestWord::Space,
            TestWord::Word("the", Tag::Determiner),
            TestWord::Space,
            TestWord::Word("mat", Tag::NounSingularOrMass),
            TestWord::Word(".", Tag::EndOfSentence),
        ]);
    }
    static BLOG_POST: &str = include_str!("../benches/data/leaving-rust-gamedev.md");

    fn test_block() -> Block<Word<'static>> {
        let words: Vec<Word<'static>> = BLOG_POST
            .split_whitespace()
            .map(|s| FullByteSpan::of_span(BLOG_POST, ByteSpan::new_in_str(BLOG_POST, s)).into())
            .collect();
        Block::new(BlockKind::Paragraph, words)
    }

    #[test]
    fn string_deterministic() {
        let mut block = test_block();
        let tagger = Tagger::default();
        tagger.tag(&mut block);

        let mut block_string_2 = test_block();
        tagger.tag(&mut block_string_2);

        assert_eq!(block.as_slice().len(), block_string_2.as_slice().len());

        for (i, (word_l, word_r)) in block
            .as_slice()
            .iter()
            .zip(block_string_2.as_slice())
            .enumerate()
        {
            assert_eq!(word_l.tag(), word_r.tag(), "i: {}", i);
        }
    }

    #[test]
    fn feature_deterministic() {
        let mut block_l = test_block();
        let tagger = Tagger::default();
        tagger.tag(&mut block_l);

        let mut block_r = test_block();
        tagger.tag(&mut block_r);

        assert_eq!(block_l.as_slice().len(), block_r.as_slice().len());

        for (i, (word_l, word_r)) in block_l
            .as_slice()
            .iter()
            .zip(block_r.as_slice())
            .enumerate()
        {
            assert_eq!(word_l.tag(), word_r.tag(), "i: {}", i);
        }
    }

    #[test]
    fn blog_tags() {
        let mut block = test_block();
        let tagger = Tagger::default();
        tagger.tag(&mut block);

        let actual_tags: Vec<Tag> = block
            .as_slice()
            .iter()
            .map(|word| word.tag().unwrap())
            .collect::<Vec<_>>();

        // A simple VCR-like test
        let expected_tags_filename = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/leaving-rust-gamedev-tags.json"
        );

        let expected_tags = std::fs::read_to_string(expected_tags_filename);
        let expected_tags: Option<Vec<Tag>> = match expected_tags {
            Ok(s) => match serde_json::from_str(&s) {
                // If the file exists on disk, read the expected tags
                Ok(Value::Array(v)) => Some(
                    v.into_iter()
                        .map(|item| Tag::from_str(item.as_str().unwrap()).unwrap())
                        .collect(),
                ),
                _ => panic!("Invalid JSON format"),
            },
            Err(_) => None,
        };

        let expected_tags: Vec<Tag> = match expected_tags {
            Some(expected_tags) => expected_tags,
            None => {
                // If the file is missing, write the actual tags
                let expected_tags_file = File::create(expected_tags_filename).unwrap();
                serde_json::to_writer(
                    expected_tags_file,
                    &actual_tags
                        .iter()
                        .map(Into::<&'static str>::into)
                        .collect::<Vec<&'static str>>(),
                )
                .unwrap();
                actual_tags.clone()
            }
        };

        assert_eq!(actual_tags.len(), expected_tags.len());
        for (i, (a, e)) in actual_tags.iter().zip(expected_tags.iter()).enumerate() {
            assert_eq!(a, e, "i: {}", i);
        }
    }
}
