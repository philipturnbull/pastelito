use pastelito_model::Tag;

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
                (Tag::Determiner, Tag::Determiner),
                (
                    Tag::VerbNon3rdPersonSingularPresent,
                    Tag::VerbNon3rdPersonSingularPresent,
                ),
                (Tag::Modal, Tag::Modal),
                (
                    Tag::PrepositionOrSubordinatingConjunction,
                    Tag::PrepositionOrSubordinatingConjunction,
                ),
                (Tag::PersonalPronoun, Tag::PersonalPronoun),
                (Tag::PossesivePronoun, Tag::PossesivePronoun),
                (Tag::To, Tag::To),
            ]),
            OneOf([("be", "be"), ("is", "is"), ("are", "are")]),
        )
    }

    fn on_match(words: &[Word], warnings: &mut WarningsBuilder) {
        assert_eq!(
            words.len(),
            2,
            "Internal error: expected 2 words in RepeatedWords"
        );

        let word0 = words[0];
        let word1 = words[1];

        if word0.as_str().eq_ignore_ascii_case(word1.as_str()) {
            warnings.add_warning(
                WarningBuilder::new(words)
                    .message("Repeated words".into())
                    .build(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::rule_eq;

    use super::RepeatedWords;

    #[test]
    fn test() {
        // Determiner
        rule_eq(RepeatedWords, "An an apple.", 1);
        // VerbNon3rdPersonSingularPresent
        rule_eq(RepeatedWords, "I am am describing.", 1);
        // Modal
        rule_eq(RepeatedWords, "I will will be describing.", 1);
        // PrepositionOrSubordinatingConjunction
        rule_eq(RepeatedWords, "It is on on the table.", 1);
        // PersonalPronoun
        rule_eq(RepeatedWords, "He he is here.", 1);
        // PossesivePronoun
        rule_eq(RepeatedWords, "His his cat is here.", 1);
        // To
        rule_eq(RepeatedWords, "We are going to to the store.", 1);
        // be
        rule_eq(RepeatedWords, "I will be be there.", 1);
        // is
        rule_eq(RepeatedWords, "It is is a cat.", 1);
        // are
        rule_eq(RepeatedWords, "They are are here.", 1);

        // Prepositions are allowed to repeat if they are different
        rule_eq(RepeatedWords, "Because because it was difficult.", 1);
    }
}
