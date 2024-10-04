use pastelito_data::POS;

use crate::{
    matcher::{AndS, Lowercase, Matcher, OneOfS},
    rule::{MatcherRule, WarningBuilder, WarningsBuilder},
    Word,
};

pub struct WeaselWords;

impl MatcherRule for WeaselWords {
    fn matcher() -> impl Matcher {
        AndS(
            POS::Adverb,
            OneOfS([
                Lowercase("absolutely"),
                Lowercase("actually"),
                Lowercase("basically"),
                Lowercase("certainly"),
                Lowercase("completely"),
                Lowercase("definitely"),
                Lowercase("easily"),
                Lowercase("just"),
                Lowercase("literally"),
                Lowercase("probably"),
                Lowercase("quite"),
                Lowercase("rather"),
                Lowercase("really"),
                Lowercase("somehow"),
                Lowercase("suddenly"),
                Lowercase("totally"),
                Lowercase("virtually"),
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
    }
}
