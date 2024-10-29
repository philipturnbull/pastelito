use pastelito_model::POS;

use crate::{
    matcher::Matcher,
    rule::{MatcherRule, WarningBuilder, WarningsBuilder},
    Word,
};

pub struct WeakIng;

impl MatcherRule for WeakIng {
    fn matcher() -> impl Matcher {
        (POS::Modal, "be", POS::VerbGerundOrPresentParticiple)
    }

    fn on_match(words: &[Word], warnings: &mut WarningsBuilder) {
        warnings.add_warning(
            WarningBuilder::new(words)
                .message("Weak -ing".into())
                .build(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::rule_eq;

    use super::WeakIng;

    #[test]
    fn test() {
        rule_eq(WeakIng, "I will be describing.", 1);
        rule_eq(WeakIng, "I shall be attempting to describe.", 1);
        rule_eq(WeakIng, "I may be attempting to try.", 1);
    }
}
