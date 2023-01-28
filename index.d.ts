export class Beatmap {
    constructor(params?: BeatmapParams);

    fromPath(path: string): Beatmap;
    fromContent(content: string | Uint8Array): Beatmap;
    fromBytes(bytes: Uint8Array): Beatmap;

    ar(ar: number): Beatmap;
    cs(cs: number): Beatmap;
    hp(hp: number): Beatmap;
    od(od: number): Beatmap;
}

export interface BeatmapParams {
    path?: string,
    content?: string | Uint8Array,
    bytes?: Uint8Array,
    ar?: number,
    cs?: number,
    hp?: number,
    od?: number,
}

export class Calculator {
    constructor(score?: Score);

    mapAttributes(map: Beatmap): MapAttributes;
    difficulty(map: Beatmap): DifficultyAttributes;
    performance(map: Beatmap): PerformanceAttributes;
    strains(map: Beatmap): Strains;

    mode(mode: GameMode): Calculator;
    mods(mods: number): Calculator;
    acc(acc: number): Calculator;
    nGeki(nGeki: number): Calculator;
    nKatu(nKatu: number): Calculator;
    n300(n300: number): Calculator;
    n100(n100: number): Calculator;
    n50(n50: number): Calculator;
    nMisses(nMisses: number): Calculator;
    combo(combo: number): Calculator;
    passedObjects(passedObjects: number): Calculator;
    clockRate(clockRate: number): Calculator;
}

export interface Score {
    mode?: GameMode,
    mods?: number,
    acc?: number,
    nGeki?: number,
    nKatu?: number,
    n300?: number,
    n100?: number,
    n50?: number,
    nMisses?: number,
    combo?: number,
    passedObjects?: number,
    clockRate?: number,
}

export interface MapAttributes {
    mode: GameMode,
    version: number,
    nCircles: number,
    nSliders: number,
    nSpinners: number,
    ar: number,
    cs: number,
    hp: number,
    od: number,
    arHitWindow: number,
    odHitWindow: number,
    clockRate: number,
    bpm: number,
}

export const enum GameMode {
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

export interface AnyDifficultyAttributes {
    stars: number,
    maxCombo: number,
}

export interface OsuDifficultyAttributes extends AnyDifficultyAttributes {
    mode: GameMode.Osu,
    aim: number,
    speed: number,
    flashlight: number,
    sliderFactor: number,
    speedNoteCount: number,
    ar: number,
    od: number,
    nCircles: number,
    nSliders: number,
    nSpinners: number,
}

export interface TaikoDifficultyAttributes extends AnyDifficultyAttributes {
    mode: GameMode.Taiko,
    stamina: number,
    rhythm: number,
    color: number,
    peak: number,
    hitWindow: number,
}

export interface CatchDifficultyAttributes extends AnyDifficultyAttributes {
    mode: GameMode.Catch,
    ar: number,
    nFruits: number,
    nDroplets: number,
    nTinyDroplets: number,
}

export interface ManiaDifficultyAttributes extends AnyDifficultyAttributes {
    mode: GameMode.Mania,
    hitWindow: number,
}

export type DifficultyAttributes = OsuDifficultyAttributes | TaikoDifficultyAttributes | CatchDifficultyAttributes | ManiaDifficultyAttributes;

export interface AnyPerformanceAttributes {
    pp: number,
}

export interface OsuPerformanceAttributes extends AnyPerformanceAttributes {
    mode: GameMode.Osu,
    difficulty: OsuDifficultyAttributes,
    ppAcc: number,
    ppAim: number,
    ppFlashlight: number,
    ppSpeed: number,
    effectiveMissCount: number,
}

export interface TaikoPerformanceAttributes extends AnyPerformanceAttributes {
    mode: GameMode.Taiko,
    difficulty: TaikoDifficultyAttributes,
    ppAcc: number,
    ppDifficulty: number,
    effectiveMissCount: number,
}

export interface CatchPerformanceAttributes extends AnyPerformanceAttributes {
    mode: GameMode.Catch,
    difficulty: CatchDifficultyAttributes,
    ppDifficulty: number,
}

export interface ManiaPerformanceAttributes extends AnyPerformanceAttributes {
    mode: GameMode.Mania,
    difficulty: ManiaDifficultyAttributes,
}

export type PerformanceAttributes = OsuPerformanceAttributes | TaikoPerformanceAttributes | CatchPerformanceAttributes | ManiaPerformanceAttributes;

export interface AnyStrains {
    sectionLength: number,
}

export interface OsuStrains extends AnyStrains {
    mode: GameMode.Osu,
    aim: Array<number>,
    aimNoSliders: Array<number>,
    speed: Array<number>,
    flashlight: Array<number>,
}

export interface TaikoStrains extends AnyStrains {
    mode: GameMode.Taiko,
    color: Array<number>,
    rhythm: Array<number>,
    stamina: Array<number>,
}

export interface CatchStrains extends AnyStrains {
    mode: GameMode.Catch,
    movement: Array<number>,
}

export interface ManiaStrains extends AnyStrains {
    mode: GameMode.Mania,
    strains: Array<number>,
}

export type Strains = OsuStrains | TaikoStrains | CatchStrains | ManiaStrains;
