import * as vscode from 'vscode';
import { WASMDisplay } from './display-wasm';

export async function activate(context: vscode.ExtensionContext) {
	const display = await WASMDisplay.create(context.extensionUri);
	context.subscriptions.push(display);

	context.subscriptions.push(
		vscode.commands.registerCommand('pastelito.toggle', () => {
			display.toggleHighlighting();
		})
	);
}