use pastelito_model::POS;

use crate::{
    matcher::{Matcher, OneOfS, Regex},
    rule::{MatcherRule, WarningBuilder, WarningsBuilder},
    Word,
};

pub struct AcademicWe;

impl MatcherRule for AcademicWe {
    fn matcher() -> impl Matcher {
        (
            Regex::new(r"(?i)^we$"),
            POS::Modal,
            OneOfS([
                POS::VerbBaseForm,
                POS::VerbPastTense,
                POS::VerbGerundOrPresentParticiple,
                POS::VerbPastParticiple,
                POS::VerbNon3rdPersonSingularPresent,
                POS::Verb3rdPersonSingularPresent,
            ]),
        )
    }

    fn on_match(words: &[Word], warnings: &mut WarningsBuilder) {
        warnings.add_warning(
            WarningBuilder::new(words)
                .message("Academic we".into())
                .build(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::rule::test::rule_eq;

    use super::AcademicWe;

    #[test]
    fn test() {
        rule_eq(AcademicWe, "In this paper, we will show a novel blah.", 1);
        rule_eq(AcademicWe, "The knowledge that we gained while working.", 0);
    }
}
