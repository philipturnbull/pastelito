use pastelito_data::POS;

use crate::{
    matcher::{OrS, SingleWordPattern},
    rule::{MeasureKey, PatternMeasure},
};

pub struct Prepositions;

impl PatternMeasure for Prepositions {
    fn key() -> MeasureKey {
        "prepositions".into()
    }

    fn pattern() -> impl SingleWordPattern {
        OrS(POS::PrepositionOrSubordinatingConjunction, POS::To)
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::measure_eq;

    use super::Prepositions;

    #[test]
    fn test() {
        measure_eq(Prepositions, "On the table.", 1);
        measure_eq(Prepositions, "On the table by the bed.", 2);
        measure_eq(Prepositions, "I am going to the store.", 1);
    }
}
