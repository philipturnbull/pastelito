use pastelito_model::Tag;

use crate::{
    matcher::{AndS, IgnoreCase, Matcher, OneOfS},
    rule::{MatcherRule, WarningBuilder, WarningsBuilder},
    Word,
};

pub struct WeaselWords;

impl MatcherRule for WeaselWords {
    fn matcher() -> impl Matcher {
        AndS(
            Tag::Adverb,
            OneOfS([
                IgnoreCase::new("absolutely"),
                IgnoreCase::new("actually"),
                IgnoreCase::new("basically"),
                IgnoreCase::new("certainly"),
                IgnoreCase::new("completely"),
                IgnoreCase::new("definitely"),
                IgnoreCase::new("easily"),
                IgnoreCase::new("just"),
                IgnoreCase::new("literally"),
                IgnoreCase::new("probably"),
                IgnoreCase::new("quite"),
                IgnoreCase::new("rather"),
                IgnoreCase::new("really"),
                IgnoreCase::new("somehow"),
                IgnoreCase::new("suddenly"),
                IgnoreCase::new("totally"),
                IgnoreCase::new("virtually"),
            ]),
        )
    }

    fn on_match(words: &[Word], warnings: &mut WarningsBuilder) {
        warnings.add_warning(
            WarningBuilder::new(words)
                .message("Weasel words".into())
                .build(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::rule_eq;

    use super::WeaselWords;

    #[test]
    fn test() {
        rule_eq(WeaselWords, "It was quite complex.", 1);
        rule_eq(WeaselWords, "Basically, it was blah.", 1);
    }
}
