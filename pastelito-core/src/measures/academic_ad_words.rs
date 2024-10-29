use pastelito_model::POS;

use crate::{
    matcher::{AndS, OrS, Regex, SingleWordPattern},
    rule::{Measure, MeasureKey},
};

pub struct AcademicAdWords;

impl Measure for AcademicAdWords {
    fn key(&self) -> MeasureKey {
        "academic-ad-words".into()
    }

    fn pattern(&self) -> Box<dyn SingleWordPattern> {
        Box::new(AndS(
            OrS(POS::Adjective, POS::Adverb),
            Regex::new(r"(?i)\w(able|ac|al|ant|ary|ent|ful|ible|ic|ive|less|ous)$"),
        ))
    }
}
