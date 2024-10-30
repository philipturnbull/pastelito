import * as vscode from 'vscode';
import { Display } from './display';
import { pastelito } from './pastelito';
import { Measurement } from './core';
import { Memory, WasmContext } from '@vscode/wasm-component-model';

async function initPastelitoAPI(extensionUri: vscode.Uri): Promise<pastelito.Exports> {
    const filename = vscode.Uri.joinPath(
        extensionUri,
        'out',
        'pastelito_vscode.wasm'
    );

    const bytes = await vscode.workspace.fs.readFile(filename);
    const module = await WebAssembly.compile(bytes);

    const wasmContext: WasmContext.Default = new WasmContext.Default();
    const instance = await WebAssembly.instantiate(module);
    wasmContext.initialize(new Memory.Default(instance.exports));

    return pastelito._.exports.bind(instance.exports as pastelito._.Exports, wasmContext)
}

let API: pastelito.Exports | undefined = undefined;

export class WASMDisplay extends Display {
    diagnostics: vscode.DiagnosticCollection;

    // Constructors can't be async, so we need to init the WASM bundle, then
    // create the display.
    public static async create(extensionUri: vscode.Uri, outputChannel: vscode.OutputChannel): Promise<WASMDisplay> {
        if (!API) {
            API = await initPastelitoAPI(extensionUri);
        }

        return new WASMDisplay(API, outputChannel);
    }

    private constructor(private api: pastelito.Exports, outputChannel: vscode.OutputChannel) {
        super(outputChannel);

        this.diagnostics = vscode.languages.createDiagnosticCollection('pastelito');
        this.disposables.push(this.diagnostics);

        this.disposables.push(
            vscode.workspace.onDidChangeTextDocument((event) => {
                this.update(event.document);
            })
        );

        this.disposables.push(
            vscode.workspace.onDidOpenTextDocument((document) => {
                this.update(document);
            })
        );

        vscode.window.visibleTextEditors.forEach((editor) => {
            this.update(editor.document);
        });
    }

    private update(document: vscode.TextDocument) {
        try {
            this.update_(document);
        } catch (e) {
            this.log(`Update failed for ${document.uri.toString()}:\n${e}`);
        }
    }

    private update_(document: vscode.TextDocument) {
        if (document.languageId !== 'markdown') {
            return;
        }

        const results = this.api.applyDefaultRules(document.getText());

        const uri = document.uri;
        this.setMeasurements(uri, results.measurements.map(Measurement.fromWASM));

        let warnings = results.warnings.map((warning) =>
            new vscode.Diagnostic(
                new vscode.Range(
                    warning.range.startLine,
                    warning.range.startCharUtf16,
                    warning.range.endLine,
                    warning.range.endCharUtf16
                ),
                warning.message,
                vscode.DiagnosticSeverity.Warning
            )
        );

        this.diagnostics.set(uri, warnings);
    }
}