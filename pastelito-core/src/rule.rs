use std::collections::HashMap;

use tracing::debug_span;

use crate::{
    block::Word,
    doc::Document,
    matcher::{match_words, Matcher, SingleWordPattern},
    measures::default_measures,
    rules::default_rules,
    span::ByteSpan,
};

/// This structure has a `ByteSpan`.
pub trait HasSpan {
    fn span(&self) -> ByteSpan;
}

/// A single finding from a rule which indicates a possible error in the
/// document.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Warning {
    /// The span of the warning.
    pub span: ByteSpan,
    /// The message associated with the warning.
    pub message: String,
}

impl HasSpan for Warning {
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeasureKey(&'static str);

impl From<&'static str> for MeasureKey {
    fn from(key: &'static str) -> Self {
        MeasureKey(key)
    }
}

impl From<MeasureKey> for String {
    fn from(val: MeasureKey) -> Self {
        val.0.to_owned()
    }
}

/// An instance of a measurement.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Measurement<'a> {
    /// The word that was measured.
    pub word: Word<'a>,
    /// The key for the measurement.
    pub key: MeasureKey,
}

impl<'a> Measurement<'a> {
    /// Create a new measurement.
    fn new(key: MeasureKey, word: Word<'a>) -> Self {
        Measurement { word, key }
    }
}

impl HasSpan for Measurement<'_> {
    fn span(&self) -> ByteSpan {
        self.word.as_span()
    }
}

/// Build a set of measurements for a single document.
#[derive(Debug, Default)]
pub struct MeasurementsBuilder<'a> {
    measurements: HashMap<MeasureKey, Vec<Word<'a>>>,
}

impl<'a> MeasurementsBuilder<'a> {
    /// Add a new measurement to the builder.
    pub fn add_measurement(&mut self, key: MeasureKey, word: &Word<'a>) {
        self.measurements.entry(key).or_default().push(*word);
    }

    fn build(self) -> Vec<Measurement<'a>> {
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
#[derive(Debug, Default)]
struct ResultsBuilder<'a> {
    warnings_builder: WarningsBuilder,
    measurements_builder: MeasurementsBuilder<'a>,
}

impl<'a> ResultsBuilder<'a> {
    /// Build the `Results`.
    fn build(self) -> Results<'a> {
        Results {
            warnings: self.warnings_builder.build(),
            measurements: self.measurements_builder.build(),
        }
    }
}

/// The results of applying rules and measures to a document.
#[derive(Debug, Default)]
pub struct Results<'a> {
    warnings: Vec<Warning>,
    measurements: Vec<Measurement<'a>>,
}

impl<'a> Results<'a> {
    /// Iterate over the warnings.
    ///
    /// Warnings are ordered by their span in ascending order.
    pub fn iter_warnings(&self) -> impl Iterator<Item = &Warning> {
        self.warnings.iter()
    }

    /// Iterate over the measurements.
    ///
    /// Measurements are ordered by the word in ascending order, and then by the `MeasureKey`.
    pub fn iter_measurements(&self) -> impl Iterator<Item = &Measurement> {
        self.measurements.iter()
    }

    /// Consume the results and iterate over the warnings and measurements.
    ///
    /// Warnings are ordered by their span in ascending order. Measurements are
    /// ordered by the word in ascending order, and then by the `MeasureKey`.
    pub fn into_iter_both(
        self,
    ) -> (
        impl Iterator<Item = Warning>,
        impl Iterator<Item = Measurement<'a>>,
    ) {
        (self.warnings.into_iter(), self.measurements.into_iter())
    }
}

/// A rule that finds warnings in a document.
pub trait Rule: Send {
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

impl<U: MatcherRule> Rule for U {
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
    fn apply<'a>(&self, doc: &Document<'a>, measurements: &mut MeasurementsBuilder<'a>) {
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
    pub fn apply<'a>(&self, doc: &Document<'a>) -> Results<'a> {
        let apply_span = debug_span!("RuleSet::apply");
        apply_span.in_scope(|| {
            let mut results = ResultsBuilder::default();

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
        RuleSet::new(default_rules(), default_measures())
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{
        doc::Document,
        parsers::PlaintextParser,
        rule::{Measure, Rule, RuleSet},
    };

    pub(crate) fn rule_eq<R: Rule + 'static>(rule: R, data: &str, expected: usize) {
        let doc = Document::new(&PlaintextParser::default(), data);
        let ruleset = RuleSet::new(vec![Box::new(rule)], Vec::new());
        let results = ruleset.apply(&doc);
        assert_eq!(
            results.iter_warnings().count(),
            expected,
            "\n\ndata={:?}\n\ndoc={:#?}\n\nresults={:#?}",
            data,
            doc,
            results
        );
    }

    pub(crate) fn measure_eq<M: Measure + 'static>(measure: M, data: &str, expected: usize) {
        let doc = Document::new(&PlaintextParser::default(), data);
        let ruleset = RuleSet::new(Vec::new(), vec![Box::new(measure)]);
        let results = ruleset.apply(&doc);
        let measurements = results.iter_measurements().collect::<Vec<_>>();
        assert_eq!(
            measurements.len(),
            expected,
            "\n\ndata={:?}\n\ndoc={:#?}\n\nmeasurements={:#?}",
            data,
            doc,
            measurements
        );
    }
}
