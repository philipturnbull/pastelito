use pastelito_model::Tag;

use crate::{
    matcher::{AndS, OrS, Regex, SingleWordPattern},
    rule::{Measure, MeasureKey},
};

pub struct AcademicAdWords;

impl Measure for AcademicAdWords {
    fn key(&self) -> MeasureKey {
        MeasureKey::AcademicAdWords
    }

    fn pattern(&self) -> Box<dyn SingleWordPattern> {
        Box::new(AndS(
            OrS(Tag::Adjective, Tag::Adverb),
            Regex::new(r"(?i)\w(able|ac|al|ant|ary|ent|ful|ible|ic|ive|less|ous)$"),
        ))
    }
}
