use std::{cell::RefCell, panic};

use libfuzzer_sys::Corpus;
use pastelito_core::rule::RuleSet;

thread_local! {
    static REPORT_ALL_PANICS: bool = {
        let report_all_panics = std::env::var("REPORT_ALL_PANICS").is_ok();
        if !report_all_panics {
            panic::set_hook(Box::new(filter_panic));
        }
        report_all_panics
    };
    static DID_PANIC_IN_PULLDOWN_CMARK: RefCell<bool> = const { RefCell::new(false) };
    pub static RULESET: RuleSet = RuleSet::default();
}

fn filter_panic(panic_info: &panic::PanicHookInfo<'_>) {
    if let Some(location) = panic_info.location() {
        if location.file().contains("/pulldown-cmark") {
            DID_PANIC_IN_PULLDOWN_CMARK.replace(true);
        } else {
            eprintln!("{}", panic_info);
        }
    }
}

pub fn filter_panics<F>(cb: F) -> Corpus
where
    F: FnOnce() -> Corpus + panic::UnwindSafe,
{
    let report_all_panics = REPORT_ALL_PANICS.with(|all_panics| *all_panics);

    if report_all_panics {
        cb()
    } else {
        let err = panic::catch_unwind(cb);

        let did_panic_in_pulldown_cmark = DID_PANIC_IN_PULLDOWN_CMARK.take();

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
    }
}
