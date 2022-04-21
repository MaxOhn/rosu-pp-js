export interface SingleScoreQuery {
    path: string;
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

export interface MultipleScoreQuery {
    path: string;
    params: MultipleScoreQueryOptions[];
}

export interface MultipleScoreQueryOptions {
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
    mode: 0;
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
    mode: 1;
    ppAcc: number;
    ppStrain: number;
    nCircles: number;
    nSliders: number;
    nSpinners: number;
    maxCombo: number;
}

export interface CatchData extends GeneralData {
    mode: 2;
    nFruits: number;
    nDroplets: number;
    nTinyDroplets: number;
    nSpinners: number;
    maxCombo: number;
}

export interface ManiaData extends GeneralData {
    mode: 3;
    ppAcc: number;
    ppStrain: number;
    clockRate: number;
    nCircles: number;
    nSliders: number;
}

export declare function calculate(query: SingleScoreQuery | MultipleScoreQuery): (OsuData | ManiaData | TaikoData | CatchData)[];
