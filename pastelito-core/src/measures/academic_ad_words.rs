use pastelito_data::POS;

use crate::{
    matcher::{AndS, OrS, Regex, SingleWordPattern},
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
            Regex::new(r"(?i)\w(able|ac|al|ant|ary|ent|ful|ible|ic|ive|less|ous)$"),
        )
    }
}
