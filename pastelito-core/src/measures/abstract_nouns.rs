use pastelito_data::POS;

use crate::{
    matcher::{AndS, OrS, SingleWordPattern, StrFn},
    rule::{MeasureKey, PatternMeasure},
};

pub struct AbstractNouns;

impl PatternMeasure for AbstractNouns {
    fn key() -> MeasureKey {
        "abstract-nouns".into()
    }

    fn pattern() -> impl SingleWordPattern {
        AndS(
            OrS(POS::NounPlural, POS::NounSingularOrMass),
            StrFn(|word| {
                word.ends_with("ance")
                    || word.ends_with("ence")
                    || word.ends_with("ences")
                    || (word.len() > 3 && word.ends_with("ion"))
                    || (word.len() > 4 && word.ends_with("ions"))
                    || word.ends_with("ism")
                    || word.ends_with("isms")
                    || word.ends_with("ment")
                    || word.ends_with("ments")
                    || word.ends_with("ty")
                    || (word.len() > 4 && word.ends_with("ties"))
            }),
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
    }
}
