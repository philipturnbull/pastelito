use std::fmt::Debug;

use pastelito_model::Tag;
use smallvec::SmallVec;

use crate::block::{Block, Word};

/// A pattern that matches a single word.
pub trait SingleWordPattern: Send + Sync {
    /// Check if the pattern matches the given word.
    fn matches_word(&self, word: &Word) -> bool;
}

/// A pattern that matches multiple words.
pub trait MultipleWordPattern {
    /// An estimate of the number of words that this pattern will match.
    fn size_hint(&self) -> usize;

    /// Check if the pattern matches the given words.
    ///
    /// `matched_words` is the list of words that have been matched so far. If
    /// this pattern matches, then the function should push the matched words to
    /// this list.
    fn matches<'input>(
        &self,
        matched_words: &mut Vec<Word<'input>>,
        words: &[Word<'input>],
    ) -> Option<usize>;
}

/// A pattern that matches words based on their tag.
#[derive(Copy, Clone)]
pub struct TagFn(pub fn(Tag) -> bool);

impl SingleWordPattern for TagFn {
    fn matches_word(&self, word: &Word) -> bool {
        match word.tag() {
            Some(tag) => self.0(tag),
            None => false,
        }
    }
}

/// A pattern that matches words based on their string value.
#[derive(Copy, Clone)]
pub struct StrFn(pub fn(&str) -> bool);

impl SingleWordPattern for StrFn {
    fn matches_word(&self, word: &Word) -> bool {
        self.0(word.as_str())
    }
}

/// A pattern that matches words based on their string value, ignoring case.
///
/// Strings are compared using `eq_ignore_ascii_case`.
#[derive(Clone)]
pub struct IgnoreCase {
    str: &'static str,
}

impl IgnoreCase {
    /// Create a new pattern.
    pub fn new(str: &'static str) -> Self {
        Self { str }
    }
}

impl SingleWordPattern for IgnoreCase {
    fn matches_word(&self, word: &Word) -> bool {
        word.as_str().eq_ignore_ascii_case(self.str)
    }
}

/// A pattern that matches words which end with an ASCII suffix, ignoring case.
#[derive(Clone)]
pub struct EndsWithIgnoreCase {
    suffix_reversed: SmallVec<[char; 8]>,
}

impl EndsWithIgnoreCase {
    /// Create a new pattern.
    ///
    /// This will panic if `suffix` contains any non-ASCII characters.
    pub fn new(suffix: &'static str) -> Self {
        let suffix_reversed: SmallVec<[char; 8]> = suffix.chars().rev().collect();

        if suffix_reversed.iter().any(|c| !c.is_ascii()) {
            panic!("EndsWithIgnoreCase only supports ASCII suffixes");
        }

        Self { suffix_reversed }
    }
}

impl SingleWordPattern for EndsWithIgnoreCase {
    fn matches_word(&self, word: &Word) -> bool {
        let suffix_count = self.suffix_reversed.len();

        if word.as_str().chars().count() > suffix_count {
            word.as_str()
                .chars()
                .rev()
                .take(suffix_count)
                .eq_by(self.suffix_reversed.iter().copied(), |a, b| {
                    a.eq_ignore_ascii_case(&b)
                })
        } else {
            false
        }
    }
}

/// All single word patterns are also multiple word patterns.
impl<P: SingleWordPattern> MultipleWordPattern for P {
    fn size_hint(&self) -> usize {
        1
    }

    fn matches<'input>(
        &self,
        matched_words: &mut Vec<Word<'input>>,
        words: &[Word<'input>],
    ) -> Option<usize> {
        if let Some(word) = words.first() {
            if self.matches_word(word) {
                matched_words.push(*word);
                return Some(1);
            }
        }

        None
    }
}

impl SingleWordPattern for Tag {
    fn matches_word(&self, word: &Word) -> bool {
        word.tag() == Some(*self)
    }
}

impl SingleWordPattern for &str {
    fn matches_word(&self, word: &Word) -> bool {
        word.as_str() == *self
    }
}

/// A pattern that matches a sequence of two multiple word patterns.
impl<P0: MultipleWordPattern, P1: MultipleWordPattern> MultipleWordPattern for (P0, P1) {
    fn size_hint(&self) -> usize {
        self.0.size_hint() + self.1.size_hint()
    }

    fn matches<'input>(
        &self,
        matched_words: &mut Vec<Word<'input>>,
        words: &[Word<'input>],
    ) -> Option<usize> {
        let original_len = matched_words.len();

        if let Some(next) = self.0.matches(matched_words, words) {
            let mut offset = next;
            if let Some(next) = self.1.matches(matched_words, &words[next..]) {
                offset += next;
                return Some(offset);
            }
        }

        matched_words.truncate(original_len);
        None
    }
}

/// A pattern that matches a sequence of three multiple word patterns.
impl<P0: MultipleWordPattern, P1: MultipleWordPattern, P2: MultipleWordPattern> MultipleWordPattern
    for (P0, P1, P2)
{
    fn size_hint(&self) -> usize {
        self.0.size_hint() + self.1.size_hint() + self.2.size_hint()
    }

    fn matches<'input>(
        &self,
        matched_words: &mut Vec<Word<'input>>,
        words: &[Word<'input>],
    ) -> Option<usize> {
        let original_len = matched_words.len();

        if let Some(next) = self.0.matches(matched_words, words) {
            let mut offset = next;
            if let Some(next) = self.1.matches(matched_words, &words[offset..]) {
                offset += next;
                if let Some(next) = self.2.matches(matched_words, &words[offset..]) {
                    offset += next;
                    return Some(offset);
                }
            }
        }

        matched_words.truncate(original_len);
        None
    }
}

/// A pattern that matches a sequence of four multiple word patterns.
impl<
        P0: MultipleWordPattern,
        P1: MultipleWordPattern,
        P2: MultipleWordPattern,
        P3: MultipleWordPattern,
    > MultipleWordPattern for (P0, P1, P2, P3)
{
    fn size_hint(&self) -> usize {
        self.0.size_hint() + self.1.size_hint() + self.2.size_hint() + self.3.size_hint()
    }

    fn matches<'input>(
        &self,
        matched_words: &mut Vec<Word<'input>>,
        words: &[Word<'input>],
    ) -> Option<usize> {
        let original_len = matched_words.len();

        if let Some(next) = self.0.matches(matched_words, words) {
            let mut offset = next;
            if let Some(next) = self.1.matches(matched_words, &words[offset..]) {
                offset += next;
                if let Some(next) = self.2.matches(matched_words, &words[offset..]) {
                    offset += next;
                    if let Some(next) = self.3.matches(matched_words, &words[offset..]) {
                        offset += next;
                        return Some(offset);
                    }
                }
            }
        }

        matched_words.truncate(original_len);
        None
    }
}

/// A pattern that matches any word.
#[derive(Copy, Clone, Debug)]
pub struct Any;

impl SingleWordPattern for Any {
    fn matches_word(&self, _word: &Word) -> bool {
        true
    }
}

/// A pattern that matches either of two single word patterns.
#[derive(Copy, Clone, Debug)]
pub struct OrS<L: SingleWordPattern, R: SingleWordPattern>(pub L, pub R);

impl<L: SingleWordPattern, R: SingleWordPattern> SingleWordPattern for OrS<L, R> {
    fn matches_word(&self, word: &Word) -> bool {
        self.0.matches_word(word) || self.1.matches_word(word)
    }
}

/// A pattern that matches either of two multiple word patterns.
#[derive(Copy, Clone, Debug)]
pub struct Or<L: MultipleWordPattern, R: MultipleWordPattern>(pub L, pub R);

impl<L: MultipleWordPattern, R: MultipleWordPattern> MultipleWordPattern for Or<L, R> {
    fn size_hint(&self) -> usize {
        std::cmp::max(self.0.size_hint(), self.1.size_hint())
    }
    fn matches<'input>(
        &self,
        matched_words: &mut Vec<Word<'input>>,
        words: &[Word<'input>],
    ) -> Option<usize> {
        if let Some(next) = self.0.matches(matched_words, words) {
            return Some(next);
        }

        if let Some(next) = self.1.matches(matched_words, words) {
            return Some(next);
        }

        None
    }
}

/// A pattern that matches if both of two single word patterns match a single word.
#[derive(Copy, Clone, Debug)]
pub struct AndS<L: SingleWordPattern, R: SingleWordPattern>(pub L, pub R);

impl<L: SingleWordPattern, R: SingleWordPattern> SingleWordPattern for AndS<L, R> {
    fn matches_word(&self, word: &Word) -> bool {
        self.0.matches_word(word) && self.1.matches_word(word)
    }
}

/// A pattern that matches if any of a set of single word patterns match a single word.
#[derive(Copy, Clone)]
pub struct OneOfS<P, const N: usize>(pub [P; N]);

impl<P: SingleWordPattern, const N: usize> SingleWordPattern for OneOfS<P, N> {
    fn matches_word(&self, word: &Word) -> bool {
        self.0.iter().any(|pattern| pattern.matches_word(word))
    }
}

/// A pattern that matches if any of a set of multiple word patterns match a sequence of words.
#[derive(Copy, Clone, Debug)]
pub struct OneOf<P, const N: usize>(pub [P; N]);

impl<P: MultipleWordPattern, const N: usize> MultipleWordPattern for OneOf<P, N> {
    fn size_hint(&self) -> usize {
        self.0
            .iter()
            .map(|pattern| pattern.size_hint())
            .max()
            .unwrap_or(1)
    }

    fn matches<'input>(
        &self,
        matched_words: &mut Vec<Word<'input>>,
        words: &[Word<'input>],
    ) -> Option<usize> {
        for pattern in self.0.iter() {
            if let Some(next) = pattern.matches(matched_words, words) {
                return Some(next);
            }
        }

        None
    }
}

/// A pattern that matches an optional multiple word pattern.
#[derive(Copy, Clone, Debug)]
pub struct Opt<P>(pub P);

impl<P: MultipleWordPattern> MultipleWordPattern for Opt<P> {
    fn size_hint(&self) -> usize {
        self.0.size_hint()
    }

    fn matches<'input>(
        &self,
        matched_words: &mut Vec<Word<'input>>,
        words: &[Word<'input>],
    ) -> Option<usize> {
        if let Some(next) = self.0.matches(matched_words, words) {
            return Some(next);
        }

        Some(0)
    }
}

/// A top-level matcher that can match multiple word patterns while ignoring
/// certain words.
pub trait Matcher {
    /// The type of pattern that matches words to ignore during matching.
    type IgnorePattern: SingleWordPattern;

    /// Get the pattern that matches words to ignore.
    ///
    /// By default, this returns `None`, indicating that no words should be
    /// ignored.
    fn ignore_pattern(&self) -> Option<&Self::IgnorePattern> {
        None
    }

    /// The type of pattern to search for.
    type Pattern: MultipleWordPattern;

    /// Get the pattern to search for.
    fn pattern(&self) -> &Self::Pattern;
}

/// All multiple word patterns are also matchers, which do not ignore any words.
impl<P: MultipleWordPattern> Matcher for P {
    type IgnorePattern = Any;
    type Pattern = P;

    fn pattern(&self) -> &P {
        self
    }
}

/// A matcher that matches a sequence of multiple word patterns while ignoring
/// some words.
#[derive(Copy, Clone)]
pub struct Ignore<I, P>(pub I, pub P);

impl<I: SingleWordPattern, P: MultipleWordPattern> Matcher for Ignore<I, P> {
    type IgnorePattern = I;
    fn ignore_pattern(&self) -> Option<&Self::IgnorePattern> {
        Some(&self.0)
    }

    type Pattern = P;
    fn pattern(&self) -> &Self::Pattern {
        &self.1
    }
}

/// Find each sequence of words in `block` that match `pattern`, and call
/// `on_match` with the matched words.
pub fn match_words<'input, 'm, M>(
    block: &Block<Word<'input>>,
    matcher: &'m M,
    mut on_match: impl FnMut(&[Word<'input>]),
) where
    M: Matcher,
{
    let original_words = block.as_slice();

    let ignore_pattern = matcher.ignore_pattern();

    // If an ignore pattern is provided, filter out any words that match it.
    let ignored_words: Vec<Word> = match ignore_pattern {
        Some(ignore) => original_words
            .iter()
            .filter_map(|word| {
                if ignore.matches_word(word) {
                    None
                } else {
                    Some(*word)
                }
            })
            .collect(),
        None => Vec::new(),
    };

    // As an optimization, we can use a slice of the original words if no
    // `ignore_pattern` is provided.
    let words = match ignore_pattern {
        Some(_) => ignored_words.as_slice(),
        None => original_words,
    };

    let pattern = matcher.pattern();

    // Pre-allocate a vector to hold the matched words, based on the provided
    // size hint.
    let mut matched_words = Vec::with_capacity(pattern.size_hint());

    for i in 0..words.len() {
        if pattern.matches(&mut matched_words, &words[i..]).is_some() {
            on_match(&matched_words);
        }

        // Re-use the same buffer. The pattern may have modified the buffer even
        // if it didn't successfully match, so we need to clear the buffer on
        // every iteration.
        matched_words.clear();
    }
}

#[cfg(test)]
mod tests {
    use pastelito_model::Tag;

    use crate::{
        block::test::{with_testing_block, TestWord},
        matcher::match_words,
    };

    use super::{Any, EndsWithIgnoreCase, Ignore, IgnoreCase, Matcher, Opt, Or, TagFn};

    fn eq<P: Matcher>(pattern: P, expected: Vec<Vec<&str>>) {
        let words = &[
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
            TestWord::Word("big", Tag::Adjective),
            TestWord::Word(",", Tag::Comma),
            TestWord::Space,
            TestWord::Word("green", Tag::Adjective),
            TestWord::Space,
            TestWord::Word("mat", Tag::NounSingularOrMass),
            TestWord::Word(".", Tag::EndOfSentence),
        ];

        with_testing_block(words, |_, block| {
            let mut matches: Vec<Vec<&str>> = Vec::new();

            match_words(&block, &pattern, |words| {
                let strings = words.iter().map(|word| word.as_str()).collect();
                matches.push(strings);
            });

            assert_eq!(matches, expected);
        });
    }

    #[test]
    fn test_any() {
        eq(
            Any,
            vec![
                vec!["The"],
                vec!["cat"],
                vec!["sat"],
                vec!["on"],
                vec!["the"],
                vec!["big"],
                vec![","],
                vec!["green"],
                vec!["mat"],
                vec!["."],
            ],
        );
    }

    #[test]
    fn test_match_tag() {
        eq(Tag::Determiner, vec![vec!["The"], vec!["the"]]);
        eq(Tag::Adjective, vec![vec!["big"], vec!["green"]]);
    }

    #[test]
    fn test_match_literal() {
        eq("cat", vec![vec!["cat"]]);
    }

    #[test]
    fn test_multiple() {
        eq((Tag::Determiner, "cat"), vec![vec!["The", "cat"]]);
        eq(
            (Tag::Determiner, Any),
            vec![vec!["The", "cat"], vec!["the", "big"]],
        );
        eq(
            ("green", Tag::NounSingularOrMass),
            vec![vec!["green", "mat"]],
        );
        eq(
            (Tag::Determiner, "big", Tag::Comma, Tag::Adjective),
            vec![vec!["the", "big", ",", "green"]],
        );
    }

    #[test]
    fn test_or() {
        eq(
            Or(Tag::Determiner, "big"),
            vec![vec!["The"], vec!["the"], vec!["big"]],
        );
    }

    #[test]
    fn test_opt() {
        eq(
            (Tag::Determiner, Opt("very"), "big"),
            vec![vec!["the", "big"]],
        );
    }

    #[test]
    fn test_fn() {
        eq(
            TagFn(|tag| tag == Tag::Determiner),
            vec![vec!["The"], vec!["the"]],
        );
    }

    #[test]
    fn test_ignore() {
        eq(
            Ignore(Tag::Comma, (Tag::Adjective, Tag::Adjective)),
            vec![vec!["big", "green"]],
        )
    }

    #[test]
    fn test_backtracking() {
        eq(
            Or((Tag::Determiner, "dog"), (Tag::Determiner, "cat")),
            vec![vec!["The", "cat"]],
        );

        eq(
            Or(
                (Tag::Determiner, "cat", "lay"),
                (Tag::Determiner, "cat", "sat"),
            ),
            vec![vec!["The", "cat", "sat"]],
        );

        eq(
            Or(
                (Tag::Determiner, "cat", "sat", "by"),
                (Tag::Determiner, "cat", "sat", "on"),
            ),
            vec![vec!["The", "cat", "sat", "on"]],
        );
    }

    #[test]
    fn test_ignore_case() {
        eq(IgnoreCase::new("the"), vec![vec!["The"], vec!["the"]]);
        eq(IgnoreCase::new("BIG"), vec![vec!["big"]]);
    }

    #[test]
    fn test_ends_with_ignore_case() {
        eq(
            EndsWithIgnoreCase::new("aT"),
            vec![vec!["cat"], vec!["sat"], vec!["mat"]],
        );

        eq(EndsWithIgnoreCase::new("cat"), vec![])
    }

    #[should_panic]
    #[test]
    fn test_ends_with_ignore_case_non_ascii() {
        let _ = EndsWithIgnoreCase::new("🦕");
    }
}
