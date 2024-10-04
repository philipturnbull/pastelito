//! Pastelito data structures.
#![feature(ascii_char)]
use speedy::{Readable, Writable};
use std::str::FromStr;
use std::{collections::HashMap, ops::Range};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, EnumString, IntoStaticStr};

/// A part of speech.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    Readable,
    Writable,
    EnumCount,
    EnumIter,
    EnumString,
    IntoStaticStr,
)]
pub enum POS {
    #[strum(serialize = "-START-")]
    Start,
    #[strum(serialize = "-START2-")]
    Start2,
    #[strum(serialize = "-END-")]
    End,
    #[strum(serialize = "-END2-")]
    End2,
    #[strum(serialize = "#")]
    Hash,
    #[strum(serialize = "$")]
    Dollar,
    #[strum(serialize = "''")]
    TwoQuotes,
    #[strum(serialize = "(")]
    Open,
    #[strum(serialize = ")")]
    Close,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ".")]
    EndOfSentence,
    #[strum(serialize = ":")]
    Colon,
    #[strum(serialize = "CC")]
    CoordinatingConjunction,
    #[strum(serialize = "CD")]
    CardinalNumber,
    #[strum(serialize = "DT")]
    Determiner,
    #[strum(serialize = "EX")]
    ExistentialThere,
    #[strum(serialize = "FW")]
    ForeignWord,
    #[strum(serialize = "IN")]
    PrepositionOrSubordinatingConjunction,
    #[strum(serialize = "JJ")]
    Adjective,
    #[strum(serialize = "JJR")]
    AdjectiveComparative,
    #[strum(serialize = "JJS")]
    AdjectiveSuperlative,
    #[strum(serialize = "LS")]
    ListItemMarker,
    #[strum(serialize = "MD")]
    Modal,
    #[strum(serialize = "NN")]
    NounSingularOrMass,
    #[strum(serialize = "NNP")]
    ProperNounSingular,
    #[strum(serialize = "NNPS")]
    ProperNounPlural,
    #[strum(serialize = "NNS")]
    NounPlural,
    #[strum(serialize = "PDT")]
    Predeterminer,
    #[strum(serialize = "POS")]
    PossessiveEnding,
    #[strum(serialize = "PRP")]
    PersonalPronoun,
    #[strum(serialize = "PRP$")]
    PossesivePronoun,
    #[strum(serialize = "RB")]
    Adverb,
    #[strum(serialize = "RBR")]
    AdverbComparative,
    #[strum(serialize = "RBS")]
    AdverbSuperlative,
    #[strum(serialize = "RP")]
    Particle,
    #[strum(serialize = "SYM")]
    Symbol,
    #[strum(serialize = "TO")]
    To,
    #[strum(serialize = "UH")]
    Interjection,
    #[strum(serialize = "VB")]
    VerbBaseForm,
    #[strum(serialize = "VBD")]
    VerbPastTense,
    #[strum(serialize = "VBG")]
    VerbGerundOrPresentParticiple,
    #[strum(serialize = "VBN")]
    VerbPastParticiple,
    #[strum(serialize = "VBP")]
    VerbNon3rdPersonSingularPresent,
    #[strum(serialize = "VBZ")]
    Verb3rdPersonSingularPresent,
    #[strum(serialize = "WDT")]
    WhDeterminer,
    #[strum(serialize = "WP")]
    WhPronoun,
    #[strum(serialize = "WP$")]
    PossesiveWhPronoun,
    #[strum(serialize = "WRB")]
    WhAdverb,
    #[strum(serialize = "``")]
    Backtick,
}

/// A mapping of `POS` to a score.
#[derive(Copy, Clone, Readable, Writable)]
pub struct Scores {
    scores: [(POS, f32); POS::COUNT],
}

impl Scores {
    /// Add `score` to the current score for `pos`.
    pub fn update(&mut self, pos: POS, score: f32) {
        self.scores.iter_mut().find(|(p, _)| *p == pos).unwrap().1 += score;
    }

    /// Get the `POS` with the highest score.
    pub fn max(&self) -> POS {
        self.scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
    }
}

impl Default for Scores {
    fn default() -> Self {
        Scores {
            scores: [
                (POS::Start, 0.0),
                (POS::Start2, 0.0),
                (POS::End, 0.0),
                (POS::End2, 0.0),
                (POS::Hash, 0.0),
                (POS::Dollar, 0.0),
                (POS::TwoQuotes, 0.0),
                (POS::Open, 0.0),
                (POS::Close, 0.0),
                (POS::Comma, 0.0),
                (POS::EndOfSentence, 0.0),
                (POS::Colon, 0.0),
                (POS::CoordinatingConjunction, 0.0),
                (POS::CardinalNumber, 0.0),
                (POS::Determiner, 0.0),
                (POS::ExistentialThere, 0.0),
                (POS::ForeignWord, 0.0),
                (POS::PrepositionOrSubordinatingConjunction, 0.0),
                (POS::Adjective, 0.0),
                (POS::AdjectiveComparative, 0.0),
                (POS::AdjectiveSuperlative, 0.0),
                (POS::ListItemMarker, 0.0),
                (POS::Modal, 0.0),
                (POS::NounSingularOrMass, 0.0),
                (POS::ProperNounSingular, 0.0),
                (POS::ProperNounPlural, 0.0),
                (POS::NounPlural, 0.0),
                (POS::Predeterminer, 0.0),
                (POS::PossessiveEnding, 0.0),
                (POS::PersonalPronoun, 0.0),
                (POS::PossesivePronoun, 0.0),
                (POS::Adverb, 0.0),
                (POS::AdverbComparative, 0.0),
                (POS::AdverbSuperlative, 0.0),
                (POS::Particle, 0.0),
                (POS::Symbol, 0.0),
                (POS::To, 0.0),
                (POS::Interjection, 0.0),
                (POS::VerbBaseForm, 0.0),
                (POS::VerbPastTense, 0.0),
                (POS::VerbGerundOrPresentParticiple, 0.0),
                (POS::VerbPastParticiple, 0.0),
                (POS::VerbNon3rdPersonSingularPresent, 0.0),
                (POS::Verb3rdPersonSingularPresent, 0.0),
                (POS::WhDeterminer, 0.0),
                (POS::WhPronoun, 0.0),
                (POS::PossesiveWhPronoun, 0.0),
                (POS::WhAdverb, 0.0),
                (POS::Backtick, 0.0),
            ],
        }
    }
}

/// The suffix of a `ContextWord`. This can be one, two or three ASCII characters long.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Readable, Writable)]
pub struct ContextSuffix {
    /// The characters of the suffix. These are stored right-aligned and padded
    /// with leading zeroes.
    chars: [u8; 3],
}

impl ContextSuffix {
    pub fn new(chars: [u8; 3]) -> Self {
        ContextSuffix { chars }
    }
}

impl TryFrom<&str> for ContextSuffix {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let num_chars = s.chars().count();
        if num_chars == 0 {
            panic!("Empty string");
        }

        let mut chars = [u8::default(); 3];

        let num_chars_to_skip = num_chars - 3.min(num_chars);
        let offset = 3usize.saturating_sub(num_chars);
        for (i, c) in s.chars().skip(num_chars_to_skip).enumerate() {
            if let Some(c) = c.as_ascii() {
                chars[offset + i] = c.to_u8();
            } else {
                return Err(());
            }
        }
        Ok(ContextSuffix { chars })
    }
}

/// A word of "context" for the perceptron. This can be up to
/// `ContextWord::LENGTH`` ASCII characters long.
///
/// For most input tokens, this is just the token itself but some tokens are
/// replaced with a marker. For example, tokens that are numbers are replaced
/// with a `!DIGITS` marker, and tokens that contain hyphens are replaced with a
/// `!HYPHEN` marker.
///
/// There are also special markers for tokens which don't appear in the input
/// stream: `CONTEXT_START`, `CONTEXT_START2`, `CONTEXT_END`, and
/// `CONTEXT_END2` which represent tokens before and after the input stream.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Readable, Writable)]
pub struct ContextWord {
    /// The data is stored right-aligned in the array and padded with leading
    /// zeros. This allows us to get the suffix of a `ContextWord` in constant
    /// time.
    chars: [u8; ContextWord::LENGTH],
}

impl ContextWord {
    /// The maximum length of a `ContextWord` in bytes.
    pub const LENGTH: usize = 23;

    /// Create a new `ContextWord` from an input token.
    ///
    /// This can return `None` if the input token contains non-ASCII characters
    /// or is too long.
    pub fn new_from_word(token: &str, pos: Option<POS>) -> Option<Self> {
        if pos == Some(POS::CardinalNumber) {
            if token.len() == 4 && token.chars().all(|c| c.is_ascii_digit()) {
                return Some(Self::YEAR);
            } else {
                return Some(Self::DIGITS);
            }
        }

        if token.find('-').map(|i| i > 0).unwrap_or(false) {
            return Some(Self::HYPHEN);
        }

        let num_bytes = token.len();
        if num_bytes > ContextWord::LENGTH {
            return None;
        }

        let num_chars = token.chars().count();
        if num_chars != num_bytes {
            // Token contains non-ASCII characters
            return None;
        }

        let mut chars = [0; ContextWord::LENGTH];
        let bytes = token.as_bytes();
        for (i, b) in bytes.iter().enumerate() {
            chars[ContextWord::LENGTH - num_bytes + i] = *b;
        }

        Some(ContextWord { chars })
    }

    /// Create a new `ContextWord` from a model token.
    ///
    /// This will panic if the token is invalid.
    pub fn new_from_model(token: &str) -> Self {
        Self::new_from_word(token, None).expect("Invalid word in model")
    }

    /// Get the suffix of the word.
    pub fn suffix(&self) -> ContextSuffix {
        ContextSuffix {
            chars: [
                self.chars[ContextWord::LENGTH - 3],
                self.chars[ContextWord::LENGTH - 2],
                self.chars[ContextWord::LENGTH - 1],
            ],
        }
    }

    const YEAR: ContextWord = ContextWord {
        chars: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 89, 69, 65, 82,
        ],
    };

    const DIGITS: ContextWord = ContextWord {
        chars: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 68, 73, 71, 73, 84, 83,
        ],
    };

    const HYPHEN: ContextWord = ContextWord {
        chars: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 72, 89, 80, 72, 69, 78,
        ],
    };

    pub const START: ContextWord = ContextWord {
        chars: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 83, 84, 65, 82, 84, 45,
        ],
    };

    pub const START2: ContextWord = ContextWord {
        chars: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 83, 84, 65, 82, 84, 50, 45,
        ],
    };

    pub const END: ContextWord = ContextWord {
        chars: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 69, 78, 68, 45,
        ],
    };

    pub const END2: ContextWord = ContextWord {
        chars: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 69, 78, 68, 50, 45,
        ],
    };
}

/// A feature for the perceptron
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Readable, Writable)]
pub enum Feature {
    Bias,
    Suffix(ContextSuffix),
    Pref1(u8),
    IMinus1Tag(POS),
    IMinus2Tag(POS),
    ITagPlusIMinus2Tag(POS, POS),
    IWord(ContextWord),
    IMinus1TagPlusIWord(POS, ContextWord),
    IMinus1Word(ContextWord),
    IMinus1Suffix(ContextSuffix),
    IMinus2Word(ContextWord),
    IPlus1Word(ContextWord),
    IPlus1Suffix(ContextSuffix),
    IPlus2Word(ContextWord),
}

impl From<String> for Feature {
    fn from(s: String) -> Self {
        let s = s.as_str();

        if s == "bias" {
            Feature::Bias
        } else if let Some(value) = s.strip_prefix("i suffix ") {
            Feature::Suffix(value.try_into().unwrap())
        } else if let Some(value) = s.strip_prefix("i pref1 ") {
            Feature::Pref1(value.chars().next().unwrap().as_ascii().unwrap().to_u8())
        } else if let Some(value) = s.strip_prefix("i-1 suffix ") {
            Feature::IMinus1Suffix(value.try_into().unwrap())
        } else if let Some(value) = s.strip_prefix("i-1 tag ") {
            Feature::IMinus1Tag(POS::from_str(value).unwrap())
        } else if let Some(value) = s.strip_prefix("i-2 tag ") {
            Feature::IMinus2Tag(POS::from_str(value).unwrap())
        } else if let Some(value) = s.strip_prefix("i tag+i-2 tag ") {
            let parts = value.split_once(' ').unwrap();
            Feature::ITagPlusIMinus2Tag(
                POS::from_str(parts.0).unwrap(),
                POS::from_str(parts.1).unwrap(),
            )
        } else if let Some(value) = s.strip_prefix("i word ") {
            Feature::IWord(ContextWord::new_from_model(value))
        } else if let Some(value) = s.strip_prefix("i-1 tag+i word ") {
            let parts = value.split_once(' ').unwrap();
            Feature::IMinus1TagPlusIWord(
                POS::from_str(parts.0).unwrap(),
                ContextWord::new_from_model(parts.1),
            )
        } else if let Some(value) = s.strip_prefix("i-1 word ") {
            Feature::IMinus1Word(ContextWord::new_from_model(value))
        } else if let Some(value) = s.strip_prefix("i+1 word ") {
            Feature::IPlus1Word(ContextWord::new_from_model(value))
        } else if let Some(value) = s.strip_prefix("i+1 suffix ") {
            Feature::IPlus1Suffix(value.try_into().unwrap())
        } else if let Some(value) = s.strip_prefix("i+2 word ") {
            Feature::IPlus2Word(ContextWord::new_from_model(value))
        } else if let Some(value) = s.strip_prefix("i-2 word ") {
            Feature::IMinus2Word(ContextWord::new_from_model(value))
        } else {
            // This is only used to parse the model file, so we expect all
            // features to be valid
            panic!("Unknown feature in model: {}", s);
        }
    }
}

/// The `IsFeature` trait allows us to abstract over how a feature is encoded.
pub trait IsFeature {
    type Output;

    /// The maximum number of features
    fn max_features() -> usize;
    /// Create a `bias`` feature
    fn bias() -> Self::Output;
    /// Create a `suffix` feature
    fn suffix(suffix: ContextSuffix) -> Self::Output;
    /// Create a `pref1` feature
    fn pref1(c: u8) -> Self::Output;
    /// Create an `i-1 tag` feature
    fn iminus1tag(pos: POS) -> Self::Output;
    /// Create an `i-2 tag` feature
    fn iminus2tag(pos: POS) -> Self::Output;
    /// Create an `i tag+i-2 tag` feature
    fn itagplusiminus2tag(pos1: POS, pos2: POS) -> Self::Output;
    /// Create an `i word` feature
    fn iword(word: ContextWord) -> Self::Output;
    /// Create an `i-1 tag+i word` feature
    fn iminus1tagplusiword(pos: POS, word: ContextWord) -> Self::Output;
    /// Create an `i-1 word` feature
    fn iminus1word(word: ContextWord) -> Self::Output;
    /// Create an `i-1 suffix` feature
    fn iminus1suffix(suffix: ContextSuffix) -> Self::Output;
    /// Create an `i-2 word` feature
    fn iminus2word(word: ContextWord) -> Self::Output;
    /// Create an `i+1 word` feature
    fn iplus1word(word: ContextWord) -> Self::Output;
    /// Create an `i+1 suffix` feature
    fn iplus1suffix(suffix: ContextSuffix) -> Self::Output;
    /// Create an `i+2 word` feature
    fn iplus2word(word: ContextWord) -> Self::Output;
}

impl IsFeature for Feature {
    type Output = Feature;

    fn max_features() -> usize {
        14
    }

    fn bias() -> Self::Output {
        Feature::Bias
    }

    fn suffix(suffix: ContextSuffix) -> Self::Output {
        Feature::Suffix(suffix)
    }

    fn pref1(c: u8) -> Self::Output {
        Feature::Pref1(c)
    }

    fn iminus1tag(pos: POS) -> Self::Output {
        Feature::IMinus1Tag(pos)
    }

    fn iminus2tag(pos: POS) -> Self::Output {
        Feature::IMinus2Tag(pos)
    }

    fn itagplusiminus2tag(pos1: POS, pos2: POS) -> Self::Output {
        Feature::ITagPlusIMinus2Tag(pos1, pos2)
    }

    fn iword(word: ContextWord) -> Self::Output {
        Feature::IWord(word)
    }

    fn iminus1tagplusiword(pos: POS, word: ContextWord) -> Self::Output {
        Feature::IMinus1TagPlusIWord(pos, word)
    }

    fn iminus1word(word: ContextWord) -> Self::Output {
        Feature::IMinus1Word(word)
    }

    fn iminus1suffix(suffix: ContextSuffix) -> Self::Output {
        Feature::IMinus1Suffix(suffix)
    }

    fn iminus2word(word: ContextWord) -> Self::Output {
        Feature::IMinus2Word(word)
    }

    fn iplus1word(word: ContextWord) -> Self::Output {
        Feature::IPlus1Word(word)
    }

    fn iplus1suffix(suffix: ContextSuffix) -> Self::Output {
        Feature::IPlus1Suffix(suffix)
    }

    fn iplus2word(word: ContextWord) -> Self::Output {
        Feature::IPlus2Word(word)
    }
}

fn format_context_suffix(suffix: ContextSuffix) -> String {
    suffix
        .chars
        .iter()
        .filter_map(|&b| if b > 0 { Some(b as char) } else { None })
        .collect()
}

fn format_word(word: ContextWord) -> String {
    word.chars
        .iter()
        .filter_map(|&b| if b > 0 { Some(b as char) } else { None })
        .collect()
}

pub fn max_features() -> usize {
    14
}

pub fn bias() -> String {
    "bias".to_string()
}

pub fn suffix(suffix: ContextSuffix) -> String {
    format!("i suffix {}", format_context_suffix(suffix))
}

pub fn pref1(c: u8) -> String {
    format!("i pref1 {}", c as char)
}

pub fn iminus1tag(pos: POS) -> String {
    let tag: &str = pos.into();
    format!("i-1 tag {}", tag)
}

pub fn iminus2tag(pos: POS) -> String {
    let tag: &str = pos.into();
    format!("i-2 tag {}", tag)
}

pub fn itagplusiminus2tag(pos1: POS, pos2: POS) -> String {
    let tag1: &str = pos1.into();
    let tag2: &str = pos2.into();
    format!("i tag+i-2 tag {} {}", tag1, tag2)
}

pub fn iword(word: ContextWord) -> String {
    format!("i word {}", format_word(word))
}

pub fn iminus1tagplusiword(pos: POS, word: ContextWord) -> String {
    let tag: &str = pos.into();
    format!("i-1 tag+i word {} {}", tag, format_word(word))
}

pub fn iminus1word(word: ContextWord) -> String {
    format!("i-1 word {}", format_word(word))
}

pub fn iminus1suffix(suffix: ContextSuffix) -> String {
    format!("i-1 suffix {}", format_context_suffix(suffix))
}

pub fn iminus2word(word: ContextWord) -> String {
    format!("i-2 word {}", format_word(word))
}

pub fn iplus1word(word: ContextWord) -> String {
    format!("i+1 word {}", format_word(word))
}

pub fn iplus1suffix(suffix: ContextSuffix) -> String {
    format!("i+1 suffix {}", format_context_suffix(suffix))
}

pub fn iplus2word(word: ContextWord) -> String {
    format!("i+2 word {}", format_word(word))
}

/// A range of weights in the model
///
/// This is equivalent to a `Range<usize>` but with `u32` instead of `usize` to
/// save some memory on disk.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Readable, Writable)]
pub struct WeightRange {
    start: u32,
    end: u32,
}

impl WeightRange {
    /// Create a new `WeightRange``
    ///
    /// No validation is done on the input values
    pub fn new(start: usize, end: usize) -> Self {
        WeightRange {
            start: start as u32,
            end: end as u32,
        }
    }

    /// Get the range as a `Range<usize>`
    pub fn as_range(&self) -> Range<usize> {
        (self.start as usize)..(self.end as usize)
    }
}

/// A model for the perceptron
#[derive(Readable, Writable)]
pub struct Model {
    static_tags: HashMap<String, POS>,

    weights: Vec<(POS, f32)>,
    mapping: HashMap<String, WeightRange>,
    initial_scores: Scores,
}

impl Model {
    /// Create a new model.
    ///
    /// Users should typically use the model defined in `pastelito-model`
    /// instead of creating their own model.
    pub fn new(
        static_tags: HashMap<String, POS>,
        weights: Vec<(POS, f32)>,
        mapping: HashMap<String, WeightRange>,
        initial_scores: Scores,
    ) -> Self {
        Model {
            static_tags,
            weights,
            mapping,
            initial_scores,
        }
    }

    /// Get the initial scores that should be used when predicting a word
    pub fn initial_scores(&self) -> Scores {
        self.initial_scores
    }

    /// Get the static tag for a word
    pub fn get_static_tag(&self, word: &str) -> Option<POS> {
        self.static_tags.get(word).copied()
    }

    /// Get the weights for a feature
    pub fn get_feature(&self, feature: &str) -> Option<&[(POS, f32)]> {
        let range = self.mapping.get(feature)?;
        self.weights.get(range.as_range())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ContextSuffix, ContextWord, Feature, POS};

    fn eq(data: &str, expected: Feature) {
        let actual: Feature = data.to_owned().into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_feature() {
        eq("bias", Feature::Bias);

        eq(
            "i suffix a",
            Feature::Suffix(ContextSuffix::new([0, 0, 97])),
        );
        eq(
            "i suffix ab",
            Feature::Suffix(ContextSuffix::new([0, 97, 98])),
        );
        eq(
            "i suffix abc",
            Feature::Suffix(ContextSuffix::new([97, 98, 99])),
        );
        eq(
            "i suffix abcd",
            Feature::Suffix(ContextSuffix::new([98, 99, 100])),
        );

        eq(
            "i-1 suffix a",
            Feature::IMinus1Suffix(ContextSuffix::new([0, 0, 97])),
        );
        eq(
            "i-1 suffix ab",
            Feature::IMinus1Suffix(ContextSuffix::new([0, 97, 98])),
        );
        eq(
            "i-1 suffix abc",
            Feature::IMinus1Suffix(ContextSuffix::new([97, 98, 99])),
        );
        eq(
            "i-1 suffix abcd",
            Feature::IMinus1Suffix(ContextSuffix::new([98, 99, 100])),
        );

        eq(
            "i-1 tag CC",
            Feature::IMinus1Tag(POS::CoordinatingConjunction),
        );

        eq(
            "i-2 tag CC",
            Feature::IMinus2Tag(POS::CoordinatingConjunction),
        );

        eq(
            "i tag+i-2 tag CC CC",
            Feature::ITagPlusIMinus2Tag(POS::CoordinatingConjunction, POS::CoordinatingConjunction),
        );

        eq(
            "i word a",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97,
                ],
            }),
        );
        eq(
            "i word ab",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98,
                ],
            }),
        );
        eq(
            "i word abc",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99,
                ],
            }),
        );
        eq(
            "i word abcd",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100,
                ],
            }),
        );
        eq(
            "i word abcde",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101,
                ],
            }),
        );
        eq(
            "i word abcdef",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102,
                ],
            }),
        );
        eq(
            "i word abcdefg",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103,
                ],
            }),
        );
        eq(
            "i word abcdefgh",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103,
                    104,
                ],
            }),
        );
        eq(
            "i word abcdefghi",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104,
                    105,
                ],
            }),
        );
        eq(
            "i word abcdefghij",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104,
                    105, 106,
                ],
            }),
        );
        eq(
            "i word abcdefghijk",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105,
                    106, 107,
                ],
            }),
        );
        eq(
            "i word abcdefghijkl",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
                    107, 108,
                ],
            }),
        );
        eq(
            "i word abcdefghijklm",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106,
                    107, 108, 109,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmn",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
                    108, 109, 110,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmno",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107,
                    108, 109, 110, 111,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmnop",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
                    109, 110, 111, 112,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmnopq",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
                    110, 111, 112, 113,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmnopqr",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
                    110, 111, 112, 113, 114,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmnopqrs",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                    111, 112, 113, 114, 115,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmnopqrst",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                    111, 112, 113, 114, 115, 116,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmnopqrstu",
            Feature::IWord(ContextWord {
                chars: [
                    0, 0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
                    112, 113, 114, 115, 116, 117,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmnopqrstuv",
            Feature::IWord(ContextWord {
                chars: [
                    0, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
                    113, 114, 115, 116, 117, 118,
                ],
            }),
        );
        eq(
            "i word abcdefghijklmnopqrstuvw",
            Feature::IWord(ContextWord {
                chars: [
                    97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
                    113, 114, 115, 116, 117, 118, 119,
                ],
            }),
        );

        eq(
            "i-1 tag+i word CC a",
            Feature::IMinus1TagPlusIWord(
                POS::CoordinatingConjunction,
                ContextWord {
                    chars: [
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97,
                    ],
                },
            ),
        );

        eq(
            "i-1 word a",
            Feature::IMinus1Word(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97,
                ],
            }),
        );

        eq(
            "i+1 word a",
            Feature::IPlus1Word(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97,
                ],
            }),
        );

        eq(
            "i+1 suffix a",
            Feature::IPlus1Suffix(ContextSuffix::new([0, 0, 97])),
        );
        eq(
            "i+1 suffix ab",
            Feature::IPlus1Suffix(ContextSuffix::new([0, 97, 98])),
        );
        eq(
            "i+1 suffix abc",
            Feature::IPlus1Suffix(ContextSuffix::new([97, 98, 99])),
        );

        eq(
            "i+2 word a",
            Feature::IPlus2Word(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97,
                ],
            }),
        );

        eq(
            "i-2 word a",
            Feature::IMinus2Word(ContextWord {
                chars: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 97,
                ],
            }),
        );
    }

    #[should_panic]
    #[test]
    fn test_overlong_word() {
        let _: Feature = "i word abcdefghijklmnopqrstuvwx".to_owned().into();
    }

    #[should_panic]
    #[test]
    fn unknown_feature() {
        let _: Feature = "unknown".to_owned().into();
    }
}
