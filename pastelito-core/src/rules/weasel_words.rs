use pastelito_model::POS;

use crate::{
    matcher::{AndS, Matcher, Regex},
    rule::{MatcherRule, WarningBuilder, WarningsBuilder},
    Word,
};

pub struct WeaselWords;

impl MatcherRule for WeaselWords {
    fn matcher() -> impl Matcher {
        AndS(
            POS::Adverb,
            Regex::new("(?i)^(absolutely|actually|basically|certainly|completely|definitely|easily|just|literally|probably|quite|rather|really|somehow|suddenly|totally|virtually)$"),
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
