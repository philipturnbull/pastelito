use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pastelito_core::{
    doc::{Document, Parser},
    parsers::{MarkdownParser, PlaintextParser},
    rule::RuleSet,
    Tagger,
};

const BLOG_POST: &str = include_str!("data/leaving-rust-gamedev.md");

fn count_words(doc: &Document) -> usize {
    doc.iter().map(|block| block.as_slice().len()).sum()
}

fn benchmark_tag(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tagger");

    let doc = Document::new(&MarkdownParser::default(), BLOG_POST);
    let num_words = count_words(&doc);

    let tagger = Tagger::default();

    group.throughput(Throughput::Elements(num_words as u64));
    group.bench_function("tag", |b| {
        b.iter_batched_ref(
            || {
                let mut doc = doc.clone();
                for block in doc.iter_mut() {
                    for word in block.iter_mut() {
                        word.pos = None;
                    }
                }
                doc
            },
            |doc| {
                for block in doc.iter_mut() {
                    tagger.tag(block);
                }
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn benchmark_parse<P: Parser + Default>(c: &mut Criterion, group_name: &str) {
    let mut group = c.benchmark_group(group_name);

    let doc = Document::new(&P::default(), BLOG_POST);
    let num_words = count_words(&doc);

    group.throughput(Throughput::Elements(num_words as u64));
    group.bench_function("Doc::new", |b| {
        b.iter(|| {
            let doc = Document::new(&P::default(), BLOG_POST);
            black_box(doc);
        })
    });
}

fn benchmark_parse_markdown(c: &mut Criterion) {
    benchmark_parse::<MarkdownParser>(c, "MarkdownParser");
}

fn benchmark_parse_plaintext(c: &mut Criterion) {
    benchmark_parse::<PlaintextParser>(c, "PlaintextParser");
}

fn benchmark_default_ruleset(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rules/RuleSet::default()");

    let doc = Document::new(&MarkdownParser::default(), BLOG_POST);
    let num_words = count_words(&doc);

    let ruleset = RuleSet::default();

    group.throughput(Throughput::Elements(num_words as u64));
    group.bench_function("apply", |b| {
        b.iter_batched_ref(
            || doc.clone(),
            |doc| {
                let results = ruleset.apply(doc);
                black_box(results);
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    benchmark_parse_markdown,
    benchmark_parse_plaintext,
    benchmark_tag,
    benchmark_default_ruleset
);
criterion_main!(benches);
