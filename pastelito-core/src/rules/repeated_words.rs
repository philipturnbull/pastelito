use pastelito_data::POS;

use crate::{
    matcher::{Matcher, OneOf, Or},
    rule::{MatcherRule, WarningBuilder, WarningsBuilder},
    Word,
};

pub struct RepeatedWords;

impl MatcherRule for RepeatedWords {
    fn matcher() -> impl Matcher {
        Or(
            OneOf([
                (POS::Determiner, POS::Determiner),
                (
                    POS::VerbNon3rdPersonSingularPresent,
                    POS::VerbNon3rdPersonSingularPresent,
                ),
                (POS::Modal, POS::Modal),
                (
                    POS::PrepositionOrSubordinatingConjunction,
                    POS::PrepositionOrSubordinatingConjunction,
                ),
                (POS::PersonalPronoun, POS::PersonalPronoun),
                (POS::PossesivePronoun, POS::PossesivePronoun),
                (POS::To, POS::To),
            ]),
            OneOf([("be", "be"), ("is", "is"), ("are", "are")]),
        )
    }

    fn on_match(words: &[Word], warnings: &mut WarningsBuilder) {
        warnings.add_warning(
            WarningBuilder::new(words)
                .message("Repeated words".into())
                .build(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::rule_eq;

    use super::RepeatedWords;

    #[test]
    fn test() {
        // Determiner
        rule_eq(RepeatedWords, "An the apple.", 1);
        // VerbNon3rdPersonSingularPresent
        rule_eq(RepeatedWords, "I am am describing.", 1);
        // Modal
        rule_eq(RepeatedWords, "I will will be describing.", 1);
        // PrepositionOrSubordinatingConjunction
        rule_eq(RepeatedWords, "It is on on the table.", 1);
        // PersonalPronoun
        rule_eq(RepeatedWords, "He she is here.", 1);
        // PossesivePronoun
        rule_eq(RepeatedWords, "His her cat is here.", 1);
        // To
        rule_eq(RepeatedWords, "We are going to to the store.", 1);
        // be
        rule_eq(RepeatedWords, "I will be be there.", 1);
        // is
        rule_eq(RepeatedWords, "It is is a cat.", 1);
        // are
        rule_eq(RepeatedWords, "They are are here.", 1);
    }
}
