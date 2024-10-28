import * as vscode from 'vscode';

import {
	Executable,
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
} from 'vscode-languageclient/node';
import { MeasurementsDisplay } from './display';

function partition<T>(array: T[], predicate: (value: T) => boolean): [T[], T[]] {
	return array.reduce(
		(acc, value) => {
			acc[predicate(value) ? 0 : 1].push(value);
			return acc;
		},
		[[], []] as [T[], T[]],
	);
}

let client: LanguageClient | undefined;

function getExecutable(): Executable {
	const config = vscode.workspace.getConfiguration('pastelito');
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

export function activate(context: vscode.ExtensionContext) {
	const outputChannel = vscode.window.createOutputChannel('pastelito');
	context.subscriptions.push(outputChannel);
	outputChannel.appendLine('pastelito activated');

	const measurementsDisplay = new MeasurementsDisplay(outputChannel);
	context.subscriptions.push(measurementsDisplay);

	const handleDiagnosticsHook = function(
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
		measurementsDisplay.handleDiagnostics(uri, measurements);

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
		outputChannel,
		outputChannelName: 'pastelito',
	};

	client = new LanguageClient(
		'pastelito',
		'Pastelito',
		serverOptions,
		clientOptions,
	);

	client.start().then(() => {
		outputChannel.appendLine('Client started');
	}).catch((err) => {
		const msg = `Pastelito client failed to start: ${err}`;
		outputChannel.appendLine(msg);
		vscode.window.showErrorMessage(msg);
	});
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}