import * as vscode from 'vscode';

export type MeasurementType =
    'abstract-nouns' |
    'academic-ad-words' |
    'adjectives' |
    'be-verbs' |
    'prepositions';

// The order of this array is important. It defines the precedence of the
// types. If a word has multiple types, the first one in this array will be
// used for highlighting.
export const MEASUREMENT_TYPES: MeasurementType[] = [
    'abstract-nouns',
    'academic-ad-words',
    'adjectives',
    'be-verbs',
    'prepositions'
];

const HOVER_MESSAGES = new Map<MeasurementType, string>([
    ['abstract-nouns', 'abstract noun'],
    ['academic-ad-words', 'academic adjective/adverb'],
    ['adjectives', 'adjective/adverb'],
    ['be-verbs', '\'be\' verb'],
    ['prepositions', 'preposition'],
]);

export function hoverMessageFor(measurement: MeasurementType): string {
    return HOVER_MESSAGES.get(measurement)!;
}

export class Measurement {
    constructor(
        public readonly type: MeasurementType,
        public readonly range: vscode.Range,
    ) { }

    static fromDiagnostic(diagnostic: vscode.Diagnostic): Measurement {
        return new Measurement(diagnostic.code as MeasurementType, diagnostic.range);
    }
}