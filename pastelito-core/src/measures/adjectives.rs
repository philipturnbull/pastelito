use pastelito_data::POS;

use crate::{
    matcher::{OrS, SingleWordPattern},
    rule::{Measure, MeasureKey},
};

pub struct Adjectives;

impl Measure for Adjectives {
    fn key(&self) -> MeasureKey {
        "adjectives".into()
    }

    fn pattern(&self) -> Box<dyn SingleWordPattern> {
        Box::new(OrS(POS::Adjective, POS::Adverb))
    }
}
