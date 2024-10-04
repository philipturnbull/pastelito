use pastelito_data::POS;

use crate::{
    matcher::{OrS, SingleWordPattern},
    rule::{MeasureKey, PatternMeasure},
};

pub struct Adjectives;

impl PatternMeasure for Adjectives {
    fn key() -> MeasureKey {
        "adjectives".into()
    }

    fn pattern() -> impl SingleWordPattern {
        OrS(POS::Adjective, POS::Adverb)
    }
}
