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
            let (warnings, measurements) = results.into_iter_both();

            for _warning in warnings {
                // consume iterator
            }
            for _measurement in measurements {
                // consume iterator
            }
        });

        Corpus::Keep
    } else {
        Corpus::Reject
    }
}

fuzz_target!(|data: &[u8]| -> Corpus { filter_panics(|| do_fuzz(data)) });
