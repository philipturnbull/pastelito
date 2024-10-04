use pastelito_core::parsers::MarkdownParser;
use pastelito_core::rule::Results;
use pastelito_core::rule::RuleSet;
use pastelito_core::ByteSpan;
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
}

struct LineCounter {
    line_num: usize,
    last_span_start: usize,
    start_char_offset_in_line: usize,
}

impl LineCounter {
    fn new() -> Self {
        LineCounter {
            line_num: 0,
            start_char_offset_in_line: 0,
            last_span_start: 0,
        }
    }

    fn span_to_range(&mut self, text: &str, span: ByteSpan) -> Range {
        let start = span.start();
        let end = span.end();

        if start < self.last_span_start {
            panic!("span out of order");
        }

        let (start_line_num, start_char_offset_in_line) = if start == self.last_span_start {
            (self.line_num, self.start_char_offset_in_line)
        } else {
            self.line_num += text[self.last_span_start..start]
                .chars()
                .filter(|&c| c == '\n')
                .count();
            self.last_span_start = start;

            self.start_char_offset_in_line = text[..start]
                .chars()
                .rev()
                .take_while(|&c| c != '\n')
                .count();

            (self.line_num, self.start_char_offset_in_line)
        };

        let end_line_num = start_line_num + text[start..end].chars().filter(|&c| c == '\n').count();
        let end_char_offset_in_line = text[..end].chars().rev().take_while(|&c| c != '\n').count();

        Range {
            start: Position::new(start_line_num as u32, start_char_offset_in_line as u32),
            end: Position::new(end_line_num as u32, end_char_offset_in_line as u32),
        }
    }
}

fn rule_results_to_diagnostics(text: &str, results: Results) -> Vec<Diagnostic> {
    let mut diagnostics =
        Vec::with_capacity(results.iter_measurements().count() + results.iter_warnings().count());

    let source = Some("pastelito".to_owned());

    let (warnings, measurements) = results.into_iter_both();

    let mut counter = LineCounter::new();
    diagnostics.extend(warnings.map(|result| {
        let range = counter.span_to_range(text, result.span);
        Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::WARNING),
            source: source.clone(),
            message: result.message.to_owned(),
            ..Default::default()
        }
    }));

    let mut counter = LineCounter::new();
    diagnostics.extend(measurements.map(|(key, word)| {
        let range = counter.span_to_range(text, word.span);
        Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::HINT),
            code: Some(NumberOrString::String(key.into())),
            source: source.clone(),
            message: key.into(),
            ..Default::default()
        }
    }));

    diagnostics
}

impl Service {
    pub fn new(client: Client) -> Self {
        Service { client }
    }

    fn generate_publish_diagnostics_params(
        &self,
        uri: &Url,
        text: &str,
    ) -> PublishDiagnosticsParams {
        let generate_span = debug_span!("generate_publish_diagnostics_params");
        let diagnostics = generate_span.in_scope(|| {
            let ruleset_span = debug_span!("default_ruleset");
            let ruleset = ruleset_span.in_scope(RuleSet::default);

            let doc_span = debug_span!("Document::new");
            let doc = doc_span.in_scope(|| Document::new(&MarkdownParser::default(), text));

            let results_span = debug_span!("ruleset.apply");
            let results = results_span.in_scope(|| ruleset.apply(&doc));

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
