#![no_main]

mod helpers;
use helpers::{filter_panics, RULESET};
use libfuzzer_sys::{fuzz_target, Corpus};
use pastelito_core::{parsers::MarkdownParser, Document};

fn do_fuzz(data: &[u8]) -> Corpus {
    if let Ok(markdown) = std::str::from_utf8(data) {
        let doc = Document::new(&MarkdownParser::default(), markdown);

        RULESET.with(|ruleset| {
            let results = ruleset.apply(&doc);

            let warnings = results.iter_measurements_with_ranges();
            let _num_warnings = warnings.map(|(_range, _warning)| {}).count();

            let measurements = results.iter_measurements_with_ranges();
            let _num_measurements = measurements.map(|(_range, _measurement)| {}).count();
        });

        Corpus::Keep
    } else {
        Corpus::Reject
    }
}

fuzz_target!(|data: &[u8]| -> Corpus { filter_panics(|| do_fuzz(data)) });
