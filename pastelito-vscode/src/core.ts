import * as vscode from 'vscode';
import { Types } from './pastelito';

export const ABSTRACT_NOUNS = 0;
export const ACADEMIC_AD_WORDS = 1;
export const ADJECTIVES = 2;
export const BE_VERBS = 3;
export const PREPOSITIONS = 4;

export type MeasurementKey = 0 | 1 | 2 | 3 | 4;

const KEY_LOOKUP = new Map<string, MeasurementKey>([
    ['abstract-nouns', ABSTRACT_NOUNS],
    ['academic-ad-words', ACADEMIC_AD_WORDS],
    ['adjectives', ADJECTIVES],
    ['be-verbs', BE_VERBS],
    ['prepositions', PREPOSITIONS],
]);

// The order of this array is important. It defines the precedence of the
// types. If a word has multiple types, the first one in this array will be
// used for highlighting.
export const MEASUREMENT_KEYS: MeasurementKey[] = [
    ABSTRACT_NOUNS,
    ACADEMIC_AD_WORDS,
    ADJECTIVES,
    BE_VERBS,
    PREPOSITIONS
];

const HOVER_MESSAGES = new Map<MeasurementKey, string>([
    [ABSTRACT_NOUNS, 'abstract noun'],
    [ACADEMIC_AD_WORDS, 'academic adjective/adverb'],
    [ADJECTIVES, 'adjective/adverb'],
    [BE_VERBS, '\'be\' verb'],
    [PREPOSITIONS, 'preposition'],
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
        return new Measurement(KEY_LOOKUP.get(diagnostic.code as string)!, diagnostic.range);
    }

    static fromWASM(measurement: Types.Measurement): Measurement {
        const range = new vscode.Range(
            measurement.range.startLine,
            measurement.range.startCharUtf16,
            measurement.range.endLine,
            measurement.range.endCharUtf16
        );

        return new Measurement(measurement.key as MeasurementKey, range);
    }
}