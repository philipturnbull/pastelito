use pastelito_core::lines::spans_to_ranges;
use pastelito_core::lines::LineCharRange;
use pastelito_core::parsers::MarkdownParser;
use pastelito_core::rule::Results;
use pastelito_core::rule::RuleSet;
use pastelito_core::Document;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::notification::PublishDiagnostics;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    NumberOrString, Position, PublishDiagnosticsParams, Range, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions, Url,
};
use tower_lsp::{
    lsp_types::{InitializeParams, InitializeResult},
    Client, LanguageServer,
};
use tracing::{debug_span, event, Level};

pub struct Service {
    client: Client,
    ruleset: RuleSet,
}

fn to_vscode_range(range: LineCharRange) -> Range {
    Range {
        start: Position::new(range.start_line, range.start_char),
        end: Position::new(range.end_line, range.end_char),
    }
}

fn rule_results_to_diagnostics(text: &str, results: Results) -> Vec<Diagnostic> {
    let mut diagnostics =
        Vec::with_capacity(results.iter_measurements().count() + results.iter_warnings().count());

    let source = Some("pastelito".to_owned());

    let (warnings, measurements) = results.into_iter_both();

    let warnings_span = debug_span!("rule_results_to_diagnostics.warnings");
    warnings_span.in_scope(|| {
        let warnings = spans_to_ranges(text, warnings);
        diagnostics.extend(warnings.map(|(range, result)| Diagnostic {
            range: to_vscode_range(range),
            severity: Some(DiagnosticSeverity::ERROR),
            source: source.clone(),
            message: result.message.to_owned(),
            ..Default::default()
        }));
    });

    let measurements_span = debug_span!("rule_results_to_diagnostics.measurements");
    measurements_span.in_scope(|| {
        let measurements = spans_to_ranges(text, measurements);
        diagnostics.extend(measurements.map(|(range, measurement)| Diagnostic {
            range: to_vscode_range(range),
            severity: Some(DiagnosticSeverity::HINT),
            code: Some(NumberOrString::String(measurement.key.into())),
            source: source.clone(),
            message: measurement.key.into(),
            ..Default::default()
        }));
    });

    diagnostics
}

impl Service {
    pub fn new(client: Client) -> Self {
        Service {
            client,
            ruleset: RuleSet::default(),
        }
    }

    fn generate_publish_diagnostics_params(
        &self,
        uri: &Url,
        text: &str,
    ) -> PublishDiagnosticsParams {
        let generate_span = debug_span!("generate_publish_diagnostics_params");
        let diagnostics = generate_span.in_scope(|| {
            let doc = Document::new(&MarkdownParser::default(), text);

            let results = self.ruleset.apply(&doc);

            let diagnostics_span = debug_span!("rule_results_to_diagnostics");
            diagnostics_span.in_scope(|| rule_results_to_diagnostics(text, results))
        });

        PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics,
            version: None,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Service {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        event!(Level::DEBUG, "initialize: {:#?}", params);
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: None,
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        event!(Level::DEBUG, "did_open: uri={}", params.text_document.uri);
        let params = self.generate_publish_diagnostics_params(
            &params.text_document.uri,
            &params.text_document.text,
        );

        self.client
            .send_notification::<PublishDiagnostics>(params)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        event!(Level::DEBUG, "did_change: uri={}", params.text_document.uri);
        for change in params.content_changes {
            let params =
                self.generate_publish_diagnostics_params(&params.text_document.uri, &change.text);

            self.client
                .send_notification::<PublishDiagnostics>(params)
                .await;
        }
    }
}
