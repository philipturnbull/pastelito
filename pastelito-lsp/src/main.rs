#[allow(clippy::needless_return)]
mod service;

use service::Service;
use tower_lsp::{LspService, Server};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new("/tmp/lsp-log");
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(std::fs::File::create(path)?)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Service::new);
    let _ = Server::new(stdin, stdout, socket).serve(service).await;

    Ok(())
}
