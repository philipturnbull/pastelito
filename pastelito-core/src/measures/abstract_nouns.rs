use pastelito_data::POS;

use crate::{
    matcher::{AndS, OneOfS, Regex, SingleWordPattern},
    rule::{MeasureKey, PatternMeasure},
};

pub struct AbstractNouns;

impl PatternMeasure for AbstractNouns {
    fn key() -> MeasureKey {
        "abstract-nouns".into()
    }

    fn pattern() -> impl SingleWordPattern {
        AndS(
            OneOfS([
                POS::NounPlural,
                POS::NounSingularOrMass,
                POS::ProperNounSingular,
                POS::ProperNounPlural,
            ]),
            Regex::new(r"(?i)\w(ance|ence|ences|ion|ions|ism|isms|ment|ty|ties)$"),
        )
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
