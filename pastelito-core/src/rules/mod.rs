use crate::rule::Rule;

mod academic_we;
mod passive_construction;
mod repeated_words;
mod weak_ing;
mod weasel_words;

pub(crate) fn default_rules() -> Vec<Box<dyn Rule + Send>> {
    vec![
        Box::new(academic_we::AcademicWe),
        Box::new(passive_construction::PassiveConstruction),
        Box::new(repeated_words::RepeatedWords),
        Box::new(weak_ing::WeakIng),
        Box::new(weasel_words::WeaselWords),
    ]
}
