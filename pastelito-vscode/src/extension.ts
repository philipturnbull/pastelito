import * as vscode from 'vscode';
import { WASMDisplay } from './display-wasm';

export async function activate(context: vscode.ExtensionContext) {
	const outputChannel = vscode.window.createOutputChannel('Pastelito');
	const display = await WASMDisplay.create(context.extensionUri, outputChannel);
	context.subscriptions.push(display);

	context.subscriptions.push(
		vscode.commands.registerCommand('pastelito.toggle', () => {
			display.toggleHighlighting();
		})
	);
}