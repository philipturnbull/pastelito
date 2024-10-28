import * as vscode from 'vscode';
import { Types } from './pastelito';

export type MeasurementKey =
    'abstract-nouns' |
    'academic-ad-words' |
    'adjectives' |
    'be-verbs' |
    'prepositions';

// The order of this array is important. It defines the precedence of the
// types. If a word has multiple types, the first one in this array will be
// used for highlighting.
export const MEASUREMENT_KEYS: MeasurementKey[] = [
    'abstract-nouns',
    'academic-ad-words',
    'adjectives',
    'be-verbs',
    'prepositions'
];

const HOVER_MESSAGES = new Map<MeasurementKey, string>([
    ['abstract-nouns', 'abstract noun'],
    ['academic-ad-words', 'academic adjective/adverb'],
    ['adjectives', 'adjective/adverb'],
    ['be-verbs', '\'be\' verb'],
    ['prepositions', 'preposition'],
]);

export function hoverMessageFor(measurement: MeasurementKey): string {
    return HOVER_MESSAGES.get(measurement)!;
}

export class Measurement {
    constructor(
        public readonly key: MeasurementKey,
        public readonly range: vscode.Range,
    ) { }

    static fromDiagnostic(diagnostic: vscode.Diagnostic): Measurement {
        return new Measurement(diagnostic.code as MeasurementKey, diagnostic.range);
    }

    static fromWASM(measurement: Types.Measurement): Measurement {
        const range = new vscode.Range(
            measurement.range.startLine,
            measurement.range.startChar,
            measurement.range.endLine,
            measurement.range.endChar
        );
        return new Measurement(measurement.key as MeasurementKey, range);
    }
}