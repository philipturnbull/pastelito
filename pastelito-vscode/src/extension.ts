import * as vscode from 'vscode';
import { WasmContext, Memory } from '@vscode/wasm-component-model';
import { pastelito } from './pastelito';

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

async function pastelitoAPI(extensionUri: vscode.Uri) {
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

export async function activate(context: vscode.ExtensionContext) {
	const outputChannel = vscode.window.createOutputChannel('pastelito');
	context.subscriptions.push(outputChannel);
	outputChannel.appendLine('pastelito activated');

	const measurementsDisplay = new MeasurementsDisplay(outputChannel);
	context.subscriptions.push(measurementsDisplay);

	const api = await pastelitoAPI(context.extensionUri);
	//const markdown = "This isn't meant to brag or indicate success, but rather just show there has been more than enough effort put into Rust, to dispel the the commonly said 'once you gain enough experience it'll all make sense' argument.*";
	//const results = api.applyDefaultRules(markdown);

	context.subscriptions.push(
		vscode.workspace.onDidChangeTextDocument((event) => {
			if (event.document.languageId === 'markdown') {
				const start = new Date().getTime();
				console.log("start=" + start);
				const results = api.applyDefaultRules(event.document.getText());
				const end = new Date().getTime();
				console.log("wasm end=" + end);
				console.log(`results in ${end - start}ms: ${results.warnings.length} warnings, ${results.measurements.length} measurements`);
				measurementsDisplay.handleResults(event.document.uri, results.measurements);
			}
		})
	);

	const handleDiagnosticsHook = function (
		this: void,
		uri: vscode.Uri,
		diagnostics: vscode.Diagnostic[],
		next: (uri: vscode.Uri, diagnostics: vscode.Diagnostic[]) => void,
	): void {
		const end = new Date().getTime();
		console.log("hook end=" + end);

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