use crate::{
    matcher::{Regex, SingleWordPattern},
    rule::{MeasureKey, PatternMeasure},
};

pub struct BeVerbs;

impl PatternMeasure for BeVerbs {
    fn key() -> MeasureKey {
        "be-verbs".into()
    }

    fn pattern() -> impl SingleWordPattern {
        Regex::new(r"(?i)^(am|are|be|been|being|is|was|were)$")
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
