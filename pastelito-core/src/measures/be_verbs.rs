use crate::{
    matcher::{IgnoreCase, OneOfS, SingleWordPattern},
    rule::{Measure, MeasureKey},
};

pub struct BeVerbs;

impl Measure for BeVerbs {
    fn key(&self) -> MeasureKey {
        MeasureKey::BeVerbs
    }

    fn pattern(&self) -> Box<dyn SingleWordPattern> {
        Box::new(OneOfS([
            IgnoreCase::new("am"),
            IgnoreCase::new("are"),
            IgnoreCase::new("be"),
            IgnoreCase::new("been"),
            IgnoreCase::new("being"),
            IgnoreCase::new("is"),
            IgnoreCase::new("was"),
            IgnoreCase::new("were"),
        ]))
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::measure_eq;

    use super::BeVerbs;

    #[test]
    fn test() {
        measure_eq(BeVerbs, "I am a person.", 1);
        measure_eq(BeVerbs, "You are a person.", 1);
        measure_eq(BeVerbs, "He is a person.", 1);
        measure_eq(BeVerbs, "Is this true?", 1);
    }
}
