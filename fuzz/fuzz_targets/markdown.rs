#![no_main]

mod helpers;
use helpers::filter_panics;
use libfuzzer_sys::{fuzz_target, Corpus};
use pastelito_core::{parsers::MarkdownParser, Document};

fn do_fuzz(data: &[u8]) -> Corpus {
    if let Ok(markdown) = std::str::from_utf8(data) {
        let _doc = Document::new(&MarkdownParser::default(), markdown);

        Corpus::Keep
    } else {
        Corpus::Reject
    }
}

fuzz_target!(|data: &[u8]| -> Corpus { filter_panics(|| do_fuzz(data)) });
