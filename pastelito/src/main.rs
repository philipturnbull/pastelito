use clap::Parser;
use pastelito_core::parsers::MarkdownParser;
use pastelito_core::rule::RuleSet;
use pastelito_core::Document;
use std::io::Read;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    debug: bool,

    #[clap(long)]
    quiet: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.debug {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::TRACE)
            .with_span_events(FmtSpan::CLOSE)
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;
    }

    let mut input = String::new();
    let _ = std::io::stdin().read_to_string(&mut input);

    let ruleset = RuleSet::default();

    let doc = Document::new(&MarkdownParser::default(), input.as_str());

    let results = ruleset.apply(&doc);
    if !args.quiet {
        println!("{:#?}", results);
    }

    Ok(())
}
