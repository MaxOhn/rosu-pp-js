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
    timePreempt: number;
    greatHitwindow: number;
}

export interface TaikoData extends GeneralData {
    mode: GameMode.Taiko;
    ppAcc: number;
    ppStrain: number;
    nCircles: number;
    nSliders: number;
    nSpinners: number;
    maxCombo: number;
    greatHitwindow: number;
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
    greatHitwindow: number;
}

export declare function calculate(query: SingleScoreQuery | MultipleScoreQuery): (OsuData | ManiaData | TaikoData | CatchData)[];

export interface GeneralStrains {
    sectionLength: number;
}

export interface OsuStrains extends GeneralStrains {
    mode: GameMode.Osu;
    aim: Array<number>;
    aimNoSliders: Array<number>;
    speed: Array<number>;
    flashlight: Array<number>;
}

export interface TaikoStrains extends GeneralStrains {
    mode: GameMode.Taiko;
    color: Array<number>;
    rhythm: Array<number>;
    staminaLeft: Array<number>;
    staminaRight: Array<number>;
}

export interface CatchStrains extends GeneralStrains {
    mode: GameMode.Catch;
    movement: Array<number>;
}

export interface ManiaStrains extends GeneralStrains {
    mode: GameMode.Mania;
    strains: Array<number>;
}

export declare function strains(path: string, mods?: number): OsuStrains | TaikoStrains | CatchStrains | ManiaStrains;
