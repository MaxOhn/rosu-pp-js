export interface SingleScoreQuery {
    path: string;
    mode?: GameMode;
    mods?: number;
    acc?: number;
    n300?: number;
    n100?: number;
    n50?: number;
    nMisses?: number;
    nKatu?: number;
    combo?: number;
    score?: number;
    passedObjects?: number;
    clockRate?: number;
    ar?: number;
    cs?: number;
    hp?: number;
    od?: number;
}

export const enum GameMode {
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

export interface MultipleScoreQuery {
    path: string;
    params: MultipleScoreQueryOptions[];
}

export interface MultipleScoreQueryOptions {
    mode?: GameMode;
    mods?: number;
    acc?: number;
    n300?: number;
    n100?: number;
    n50?: number;
    nMisses?: number;
    nKatu?: number;
    combo?: number;
    score?: number;
    passedObjects?: number;
    clockRate?: number;
    ar?: number;
    cs?: number;
    hp?: number;
    od?: number;
}

export interface GeneralData {
    stars: number;
    pp: number;
    ar: number;
    cs: number;
    hp: number;
    od: number;
    bpm: number;
    clockRate: number;
}

export interface OsuData extends GeneralData {
    mode: GameMode.Osu;
    ppAcc: number;
    ppAim: number;
    ppFlashlight: number;
    ppSpeed: number;
    aimStrain: number;
    speedStrain: number;
    flashlightRating: number;
    sliderFactor: number;
    clockRate: number;
    nCircles: number;
    nSliders: number;
    nSpinners: number;
    maxCombo: number;
}

export interface TaikoData extends GeneralData {
    mode: GameMode.Taiko;
    ppAcc: number;
    ppStrain: number;
    nCircles: number;
    nSliders: number;
    nSpinners: number;
    maxCombo: number;
}

export interface CatchData extends GeneralData {
    mode: GameMode.Catch;
    nFruits: number;
    nDroplets: number;
    nTinyDroplets: number;
    nSpinners: number;
    maxCombo: number;
}

export interface ManiaData extends GeneralData {
    mode: GameMode.Mania;
    ppAcc: number;
    ppStrain: number;
    clockRate: number;
    nCircles: number;
    nSliders: number;
}

export declare function calculate(query: SingleScoreQuery | MultipleScoreQuery): (OsuData | ManiaData | TaikoData | CatchData)[];
