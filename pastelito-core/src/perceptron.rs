use pastelito_data::{ContextWord, Feature, Model, Scores, POS};
use strum::EnumCount as _;
use tracing::debug_span;

use crate::block::{Block, Word};

/// The context used by the perceptron.
///
/// This allows us to lookup the surrounding `ContextWords` for a given word
/// index. There are also special context markers for the start and end of the
/// block which this struct handles.
struct Context {
    tokens: Vec<Option<ContextWord>>,
}

impl Context {
    /// Create a new `Context` from the given block.
    fn new(block: &Block<'_, Word>) -> Self {
        let context_span = debug_span!("create context");
        context_span.in_scope(|| {
            let tokens = block
                .iter_with_str()
                .map(|(word, str)| ContextWord::new_from_word(str, word.pos))
                .collect::<Vec<_>>();
            Context { tokens }
        })
    }

    /// Get the `ContextWord` for the given index.
    fn word(&self, context_index: usize) -> Option<ContextWord> {
        let num_tokens = self.tokens.len();

        if context_index == 0 {
            return Some(ContextWord::START2);
        } else if context_index == 1 {
            return Some(ContextWord::START);
        }

        if context_index == num_tokens + 2 {
            Some(ContextWord::END)
        } else if context_index == num_tokens + 3 {
            Some(ContextWord::END2)
        } else {
            self.tokens[context_index - 2]
        }
    }
}

/// A perceptron.
pub struct Perceptron {
    model: &'static Model,
}

impl Perceptron {
    /// Create a new perceptron with the given model.
    pub fn new(model: &'static Model) -> Self {
        Self { model }
    }

    /// Predict the POS tags for all words in the given block.
    ///
    /// This should be run after the "static tags" phase. The words are modified
    /// in place.
    pub fn predict(&self, block: &mut Block<'_, Word>) {
        let context = Context::new(block);

        let mut p1 = POS::Start;
        let mut p2 = POS::Start2;

        for (i, (word, str)) in block.iter_mut_with_str().enumerate() {
            let next_p1 = match word.pos {
                None => {
                    // Only predict if the POS tag is currently unknown.
                    let pos = self.predict_one(&context, i, str, p1, p2);
                    word.pos = Some(pos);
                    pos
                }
                Some(pos) => pos,
            };

            p2 = p1;
            p1 = next_p1;
        }
    }

    /// Predict the POS tag for a single word.
    fn predict_one(
        &self,
        context: &Context,
        word_index: usize,
        token: &str,
        p1: POS,
        p2: POS,
    ) -> POS {
        let context_index = word_index + 2;
        let mut features = Vec::with_capacity(Feature::COUNT);

        features.push(Feature::Bias);

        if let Ok(suffix) = token.try_into() {
            features.push(Feature::Suffix(suffix));
        }
        if let Some(c) = token.chars().next().unwrap().as_ascii() {
            features.push(Feature::Pref1(c.to_u8()));
        }
        features.push(Feature::IMinus1Tag(p1));
        features.push(Feature::IMinus2Tag(p2));
        features.push(Feature::ITagPlusIMinus2Tag(p1, p2));

        if let Some(word) = context.word(context_index) {
            features.push(Feature::IWord(word));
            features.push(Feature::IMinus1TagPlusIWord(p1, word));
        }

        if let Some(word) = context.word(context_index - 1) {
            features.push(Feature::IMinus1Word(word));
            features.push(Feature::IMinus1Suffix(word.suffix()));
        }

        if let Some(word) = context.word(context_index - 2) {
            features.push(Feature::IMinus2Word(word));
        }

        if let Some(word) = context.word(context_index + 1) {
            features.push(Feature::IPlus1Word(word));
            features.push(Feature::IPlus1Suffix(word.suffix()));
        }

        if let Some(word) = context.word(context_index + 2) {
            features.push(Feature::IPlus2Word(word));
        }

        let mut scores = Scores::default();

        for feature in features {
            if let Some(weights) = self.model.get(&feature) {
                for (pos, weight) in weights {
                    scores.update(*pos, *weight);
                }
            }
        }

        scores.max()
    }
}
