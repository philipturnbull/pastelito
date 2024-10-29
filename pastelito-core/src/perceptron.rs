use pastelito_model::{ContextWord, Feature, Model, Tag};
use strum::EnumCount as _;

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
    fn new(block: &Block<Word>) -> Self {
        let tokens = [Some(ContextWord::START2), Some(ContextWord::START)]
            .into_iter()
            .chain(
                block
                    .iter()
                    .map(|word| ContextWord::new_from_word(word.as_str(), word.tag())),
            )
            .chain([Some(ContextWord::END), Some(ContextWord::END2)])
            .collect::<Vec<_>>();
        Context { tokens }
    }

    /// Get the context window around the given word index.
    ///
    /// This includes the two words before and after the given index.
    #[allow(clippy::type_complexity)]
    fn window(
        &self,
        i: usize,
    ) -> (
        Option<ContextWord>,
        Option<ContextWord>,
        Option<ContextWord>,
        Option<ContextWord>,
        Option<ContextWord>,
    ) {
        (
            self.tokens[i - 2],
            self.tokens[i - 1],
            self.tokens[i],
            self.tokens[i + 1],
            self.tokens[i + 2],
        )
    }
}

/// The weights for a single word.
struct WordWeights {
    model: &'static Model,
    weights: Vec<&'static [(Tag, f32)]>,
}

impl WordWeights {
    /// Create a new `WordWeights` for the given model and zero initial weights.
    fn new(model: &'static Model) -> Self {
        WordWeights {
            model,
            weights: Vec::with_capacity(Feature::COUNT),
        }
    }

    /// Clear the current weights vector.
    fn clear(&mut self) {
        self.weights.clear();
    }

    /// Push the weights for the given feature into the weights vector.
    fn push(&mut self, feature: &Feature) {
        if let Some(weights) = self.model.get(feature) {
            self.weights.push(weights);
        }
    }

    /// Get the current weights of this word.
    fn as_slice(&self) -> &[&'static [(Tag, f32)]] {
        &self.weights
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

    /// Predict the tags for all words in the given block.
    ///
    /// This should be run after the "static tags" phase. The words are modified
    /// in place.
    pub fn predict(&self, block: &mut Block<Word>) {
        let context = Context::new(block);
        let mut weights = WordWeights::new(self.model);

        let mut t1 = Tag::Start;
        let mut t2 = Tag::Start2;

        for (i, word) in block.iter_mut().enumerate() {
            let next_t1 = match word.tag() {
                None => {
                    // Only predict if the tag is currently unknown.
                    let tag = self.predict_one(&mut weights, &context, i, word.as_str(), t1, t2);
                    word.set_tag(tag);
                    tag
                }
                Some(tag) => tag,
            };

            t2 = t1;
            t1 = next_t1;
        }
    }

    /// Predict the tag for a single word.
    fn predict_one(
        &self,
        weights: &mut WordWeights,
        context: &Context,
        word_index: usize,
        token: &str,
        t1: Tag,
        t2: Tag,
    ) -> Tag {
        weights.clear();

        if let Ok(suffix) = token.try_into() {
            weights.push(&Feature::Suffix(suffix));
        }
        if let Some(c) = token.chars().next().unwrap().as_ascii() {
            weights.push(&Feature::Pref1(c.to_u8()));
        }
        weights.push(&Feature::IMinus1Tag(t1));
        weights.push(&Feature::IMinus2Tag(t2));
        weights.push(&Feature::ITagPlusIMinus2Tag(t1, t2));

        let (minus2, minus1, current, plus1, plus2) = context.window(word_index + 2);

        if let Some(word) = minus2 {
            weights.push(&Feature::IMinus2Word(word));
        }

        if let Some(word) = minus1 {
            weights.push(&Feature::IMinus1Word(word));
            weights.push(&Feature::IMinus1Suffix(word.suffix()));
        }

        if let Some(word) = current {
            weights.push(&Feature::IWord(word));
            weights.push(&Feature::IMinus1TagPlusIWord(t1, word));
        }

        if let Some(word) = plus1 {
            weights.push(&Feature::IPlus1Word(word));
            weights.push(&Feature::IPlus1Suffix(word.suffix()));
        }

        if let Some(word) = plus2 {
            weights.push(&Feature::IPlus2Word(word));
        }

        let mut scores = self.model.initial_scores();

        for w in weights.as_slice() {
            for (tag, weight) in *w {
                scores.update(*tag, *weight);
            }
        }

        scores.max()
    }
}
