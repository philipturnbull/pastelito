import * as vscode from 'vscode';
import { Display } from './display';
import { LSPDisplay } from './display-lsp';
import { WASMDisplay } from './display-wasm';

export async function activate(context: vscode.ExtensionContext) {
	const outputChannel = vscode.window.createOutputChannel('pastelito');
	context.subscriptions.push(outputChannel);
	outputChannel.appendLine('pastelito activated');

	const measurementsDisplay = new Display(outputChannel);
	context.subscriptions.push(measurementsDisplay);

	context.subscriptions.push(
		vscode.workspace.onDidCloseTextDocument((document) => {
			measurementsDisplay.clearCache(document.uri);
		})
	);

	const useLSP = vscode.workspace.getConfiguration('pastelito').get('useLSP', false);

	if (useLSP) {
		context.subscriptions.push(new LSPDisplay(outputChannel));
	} else {
		context.subscriptions.push(await WASMDisplay.create(outputChannel, context.extensionUri));
	}
}