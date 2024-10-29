use pastelito_model::Tag;

use crate::{
    matcher::{OrS, SingleWordPattern},
    rule::{Measure, MeasureKey},
};

pub struct Prepositions;

impl Measure for Prepositions {
    fn key(&self) -> MeasureKey {
        MeasureKey::Prepositions
    }

    fn pattern(&self) -> Box<dyn SingleWordPattern> {
        Box::new(OrS(Tag::PrepositionOrSubordinatingConjunction, Tag::To))
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
