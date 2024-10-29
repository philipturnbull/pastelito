use pastelito_model::Tag;

use crate::{
    matcher::{OrS, SingleWordPattern},
    rule::{Measure, MeasureKey},
};

pub struct Adjectives;

impl Measure for Adjectives {
    fn key(&self) -> MeasureKey {
        MeasureKey::Adjectives
    }

    fn pattern(&self) -> Box<dyn SingleWordPattern> {
        Box::new(OrS(Tag::Adjective, Tag::Adverb))
    }
}
