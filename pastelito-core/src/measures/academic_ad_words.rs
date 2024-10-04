use pastelito_data::POS;

use crate::{
    matcher::{AndS, OrS, SingleWordPattern, StrFn},
    rule::{MeasureKey, PatternMeasure},
};

pub struct AcademicAdWords;

impl PatternMeasure for AcademicAdWords {
    fn key() -> MeasureKey {
        "academic-ad-words".into()
    }

    fn pattern() -> impl SingleWordPattern {
        AndS(
            OrS(POS::Adjective, POS::Adverb),
            StrFn(|word| {
                let suffixes = [
                    "able", "ac", "al", "ant", "ary", "ent", "ful", "ible", "ic", "ive", "less",
                    "ous",
                ];

                suffixes.iter().any(|suffix| word.ends_with(suffix))
            }),
        )
    }
}
