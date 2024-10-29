// Ignore warning in the generated `pastelito.rs` file
#![allow(static_mut_refs)]

mod pastelito;

use std::sync::OnceLock;

use crate::pastelito::Guest;
use pastelito::vscode::pastelito::types::{Measurement, Range, Results, Warning};
use pastelito_core::{
    parsers::MarkdownParser,
    rule::{MeasureKey, RuleSet},
    Document, LineCharRange,
};

static KNOWN_MEASURE_KEYS: OnceLock<[(&'static str, u32); 5]> = OnceLock::new();
static DEFAULT_RULESET: OnceLock<RuleSet> = OnceLock::new();

fn to_range(range: LineCharRange) -> Range {
    Range {
        start_line: range.start_line,
        start_char: range.start_char,
        end_line: range.end_line,
        end_char: range.end_char,
    }
}

fn to_key(known_measure_keys: &[(&'static str, u32)], key: MeasureKey) -> u32 {
    known_measure_keys
        .iter()
        .find(|(k, _)| <MeasureKey as From<&str>>::from(*k) == key)
        .map(|(_, v)| *v)
        .unwrap_or(0)
}

fn rule_results_to_results(results: pastelito_core::rule::Results) -> Results {
    let known_measure_keys = KNOWN_MEASURE_KEYS.get_or_init(|| {
        [
            ("abstract-nouns", 0),
            ("academic-ad-words", 1),
            ("adjectives", 2),
            ("be-verbs", 3),
            ("prepositions", 4),
        ]
    });

    let warnings = results
        .iter_warnings_with_ranges()
        .map(|(range, warning)| Warning {
            range: to_range(range),
            message: warning.message.to_owned(),
        })
        .collect::<Vec<_>>();

    let measurements = results
        .iter_measurements_with_ranges()
        .map(|(range, measurement)| Measurement {
            range: to_range(range),
            key: to_key(known_measure_keys, measurement.key),
        })
        .collect::<Vec<_>>();

    Results {
        warnings,
        measurements,
    }
}

struct Implementation;
impl Guest for Implementation {
    fn apply_default_rules(input: String) -> Results {
        let doc = Document::new(&MarkdownParser::default(), &input);

        let ruleset = DEFAULT_RULESET.get_or_init(RuleSet::default);

        let results = ruleset.apply(&doc);
        rule_results_to_results(results)
    }
}

pastelito::export!(Implementation with_types_in pastelito);
