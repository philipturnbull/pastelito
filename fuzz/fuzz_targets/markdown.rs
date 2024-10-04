#![no_main]

use lazy_static::lazy_static;
use libfuzzer_sys::fuzz_target;
use pastelito_core::{parsers::MarkdownParser, rule::RuleSet, Document};

lazy_static! {
    static ref APPLY_RULES: bool = std::env::var("APPLY_RULES").is_ok();
}

fuzz_target!(|data: &[u8]| {
    if let Ok(markdown) = std::str::from_utf8(data) {
        let doc = Document::new(&MarkdownParser::default(), markdown);
        if *APPLY_RULES {
            let ruleset = RuleSet::default();
            let results = ruleset.apply(&doc);
            let (_warnings, _measurements) = results.into_iter_both();
        }
    }
});
