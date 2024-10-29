use pastelito_core::parsers::MarkdownParser;
use pastelito_core::rule::RuleSet;
use pastelito_core::Document;
use std::io::Read;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

const HELP: &str = r#"Example CLI for pastelito-core

Usage: cli [OPTIONS] [FILENAME]

Arguments:
  [FILENAME] Input filename. If not provided, read from stdin

Options:
      --debug  Enable tracing debug output
      --quiet  Do not print results
  -h, --help   Print help"#;

#[derive(Default)]
struct Args {
    debug: bool,
    quiet: bool,
    filename: Option<std::path::PathBuf>,
}

fn parse_args() -> Args {
    let mut args = Args::default();

    for arg in std::env::args() {
        match arg.as_str() {
            "--debug" => args.debug = true,
            "--quiet" => args.quiet = true,
            "-h" | "--help" => {
                println!("{}", HELP);
                std::process::exit(0);
            }
            _ => {
                if arg.starts_with('-') {
                    eprintln!("Unknown option: {}", arg);
                    std::process::exit(1);
                } else {
                    args.filename = Some(std::path::PathBuf::from(arg));
                }
            }
        }
    }

    args
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();

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
