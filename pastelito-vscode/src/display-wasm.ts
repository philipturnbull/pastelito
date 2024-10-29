import * as vscode from 'vscode';
import { Display } from './display';
import { pastelito } from './pastelito';
import { Measurement } from './core';
import { Memory, WasmContext } from '@vscode/wasm-component-model';

async function initPastelitoAPI(extensionUri: vscode.Uri): Promise<pastelito.Exports> {
    const filename = vscode.Uri.joinPath(
        extensionUri,
        'target-wasm',
        'wasm32-unknown-unknown',
        'release',
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
    public static async create(outputChannel: vscode.OutputChannel, extensionUri: vscode.Uri): Promise<WASMDisplay> {
        if (!API) {
            API = await initPastelitoAPI(extensionUri);
        }

        return new WASMDisplay(outputChannel, API);
    }

    private constructor(outputChannel: vscode.OutputChannel, private api: pastelito.Exports) {
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
                    warning.range.startChar,
                    warning.range.endLine,
                    warning.range.endChar
                ),
                warning.message,
                vscode.DiagnosticSeverity.Warning
            )
        );

        this.diagnostics.set(uri, warnings);
    }
}