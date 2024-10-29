import * as vscode from 'vscode';
import { Executable, LanguageClient, LanguageClientOptions, ServerOptions } from "vscode-languageclient/node";
import { Display } from './display';
import { Measurement } from './core';

function partition<T>(array: T[], predicate: (value: T) => boolean): [T[], T[]] {
    return array.reduce(
        (acc, value) => {
            acc[predicate(value) ? 0 : 1].push(value);
            return acc;
        },
        [[], []] as [T[], T[]],
    );
}

function getExecutable(): Executable {
    const config = vscode.workspace.getConfiguration('pastelito.lsp');
    let command = config.get('binary', '');
    if (command === '') {
        command = 'pastelito-lsp-vscode';
    }

    const args = [];

    const logFile = config.get('logFile', '');
    if (logFile !== '') {
        args.push('--log-file', logFile);
    }

    return {
        command,
        args
    };
}

export class LSPDisplay extends Display {
    client: LanguageClient;

    constructor(outputChannel: vscode.OutputChannel) {
        super(outputChannel);

        const this_ = this;
        const handleDiagnosticsHook = function (
            this: void,
            uri: vscode.Uri,
            diagnostics: vscode.Diagnostic[],
            next: (uri: vscode.Uri, diagnostics: vscode.Diagnostic[]) => void,
        ): void {
            // Measurements are tagged with a hint severity by
            // `pastelito-lsp-vscode`. We want to display those different, *not* as
            // diagnostics.
            const [measurements, warnings] = partition(diagnostics, diagnostic =>
                diagnostic.severity === vscode.DiagnosticSeverity.Hint
            );

            // Display measurements in the editor.
            this_.update(uri, measurements);

            // Continue with the warnings.
            next(uri, warnings);
        };

        const executable = getExecutable();
        const serverOptions: ServerOptions = {
            run: executable,
            debug: executable,
        };

        const clientOptions: LanguageClientOptions = {
            documentSelector: [
                {
                    scheme: 'file',
                    language: 'markdown'
                }
            ],
            middleware: {
                handleDiagnostics: handleDiagnosticsHook,
            },
            outputChannel: this.outputChannel,
            outputChannelName: 'pastelito',
        };

        this.client = new LanguageClient(
            'pastelito',
            'Pastelito',
            serverOptions,
            clientOptions,
        );

        this.client.start().then(() => {
            this.outputChannel.appendLine('Client started');
        }).catch((err) => {
            const msg = `Pastelito client failed to start: ${err}`;
            this.outputChannel.appendLine(msg);
            vscode.window.showErrorMessage(msg);
        });
    }

    dispose(): void {
        if (this.client) {
            this.client.stop();
        }
    }

    update(uri: vscode.Uri, diagnostics: vscode.Diagnostic[]) {
        this.setMeasurements(uri, diagnostics.map(Measurement.fromDiagnostic));
    }
}