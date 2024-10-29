import * as vscode from 'vscode';
import { WASMDisplay } from './display-wasm';

export async function activate(context: vscode.ExtensionContext) {
	context.subscriptions.push(
		await WASMDisplay.create(context.extensionUri)
	);
}