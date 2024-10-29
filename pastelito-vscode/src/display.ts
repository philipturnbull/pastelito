import * as vscode from 'vscode';
import { Theme } from './theme';
import { hoverMessageFor, Measurement, MEASUREMENT_KEYS, MeasurementKey } from './core';

interface RenderOptions {
    renderOptions(): vscode.DecorationRenderOptions;
}

class ForegroundRenderOptions implements RenderOptions {
    constructor(private color: string) { }

    renderOptions(): vscode.DecorationRenderOptions {
        return {
            color: this.color
        };
    }
}

class BorderRenderOptions implements RenderOptions {
    constructor(private color: string) { }

    renderOptions(): vscode.DecorationRenderOptions {
        return {
            borderWidth: '2px',
            borderStyle: 'dotted',
            borderColor: this.color
        };
    }
}

class Matcher implements vscode.Disposable {
    public decorationType: vscode.TextEditorDecorationType;

    constructor(
        public measurementType: MeasurementKey,
        public hoverMessage: string,
        renderOptions: RenderOptions
    ) {
        this.decorationType = vscode.window.createTextEditorDecorationType(
            renderOptions.renderOptions()
        );
    }

    decorationFor(measurement: Measurement): DecorationMatch | undefined {
        if (measurement.key === this.measurementType) {
            return {
                decorationType: this.decorationType,
                decorationOptions: {
                    range: measurement.range,
                    hoverMessage: this.hoverMessage
                }
            };
        }
    }

    dispose() {
        this.decorationType.dispose();
    }
}

type DecorationMatch = {
    decorationType: vscode.TextEditorDecorationType,
    decorationOptions: {
        range: vscode.Range,
        hoverMessage: string,
    }
}

class Matchers implements vscode.Disposable {
    private matchers: Matcher[];

    public constructor(theme: Theme) {
        this.matchers = MEASUREMENT_KEYS.map(
            (type) =>
                new Matcher(
                    type,
                    hoverMessageFor(type),
                    new ForegroundRenderOptions(theme.colorFor(type))
                )
        )
    }

    decorationFor(measurement: Measurement): DecorationMatch | undefined {
        for (const matcher of this.matchers) {
            const decoration = matcher.decorationFor(measurement);
            if (decoration) {
                return decoration;
            }
        }
    }

    partition(measurements: Measurement[]) {
        const parts = new Map<vscode.TextEditorDecorationType, vscode.DecorationOptions[]>();

        measurements.forEach(measurement => {
            const match = this.decorationFor(measurement);
            if (!match) {
                return;
            }

            const decorationOptions = parts.get(match.decorationType) || [];
            decorationOptions.push(match.decorationOptions);
            parts.set(match.decorationType, decorationOptions);
        });

        return parts;
    }

    dispose() {
        this.matchers.forEach(matcher => matcher.dispose());
    }
}

export class Display implements vscode.Disposable {
    protected outputChannel: vscode.OutputChannel;
    protected disposables: vscode.Disposable[] = [];
    private matchers: Matchers;

    // We store a cache of measurements for each document. This lets us change
    // the theme without triggering a request to the LSP
    private measurementCache: Map<string, Measurement[]> = new Map();

    constructor(outputChannel: vscode.OutputChannel) {
        this.outputChannel = outputChannel;

        this.disposables.push(
            vscode.workspace.onDidChangeConfiguration((event) => {
                if (event.affectsConfiguration('pastelito.theme')) {
                    const oldMatchers = this.matchers;

                    // Create new matchers with the new theme.
                    this.matchers = new Matchers(Theme.current());
                    // Apply the new matchers to all visible editors.
                    for (const editor of vscode.window.visibleTextEditors) {
                        this.createDecorations(editor);
                    }

                    // Dispose of the old decorations after applying the new ones. This
                    // prevents flickering.
                    oldMatchers.dispose();
                }
            })
        );

        this.disposables.push(
            vscode.window.onDidChangeVisibleTextEditors((editors) => {
                for (const editor of editors) {
                    this.createDecorations(editor);
                }
            })
        );

        this.disposables.push(
            vscode.workspace.onDidCloseTextDocument((document) => {
                this.clearCache(document.uri);
            })
        );

        this.matchers = new Matchers(Theme.current());
    }

    dispose() {
        this.matchers.dispose();
        this.disposables.forEach(disposable => disposable.dispose());
    }

    clearCache(uri: vscode.Uri) {
        this.measurementCache.delete(uri.toString());
    }

    protected setMeasurements(uri: vscode.Uri, measurements: Measurement[]) {
        this.measurementCache.set(uri.toString(), measurements);

        const editor = vscode.window.visibleTextEditors.find(
            (editor) => editor.document.uri.toString() === uri.toString()
        );

        if (editor) {
            this.createDecorations(editor);
        }
    }

    private createDecorations(editor: vscode.TextEditor) {
        const measurements = this.measurementCache.get(editor.document.uri.toString());
        if (measurements === undefined) {
            return;
        }

        for (const [decorationType, decorationOptions] of this.matchers.partition(measurements)) {
            editor.setDecorations(decorationType, decorationOptions);
        }
    }
}