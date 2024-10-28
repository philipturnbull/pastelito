import * as vscode from 'vscode';
import { MeasurementKey } from './core';

export class Theme {
    private colors = new Map<MeasurementKey, string>();

    constructor(
        public name: string,
        colors: {
            abstract_nouns: string, // typically yellowish
            adjectives: string, // typically dark greenish
            academic_ad_words: string, // typically greenish
            be_verbs: string, // typically reddish
            prepositions: string, // typically blueish
        }
    ) {
        this.colors.set('abstract-nouns', colors.abstract_nouns);
        this.colors.set('academic-ad-words', colors.academic_ad_words);
        this.colors.set('adjectives', colors.adjectives);
        this.colors.set('be-verbs', colors.be_verbs);
        this.colors.set('prepositions', colors.prepositions);
    }

    colorFor(measurement: MeasurementKey): string {
        return this.colors.get(measurement)!;
    }

    static from_hex(name: string, hex: string, indexes: {
        abstract_nouns: number,
        academic_ad_words: number,
        adjectives: number,
        be_verbs: number,
        prepositions: number,
    }): Theme {
        const colors = hex.trim().split("\n").map((line) => `#${line.trim()}`);

        return new Theme(name, {
            abstract_nouns: colors[indexes.abstract_nouns],
            academic_ad_words: colors[indexes.academic_ad_words],
            adjectives: colors[indexes.adjectives],
            be_verbs: colors[indexes.be_verbs],
            prepositions: colors[indexes.prepositions]
        });
    }

    static current(): Theme {
        let themeName = vscode.workspace.getConfiguration('pastelito').get<string>('theme');
        if (themeName === undefined) {
            return FALLBACK;
        } else {
            return BUILT_IN_THEMES.get(themeName) || FALLBACK;
        }
    }
}

const FALLBACK_COLOR = 'rgba((var(--vscode-editor-foreground), 0.7)';
export const FALLBACK = new Theme('fallback', {
    abstract_nouns: FALLBACK_COLOR,
    adjectives: FALLBACK_COLOR,
    academic_ad_words: FALLBACK_COLOR,
    be_verbs: FALLBACK_COLOR,
    prepositions: FALLBACK_COLOR,
});

const BUILT_IN_THEMES = new Map<string, Theme>([
    [
        'pastel-qt',

        Theme.from_hex('pastel-qt',
            `
cb8175
e2a97e
f0cf8e
f6edcd
a8c8a6
6d8d8a
655057
`,
            {
                abstract_nouns: 2,
                academic_ad_words: 4,
                adjectives: 5,
                be_verbs: 0,
                prepositions: 6,
            }
        )
    ],

    [
        'fairydust-8',
        Theme.from_hex('fairydust-8',
            `
f0dab1
e39aac
c45d9f
634b7d
6461c2
2ba9b4
93d4b5
f0f6e8
`,
            {
                abstract_nouns: 0,
                academic_ad_words: 6,
                adjectives: 5,
                be_verbs: 2,
                prepositions: 4
            }
        )
    ],
    [
        'curiosities',
        Theme.from_hex('curiosities',
            `
    46425e
    15788c
    00b9be
    ffeecc
    ffb0a3
    ff6973
                `,
            {
                abstract_nouns: 3,
                academic_ad_words: 2,
                adjectives: 1,
                be_verbs: 5,
                prepositions: 4
            }
        )
    ],
    [
        'hydrangea-11',
        Theme.from_hex('hydrangea-11',
            `
    413652
    6f577e
    986f9c
    c090a7
    d4beb8
    eae4dd
    c9d4b8
    90c0a0
    6f919c
    62778c
    575f7e
                `,
            {
                abstract_nouns: 4,
                academic_ad_words: 7,
                adjectives: 6,
                be_verbs: 3,
                prepositions: 8
            }
        )
    ],
    [
        'marumaru-gum',
        Theme.from_hex('marumaru-gum',
            `
    fda9a9
    f3eded
    b9eedc
    96beb1
    82939b
                `,
            {
                abstract_nouns: 1,
                academic_ad_words: 2,
                adjectives: 3,
                be_verbs: 0,
                prepositions: 4,
            }
        )
    ],
    [
        'painted-parchment-9',
        Theme.from_hex('painted-parchment-9',
            `
    dda963
    c9814b
    25272a
    dbc1af
    cf6a4f
    e0b94a
    b2af5c
    a7a79e
    9b6970
                `,
            {
                abstract_nouns: 0,
                academic_ad_words: 6,
                adjectives: 5,
                be_verbs: 4,
                prepositions: 7
            }
        )
    ],
    [
        'sweethope',
        Theme.from_hex('sweethope',
            `
    615e85
    9c8dc2
    d9a3cd
    ebc3a7
    e0e0dc
    a3d1af
    90b4de
    717fb0
                `,
            {
                abstract_nouns: 3,
                academic_ad_words: 5,
                adjectives: 4,
                be_verbs: 2,
                prepositions: 6,
            }
        )
    ],
    /*
    [
        '',
        Theme.from_hex(
            `
            `,
            {
                abstract_nouns: undefined,
                academic_ad_words: undefined,
                be_verbs: undefined,
                prepositions: undefined
            }
        )
    ],
    */
]);
