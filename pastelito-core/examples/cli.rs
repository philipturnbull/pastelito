use clap::Parser;
use pastelito_core::parsers::MarkdownParser;
use pastelito_core::rule::RuleSet;
use pastelito_core::Document;
use std::io::Read;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

/// Example CLI for pastelito-core
#[derive(Parser)]
struct Args {
    /// Enable tracing debug output
    #[clap(long)]
    debug: bool,

    /// Do not print results
    #[clap(long)]
    quiet: bool,

    /// Input filename. If not provided, read from stdin
    filename: Option<std::path::PathBuf>,
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

    let input = match args.filename {
        Some(filename) => std::fs::read_to_string(filename)?,
        None => {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input)?;
            input
        }
    };

    let ruleset = RuleSet::default();

    let doc = Document::new(&MarkdownParser::default(), input.as_str());

    let results = ruleset.apply(&doc);
    if !args.quiet {
        println!("{:#?}", results);
    }

    Ok(())
}
