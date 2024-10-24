#![no_main]

mod helpers;
use helpers::{filter_panics, RULESET};
use libfuzzer_sys::{fuzz_target, Corpus};
use pastelito_core::{
    lines::{spans_to_ranges, LineCharRange},
    parsers::MarkdownParser,
    Document,
};

struct FuzzRange {}

impl LineCharRange for FuzzRange {
    fn new(_start_line: u32, _start_char: u32, _end_line: u32, _end_char: u32) -> Self {
        Self {}
    }
}

fn do_fuzz(data: &[u8]) -> Corpus {
    if let Ok(markdown) = std::str::from_utf8(data) {
        let doc = Document::new(&MarkdownParser::default(), markdown);

        RULESET.with(|ruleset| {
            let results = ruleset.apply(&doc);
            let (warnings, measurements) = results.into_iter_both();

            let warnings = spans_to_ranges(markdown, warnings);
            let _num_warnings = warnings
                .map(|(_range, _warning): (FuzzRange, _)| {})
                .count();

            let measurements = spans_to_ranges(markdown, measurements);
            let _num_measurements = measurements
                .map(|(_range, _measurement): (FuzzRange, _)| {})
                .count();
        });

        Corpus::Keep
    } else {
        Corpus::Reject
    }
}

fuzz_target!(|data: &[u8]| -> Corpus { filter_panics(|| do_fuzz(data)) });
