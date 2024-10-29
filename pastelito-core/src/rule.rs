use std::collections::HashMap;

#[allow(unused_imports)]
use strum::VariantArray as _;
use strum_macros::VariantArray;
use tracing::debug_span;

use crate::{
    block::Word,
    doc::Document,
    lines::spans_to_ranges,
    matcher::{match_words, Matcher, SingleWordPattern},
    measures::default_measures,
    rules::default_rules,
    span::ByteSpan,
    LineCharRange,
};

/// This structure has a `ByteSpan`.
pub(crate) trait HasSpan {
    fn span(&self) -> ByteSpan;
}

/// A single finding from a rule which indicates a possible error in the
/// document.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Warning {
    /// The span of the warning.
    pub span: ByteSpan,
    /// The message associated with the warning.
    pub message: String,
}

impl HasSpan for &Warning {
    fn span(&self) -> ByteSpan {
        self.span
    }
}

/// A builder for `Warning`.
pub struct WarningBuilder {
    span: ByteSpan,
    message: Option<String>,
}

impl WarningBuilder {
    /// Create a new builder for the given words.
    ///
    /// The span of the final `Warning` will be the span covering all the words.
    pub fn new(words: &[Word]) -> Self {
        WarningBuilder {
            span: words.into(),
            message: None,
        }
    }

    /// Set the message for the warning.
    ///
    /// A message is required.
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    /// Build the `Warning`.
    pub fn build(self) -> Warning {
        Warning {
            span: self.span,
            message: self.message.expect("message is required"),
        }
    }
}

/// Build a set of warnings for a single document.
#[derive(Debug, Default)]
pub struct WarningsBuilder {
    warnings: Vec<Warning>,
}

impl WarningsBuilder {
    /// Add a new warning.
    pub fn add_warning(&mut self, result: Warning) {
        self.warnings.push(result);
    }

    fn build(self) -> Vec<Warning> {
        let mut warnings = self.warnings;
        warnings.sort();
        warnings
    }
}

/// A unique id for a measure.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, VariantArray)]
#[repr(u8)]
pub enum MeasureKey {
    AbstractNouns,
    AcademicAdWords,
    Adjectives,
    BeVerbs,
    Prepositions,
}

#[cfg(test)]
impl quickcheck::Arbitrary for MeasureKey {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(MeasureKey::VARIANTS).unwrap()
    }
}

/// An instance of a measurement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Measurement<'input> {
    /// The word that was measured.
    pub word: Word<'input>,
    /// The key for the measurement.
    pub key: MeasureKey,
}

impl PartialOrd for Measurement<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Measurement<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.word
            .as_span()
            .cmp(&other.word.as_span())
            .then_with(|| self.key.cmp(&other.key))
    }
}

impl<'input> Measurement<'input> {
    /// Create a new measurement.
    fn new(key: MeasureKey, word: Word<'input>) -> Self {
        Measurement { word, key }
    }
}

impl HasSpan for &Measurement<'_> {
    fn span(&self) -> ByteSpan {
        self.word.as_span()
    }
}

/// Build a set of measurements for a single document.
#[derive(Debug, Default)]
pub struct MeasurementsBuilder<'input> {
    measurements: HashMap<MeasureKey, Vec<Word<'input>>>,
}

impl<'input> MeasurementsBuilder<'input> {
    /// Add a new measurement to the builder.
    pub fn add_measurement(&mut self, key: MeasureKey, word: &Word<'input>) {
        self.measurements.entry(key).or_default().push(*word);
    }

    fn build(self) -> Vec<Measurement<'input>> {
        let mut measurements = self
            .measurements
            .into_iter()
            .flat_map(|(key, words)| {
                words
                    .into_iter()
                    .map(move |word| Measurement::new(key, word))
            })
            .collect::<Vec<_>>();
        measurements.sort();
        measurements
    }
}

/// A builder for `Results`.
#[derive(Debug)]
struct ResultsBuilder<'input> {
    input: &'input str,
    warnings_builder: WarningsBuilder,
    measurements_builder: MeasurementsBuilder<'input>,
}

impl<'input> ResultsBuilder<'input> {
    fn new(input: &'input str) -> Self {
        ResultsBuilder {
            input,
            warnings_builder: WarningsBuilder::default(),
            measurements_builder: MeasurementsBuilder::default(),
        }
    }

    fn build(self) -> Results<'input> {
        Results {
            input: self.input,
            warnings: self.warnings_builder.build(),
            measurements: self.measurements_builder.build(),
        }
    }
}

/// The results of applying rules and measures to a document.
#[derive(Debug, Default, Clone)]
pub struct Results<'input> {
    input: &'input str,
    warnings: Vec<Warning>,
    measurements: Vec<Measurement<'input>>,
}

impl<'input> Results<'input> {
    /// Iterate over the warnings.
    ///
    /// Warnings are ordered by their span in ascending order.
    pub fn iter_warnings(&self) -> impl Iterator<Item = &Warning> {
        self.warnings.iter()
    }

    /// Iterate over the warnings with their ranges.
    ///
    /// Warnings are ordered by their span in ascending order.
    pub fn iter_warnings_with_ranges(&self) -> impl Iterator<Item = (LineCharRange, &Warning)> {
        spans_to_ranges(self.input, self.warnings.iter())
    }

    /// Iterate over the measurements.
    ///
    /// Measurements are ordered by the word in ascending order, and then by the `MeasureKey`.
    pub fn iter_measurements(&self) -> impl Iterator<Item = &Measurement<'input>> {
        self.measurements.iter()
    }

    /// Iterate over the measurements with their ranges.
    ///
    /// Measurements are ordered by the word in ascending order, and then by the `MeasureKey`.
    pub fn iter_measurements_with_ranges(
        &self,
    ) -> impl Iterator<Item = (LineCharRange, &Measurement<'input>)> {
        spans_to_ranges(self.input, self.measurements.iter())
    }
}

#[cfg(test)]
impl quickcheck::Arbitrary for Results<'static> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let warnings = Vec::<(Word<'static>, String)>::arbitrary(g);
        let measurements = Vec::<(Word<'static>, MeasureKey)>::arbitrary(g);

        let mut builder = ResultsBuilder::new(crate::block::ARBITRARY_STR);

        for (word, message) in warnings {
            builder
                .warnings_builder
                .add_warning(WarningBuilder::new(&[word]).message(message).build());
        }

        for (word, key) in measurements {
            builder.measurements_builder.add_measurement(key, &word);
        }

        builder.build()
    }
}

/// A rule that finds warnings in a document.
pub trait Rule: Send + Sync {
    /// Apply the rule to the document, adding zero or more warnings to the builder.
    fn apply(&self, doc: &Document, warnings: &mut WarningsBuilder);
}

/// A rule that searches for warnings using a specific pattern, using a `Matcher`.
pub trait MatcherRule: Send {
    /// Get the matcher for this rule.
    fn matcher() -> impl Matcher;

    /// Handle a match for the given set of words. The results should be added
    /// to the builder.
    ///
    /// Further filtering can be done here. For example, if there is a
    /// constraint that can not be specified in a `Matcher`.
    fn on_match(words: &[Word], warnings: &mut WarningsBuilder);
}

impl<U: MatcherRule + Sync> Rule for U {
    /// Run the `matcher` on each block in the document, and call `on_match` for
    /// each match.
    fn apply(&self, doc: &Document, warnings: &mut WarningsBuilder) {
        let matcher = Self::matcher();
        for block in doc.iter() {
            match_words(block, &matcher, |words| {
                Self::on_match(words, warnings);
            });
        }
    }
}

/// A measure that searches for a specific pattern, using a `SingleWordPattern`.
pub trait Measure {
    /// Get the key for this measure.
    fn key(&self) -> MeasureKey;

    /// Get the pattern for this measure.
    fn pattern(&self) -> Box<dyn SingleWordPattern>;
}

struct MeasureInstance {
    key: MeasureKey,
    pattern: Box<dyn SingleWordPattern>,
}

impl MeasureInstance {
    fn apply<'input>(
        &self,
        doc: &Document<'input>,
        measurements: &mut MeasurementsBuilder<'input>,
    ) {
        for block in doc.iter() {
            for word in block.as_slice() {
                if self.pattern.matches_word(word) {
                    measurements.add_measurement(self.key, word);
                }
            }
        }
    }
}

/// A set of rules and measures to apply to a document.
pub struct RuleSet {
    rules: Vec<Box<dyn Rule>>,
    measures: Vec<MeasureInstance>,
}

impl RuleSet {
    /// Create a new rule set with the given rules and masures.
    pub fn new(rules: Vec<Box<dyn Rule>>, measures: Vec<Box<dyn Measure>>) -> Self {
        let measures = measures
            .into_iter()
            .map(|measure| MeasureInstance {
                key: measure.key(),
                pattern: measure.pattern(),
            })
            .collect();
        RuleSet { rules, measures }
    }

    /// Apply the rules and measures to the document, returning the results.
    pub fn apply<'input>(&self, doc: &Document<'input>) -> Results<'input> {
        let apply_span = debug_span!("RuleSet::apply");
        apply_span.in_scope(|| {
            let mut results = ResultsBuilder::new(doc.input());

            for rule in &self.rules {
                rule.apply(doc, &mut results.warnings_builder);
            }

            for measure in &self.measures {
                measure.apply(doc, &mut results.measurements_builder);
            }

            results.build()
        })
    }
}

impl Default for RuleSet {
    fn default() -> Self {
        let default_span = debug_span!("RuleSet::default");
        default_span.in_scope(|| RuleSet::new(default_rules(), default_measures()))
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{
        doc::Document,
        parsers::PlaintextParser,
        rule::{Measure, Results, Rule, RuleSet},
    };

    use super::WarningBuilder;

    pub(crate) fn rule_eq<R: Rule + 'static>(rule: R, input: &str, expected: usize) {
        let doc = Document::new(&PlaintextParser::default(), input);
        let ruleset = RuleSet::new(vec![Box::new(rule)], Vec::new());
        let results = ruleset.apply(&doc);
        assert_eq!(
            results.iter_warnings().count(),
            expected,
            "\n\ninput={:?}\n\ndoc={:#?}\n\nresults={:#?}",
            input,
            doc,
            results
        );
    }

    pub(crate) fn measure_eq<M: Measure + 'static>(measure: M, input: &str, expected: usize) {
        let doc = Document::new(&PlaintextParser::default(), input);
        let ruleset = RuleSet::new(Vec::new(), vec![Box::new(measure)]);
        let results = ruleset.apply(&doc);
        let measurements = results.iter_measurements().collect::<Vec<_>>();
        assert_eq!(
            measurements.len(),
            expected,
            "\n\ninput={:?}\n\ndoc={:#?}\n\nmeasurements={:#?}",
            input,
            doc,
            measurements
        );
    }

    #[should_panic]
    #[test]
    fn empty_warnings() {
        WarningBuilder::new(&[]).message("message".into()).build();
    }

    #[quickcheck]
    fn results_are_sorted(results: Results<'static>) -> bool {
        let warnings = results.iter_warnings().collect::<Vec<_>>();

        warnings.windows(2).all(|pair| pair[0].span <= pair[1].span)
    }

    #[quickcheck]
    fn measurements_are_sorted(results: Results<'static>) -> bool {
        let measurements = results.iter_measurements().collect::<Vec<_>>();

        measurements.windows(2).all(|pair| {
            if pair[0].word.as_span() == pair[1].word.as_span() {
                pair[0].key <= pair[1].key
            } else {
                pair[0] <= pair[1]
            }
        })
    }
}
