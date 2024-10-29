use pastelito_model::Tag;

use crate::{
    matcher::{AndS, EndsWithIgnoreCase, OneOfS, SingleWordPattern},
    rule::{Measure, MeasureKey},
};

pub struct AbstractNouns;

impl Measure for AbstractNouns {
    fn key(&self) -> MeasureKey {
        MeasureKey::AbstractNouns
    }

    fn pattern(&self) -> Box<dyn SingleWordPattern> {
        Box::new(AndS(
            OneOfS([
                Tag::NounPlural,
                Tag::NounSingularOrMass,
                Tag::ProperNounSingular,
                Tag::ProperNounPlural,
            ]),
            OneOfS([
                EndsWithIgnoreCase::new("ance"),
                EndsWithIgnoreCase::new("ence"),
                EndsWithIgnoreCase::new("ences"),
                EndsWithIgnoreCase::new("ion"),
                EndsWithIgnoreCase::new("ions"),
                EndsWithIgnoreCase::new("ism"),
                EndsWithIgnoreCase::new("isms"),
                EndsWithIgnoreCase::new("ment"),
                EndsWithIgnoreCase::new("ty"),
                EndsWithIgnoreCase::new("ties"),
            ]),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::measure_eq;

    use super::AbstractNouns;

    #[test]
    fn test() {
        measure_eq(AbstractNouns, "Your participation is important.", 1);
        measure_eq(AbstractNouns, "Your criticism is important.", 1);
        measure_eq(AbstractNouns, "Your involvement is important.", 1);
        measure_eq(AbstractNouns, "Your activity is important.", 1);
        measure_eq(AbstractNouns, "Your performance is important.", 1);
        measure_eq(AbstractNouns, "Your presence is important.", 1);
        measure_eq(AbstractNouns, "The identities were hidden.", 1);
        measure_eq(AbstractNouns, "The ties were hidden.", 0);
        measure_eq(AbstractNouns, "Criticism is important.", 1);
    }
}
