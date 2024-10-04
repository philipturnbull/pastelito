use pastelito_data::POS;

use crate::{
    matcher::{Matcher, Opt, Or},
    rule::{MatcherRule, WarningBuilder, WarningsBuilder},
    Word,
};

pub struct PassiveConstruction;

impl MatcherRule for PassiveConstruction {
    fn matcher() -> impl Matcher {
        (Or("was", "were"), Opt(POS::Adverb), POS::VerbPastParticiple)
    }

    fn on_match(words: &[Word], warnings: &mut WarningsBuilder) {
        warnings.add_warning(
            WarningBuilder::new(words)
                .message("Passive construction".into())
                .build(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::rule_eq;

    use super::PassiveConstruction;

    #[test]
    fn test() {
        rule_eq(PassiveConstruction, "The item was broken.", 1);
        rule_eq(PassiveConstruction, "The item was not broken.", 1);

        rule_eq(PassiveConstruction, "Mistakes were made.", 1);
        rule_eq(PassiveConstruction, "Mistakes were not made.", 1);

        rule_eq(PassiveConstruction, "They were asked to leave.", 1);
    }
}
