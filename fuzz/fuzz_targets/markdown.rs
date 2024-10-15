#![no_main]

use lazy_static::lazy_static;
use libfuzzer_sys::{fuzz_target, Corpus};
use pastelito_core::{parsers::MarkdownParser, rule::RuleSet, Document};
use std::{
    panic,
    sync::atomic::{AtomicBool, Ordering},
};

fn filter_panic(panic_info: &panic::PanicHookInfo<'_>) {
    if let Some(location) = panic_info.location() {
        if location.file().contains("/pulldown-cmark") {
            DID_PANIC_IN_PULLDOWN_CMARK.store(true, Ordering::Relaxed);
        } else {
            eprintln!("{}", panic_info);
        }
    }
}

lazy_static! {
    static ref ALL_PANICS: bool = {
        panic::set_hook(Box::new(filter_panic));
        std::env::var("ALL_PANICS").is_ok()
    };
    static ref DID_PANIC_IN_PULLDOWN_CMARK: AtomicBool = AtomicBool::new(false);
}

fn do_fuzz(data: &[u8]) -> Corpus {
    if let Ok(markdown) = std::str::from_utf8(data) {
        let doc = Document::new(&MarkdownParser::default(), markdown);
        let ruleset = RuleSet::default();

        let results = ruleset.apply(&doc);
        let (warnings, measurements) = results.into_iter_both();

        for _warning in warnings {
            // consume iterator
        }
        for _measurement in measurements {
            // consume iterator
        }
        Corpus::Keep
    } else {
        Corpus::Reject
    }
}

fuzz_target!(|data: &[u8]| -> Corpus {
    if *ALL_PANICS {
        let err = panic::catch_unwind(|| do_fuzz(data));

        let did_panic_in_pulldown_cmark =
            DID_PANIC_IN_PULLDOWN_CMARK.swap(false, Ordering::Relaxed);

        match err {
            Ok(corpus) => corpus,
            Err(payload) => {
                if did_panic_in_pulldown_cmark {
                    // Ignore any testcases that panics in pulldown-cmark
                    Corpus::Reject
                } else {
                    // Otherwide, bubble up the panic to libfuzzer
                    panic::resume_unwind(payload);
                }
            }
        }
    } else {
        do_fuzz(data)
    }
});
