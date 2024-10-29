use pastelito_model::Tag;

use crate::{
    matcher::{AndS, EndsWithIgnoreCase, OneOfS, OrS, SingleWordPattern},
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
            OneOfS([
                EndsWithIgnoreCase::new("able"),
                EndsWithIgnoreCase::new("ac"),
                EndsWithIgnoreCase::new("al"),
                EndsWithIgnoreCase::new("ant"),
                EndsWithIgnoreCase::new("ary"),
                EndsWithIgnoreCase::new("ent"),
                EndsWithIgnoreCase::new("ful"),
                EndsWithIgnoreCase::new("ible"),
                EndsWithIgnoreCase::new("ic"),
                EndsWithIgnoreCase::new("ive"),
                EndsWithIgnoreCase::new("less"),
                EndsWithIgnoreCase::new("ous"),
            ]),
        ))
    }
}
