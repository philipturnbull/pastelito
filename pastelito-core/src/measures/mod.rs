use crate::rule::Measure;

mod abstract_nouns;
mod academic_ad_words;
mod adjectives;
mod be_verbs;
mod prepositions;

pub(crate) fn default_measures() -> Vec<Box<dyn Measure>> {
    vec![
        Box::new(abstract_nouns::AbstractNouns),
        Box::new(academic_ad_words::AcademicAdWords),
        Box::new(adjectives::Adjectives),
        Box::new(be_verbs::BeVerbs),
        Box::new(prepositions::Prepositions),
    ]
}
