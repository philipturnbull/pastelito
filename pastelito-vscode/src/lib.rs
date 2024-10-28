mod pastelito;

use std::sync::OnceLock;

use crate::pastelito::Guest;
use pastelito::vscode::pastelito::types::{Measurement, Range, Results, Warning};
//use pastelito::vscode::pastelito::types::Results;
use pastelito_core::{parsers::MarkdownParser, rule::RuleSet, Document, LineCharRange};

static DEFAULT_RULESET: OnceLock<RuleSet> = OnceLock::new();

fn to_range(range: LineCharRange) -> Range {
    Range {
        start_line: range.start_line,
        start_char: range.start_char,
        end_line: range.end_line,
        end_char: range.end_char,
    }
}

fn rule_results_to_results(results: pastelito_core::rule::Results) -> Results {
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
            key: measurement.key.into(),
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
