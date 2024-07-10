/* tslint:disable */
/* eslint-disable */
/**
* While generating remaining hitresults, decide how they should be distributed.
*/
export enum HitResultPriority {
/**
* Prioritize good hitresults over bad ones
*/
  BestCase = 0,
/**
* Prioritize bad hitresults over good ones
*/
  WorstCase = 1,
}
/**
*/
export enum GameMode {
  Osu = 0,
  Taiko = 1,
  Catch = 2,
  Mania = 3,
}
/**
* The content of a `.osu` file either as bytes or string.
*/
export type BeatmapContent = Uint8Array | string;

/**
* Arguments to provide the `BeatmapAttributesBuilder` constructor.
*/
export interface BeatmapAttributesArgs extends CommonArgs {
    /**
    * Specify a gamemode.
    */
    mode?: GameMode;
    /**
    * Specify whether it's a converted map.
    */
    isConvert?: boolean;
    /**
    * Start off with a beatmap's attributes, mode, and convert status.
    */
    map?: Beatmap;
}

/**
* Common properties to extend other argument interfaces.
*/
export interface CommonArgs {
    /**
    * Specify mods.
    *
    * The type must be either
    *   - an integer for bitflags
    *   - a string for acronyms
    *   - a single mod object as described below
    *   - a sequence of types that deserialize into a single mod
    *
    * Types that deserialize into a single mod are
    *   - an integer for bitflags
    *   - a string for an acronym
    *   - a mod object
    *
    * A mod object must have an `acronym: string` property and an optional
    * `settings?: Object` property.
    *
    * See <https://github.com/ppy/osu-api/wiki#mods>
    */
    mods?: Object;
    /**
    * Adjust the clock rate used in the calculation.
    *
    * If none is specified, it will take the clock rate based on the mods
    * i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.
    *
    * | Minimum | Maximum |
    * | :-----: | :-----: |
    * | 0.01    | 100     |
    */
    clockRate?: number;
    /**
    * Override a beatmap's set AR.
    *
    * Only relevant for osu! and osu!catch.
    *
    * | Minimum | Maximum |
    * | :-----: | :-----: |
    * | -20     | 20      |
    */
    ar?: number;
    /**
    * Determines if the given AR value should be used before
    * or after accounting for mods, e.g. on `true` the value will be
    * used as is and on `false` it will be modified based on the mods.
    */
    arWithMods?: boolean;
    /**
    * Override a beatmap's set CS.
    *
    * Only relevant for osu! and osu!catch.
    *
    * | Minimum | Maximum |
    * | :-----: | :-----: |
    * | -20     | 20      |
    */
    cs?: number;
    /**
    * Determines if the given CS value should be used before
    * or after accounting for mods, e.g. on `true` the value will be
    * used as is and on `false` it will be modified based on the mods.
    */
    csWithMods?: boolean;
    /**
    * Override a beatmap's set HP.
    *
    * | Minimum | Maximum |
    * | :-----: | :-----: |
    * | -20     | 20      |
    */
    hp?: number;
    /**
    * Determines if the given HP value should be used before
    * or after accounting for mods, e.g. on `true` the value will be
    * used as is and on `false` it will be modified based on the mods.
    */
    hpWithMods?: boolean;
    /**
    * Override a beatmap's set OD.
    *
    * | Minimum | Maximum |
    * | :-----: | :-----: |
    * | -20     | 20      |
    */
    od?: number;
    /**
    * Determines if the given OD value should be used before
    * or after accounting for mods, e.g. on `true` the value will be
    * used as is and on `false` it will be modified based on the mods.
    */
    odWithMods?: boolean;
}

/**
* Arguments to provide the `Difficulty` constructor.
*/
export interface DifficultyArgs extends CommonArgs {
    /**
    * Amount of passed objects for partial plays, e.g. a fail.
    *
    * If you want to calculate the difficulty after every few objects,
    * instead of using `Difficulty` multiple times with different
    * `passedObjects`, you should use `GradualDifficulty`.
    */
    passedObjects?: number;
    /**
    * Adjust patterns as if the HR mod is enabled.
    *
    * Only relevant for osu!catch.
    */
    hardrockOffsets?: boolean;
}

/**
* Arguments to provide the `Performance` constructor.
*/
export interface PerformanceArgs extends DifficultyArgs {
    /**
    * Set the accuracy between `0.0` and `100.0`.
    */
    accuracy?: number;
    /**
    * Specify the max combo of the play.
    *
    * Irrelevant for osu!mania.
    */
    combo?: number;
    /**
    * Specify the amount of gekis of a play.
    *
    * Only relevant for osu!mania for which it repesents the amount of n320.
    */
    nGeki?: number;
    /**
    * Specify the amount of katus of a play.
    *
    * Only relevant for osu!catch for which it represents the amount of tiny
    * droplet misses and osu!mania for which it repesents the amount of n200.
    */
    nKatu?: number;
    /**
    * Specify the amount of 300s of a play.
    */
    n300?: number;
    /**
    * Specify the amount of 100s of a play.
    */
    n100?: number;
    /**
    * Specify the amount of 50s of a play.
    *
    * Irrelevant for osu!taiko.
    */
    n50?: number;
    /**
    * Specify the amount of misses of a play.
    */
    misses?: number;
    /**
    * Specify how hitresults should be generated.
    *
    * Defaults to `HitResultPriority.BestCase`.
    */
    hitresultPriority?: HitResultPriority;
}

/**
* Either previously calculated attributes or a beatmap.
*/
export type MapOrAttributes = DifficultyAttributes | PerformanceAttributes | Beatmap;

/**
* Arguments to provide the `Difficulty` constructor.
*/
export interface ScoreState {
    /**
    * Maximum combo that the score has had so far. **Not** the maximum
    * possible combo of the map so far.
    *
    * Note that for osu!catch only fruits and droplets are considered for
    * combo.
    *
    * Irrelevant for osu!mania.
    */
    maxCombo?: number;
    /**
    * Amount of current gekis (n320 for osu!mania).
    */
    nGeki?: number;
    /**
    * Amount of current katus (tiny droplet misses for osu!catch / n200 for
    * osu!mania).
    */
    nKatu?: number;
    /**
    * Amount of current 300s (fruits for osu!catch).
    */
    n300?: number;
    /**
    * Amount of current 100s (droplets for osu!catch).
    */
    n100?: number;
    /**
    * Amount of current 50s (tiny droplets for osu!catch).
    */
    n50?: number;
    /**
    * Amount of current misses (fruits + droplets for osu!catch).
    */
    misses?: number;
}

/**
* All beatmap data that is relevant for difficulty and performance
* calculation.
*
* It is recommended to call the method `Beatmap.free` on instances that are
* no longer in use to avoid the risk of leaking memory.
*/
export class Beatmap {
  free(): void;
/**
* Create a new beatmap instance by parsing an `.osu` file's content.
* @throws Throws an error if decoding the map failed
* @param {BeatmapContent} args
*/
  constructor(args: BeatmapContent);
/**
* Convert a beatmap to a specific mode.
* @throws Throws an error if the specified mode is incompatible with the map's mode
* @param {GameMode} mode
*/
  convert(mode: GameMode): void;
/**
*/
  readonly ar: number;
/**
*/
  readonly bpm: number;
/**
*/
  readonly cs: number;
/**
*/
  readonly hp: number;
/**
*/
  readonly isConvert: boolean;
/**
*/
  readonly mode: GameMode;
/**
*/
  readonly nBreaks: number;
/**
*/
  readonly nCircles: number;
/**
*/
  readonly nHolds: number;
/**
*/
  readonly nObjects: number;
/**
*/
  readonly nSliders: number;
/**
*/
  readonly nSpinners: number;
/**
*/
  readonly od: number;
/**
*/
  readonly sliderMultiplier: number;
/**
*/
  readonly sliderTickRate: number;
/**
*/
  readonly stackLeniency: number;
/**
*/
  readonly version: number;
}
/**
*/
export class BeatmapAttributes {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
/**
* The approach rate.
*/
  readonly ar: number;
/**
* Hit window for approach rate i.e. TimePreempt in milliseconds.
*/
  readonly arHitWindow: number;
/**
* The clock rate with respect to mods.
*/
  readonly clockRate: number;
/**
* The circle size.
*/
  readonly cs: number;
/**
* The health drain rate
*/
  readonly hp: number;
/**
* The overall difficulty.
*/
  readonly od: number;
/**
* Hit window for overall difficulty i.e. time to hit a 300 ("Great") in
* milliseconds.
*/
  readonly odHitWindow: number;
}
/**
*/
export class BeatmapAttributesBuilder {
  free(): void;
/**
* Create a new `BeatmapAttributesBuilder`.
* @param {BeatmapAttributesArgs | undefined} [args]
*/
  constructor(args?: BeatmapAttributesArgs);
/**
* Calculate the `BeatmapAttributes`.
* @returns {BeatmapAttributes}
*/
  build(): BeatmapAttributes;
/**
*/
  ar?: number;
/**
*/
  arWithMods?: boolean;
/**
*/
  clockRate?: number;
/**
*/
  cs?: number;
/**
*/
  csWithMods?: boolean;
/**
*/
  hp?: number;
/**
*/
  hpWithMods?: boolean;
/**
*/
  isConvert?: boolean;
/**
*/
  map?: Beatmap;
/**
*/
  mode?: GameMode;
/**
*/
  mods?: Object;
/**
*/
  od?: number;
/**
*/
  odWithMods?: boolean;
}
/**
* Builder for a difficulty calculation.
*/
export class Difficulty {
  free(): void;
/**
* Create a new difficulty calculator.
* @param {DifficultyArgs | undefined} [args]
*/
  constructor(args?: DifficultyArgs);
/**
* Perform the difficulty calculation.
* @param {Beatmap} map
* @returns {DifficultyAttributes}
*/
  calculate(map: Beatmap): DifficultyAttributes;
/**
* Perform the difficulty calculation but instead of evaluating strain
* values, return them as is.
*
* Suitable to plot the difficulty over time.
* @param {Beatmap} map
* @returns {Strains}
*/
  strains(map: Beatmap): Strains;
/**
* Returns a gradual difficulty calculator for the current difficulty settings.
* @param {Beatmap} map
* @returns {GradualDifficulty}
*/
  gradualDifficulty(map: Beatmap): GradualDifficulty;
/**
* Returns a gradual performance calculator for the current difficulty settings.
* @param {Beatmap} map
* @returns {GradualPerformance}
*/
  gradualPerformance(map: Beatmap): GradualPerformance;
/**
*/
  ar?: number;
/**
*/
  arWithMods?: boolean;
/**
*/
  clockRate?: number;
/**
*/
  cs?: number;
/**
*/
  csWithMods?: boolean;
/**
*/
  hardrockOffsets?: boolean;
/**
*/
  hp?: number;
/**
*/
  hpWithMods?: boolean;
/**
*/
  mods?: Object;
/**
*/
  od?: number;
/**
*/
  odWithMods?: boolean;
/**
*/
  passedObjects?: number;
}
/**
* The result of a difficulty calculation.
*/
export class DifficultyAttributes {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
/**
* The difficulty of the aim skill.
*
* Only available for osu!.
*/
  readonly aim: number | undefined;
/**
* The approach rate.
*
* Only available for osu! and osu!catch.
*/
  readonly ar: number | undefined;
/**
* The difficulty of the color skill.
*
* Only available for osu!taiko.
*/
  readonly color: number | undefined;
/**
* The difficulty of the flashlight skill.
*
* Only available for osu!.
*/
  readonly flashlight: number | undefined;
/**
* The perceived hit window for an n300 inclusive of rate-adjusting mods
* (DT/HT/etc)
*
* Only available for osu!taiko and osu!mania.
*/
  readonly hitWindow: number | undefined;
/**
* The health drain rate.
*
* Only available for osu!.
*/
  readonly hp: number | undefined;
/**
* Whether the map was a convert i.e. an osu! map.
*/
  readonly isConvert: boolean;
/**
* Return the maximum combo.
*/
  readonly maxCombo: number;
/**
* The attributes' gamemode.
*/
  readonly mode: GameMode;
/**
* The amount of circles.
*
* Only available for osu!.
*/
  readonly nCircles: number | undefined;
/**
* The amount of droplets.
*
* Only available for osu!catch.
*/
  readonly nDroplets: number | undefined;
/**
* The amount of fruits.
*
* Only available for osu!catch.
*/
  readonly nFruits: number | undefined;
/**
* The amount of hitobjects in the map.
*
* Only available for osu!mania.
*/
  readonly nObjects: number | undefined;
/**
* The amount of sliders.
*
* Only available for osu!.
*/
  readonly nSliders: number | undefined;
/**
* The amount of spinners.
*
* Only available for osu!.
*/
  readonly nSpinners: number | undefined;
/**
* The amount of tiny droplets.
*
* Only available for osu!catch.
*/
  readonly nTinyDroplets: number | undefined;
/**
* The overall difficulty
*
* Only available for osu!.
*/
  readonly od: number | undefined;
/**
* The difficulty of the hardest parts of the map.
*
* Only available for osu!taiko.
*/
  readonly peak: number | undefined;
/**
* The difficulty of the rhythm skill.
*
* Only available for osu!taiko.
*/
  readonly rhythm: number | undefined;
/**
* The ratio of the aim strain with and without considering sliders
*
* Only available for osu!.
*/
  readonly sliderFactor: number | undefined;
/**
* The difficulty of the speed skill.
*
* Only available for osu!.
*/
  readonly speed: number | undefined;
/**
* The number of clickable objects weighted by difficulty.
*
* Only available for osu!.
*/
  readonly speedNoteCount: number | undefined;
/**
* The difficulty of the stamina skill.
*
* Only available for osu!taiko.
*/
  readonly stamina: number | undefined;
/**
* The final star rating.
*/
  readonly stars: number;
}
/**
* Gradually calculate difficulty attributes after each hitobject.
*/
export class GradualDifficulty {
  free(): void;
/**
* @param {Difficulty} difficulty
* @param {Beatmap} map
*/
  constructor(difficulty: Difficulty, map: Beatmap);
/**
* Advances the iterator and returns the next attributes.
* @returns {DifficultyAttributes | undefined}
*/
  next(): DifficultyAttributes | undefined;
/**
* Returns the `n`th attributes of the iterator.
*
* Note that the count starts from zero, so `nth(0)` returns the first
* value, `nth(1)` the second, and so on.
* @param {number} n
* @returns {DifficultyAttributes | undefined}
*/
  nth(n: number): DifficultyAttributes | undefined;
/**
* Advances the iterator to the end to collect all remaining attributes
* into a list and return them.
* @returns {(DifficultyAttributes)[]}
*/
  collect(): (DifficultyAttributes)[];
/**
* Returns the amount of remaining items.
*/
  readonly nRemaining: number;
}
/**
* Gradually calculate performance attributes after each hitresult.
*/
export class GradualPerformance {
  free(): void;
/**
* @param {Difficulty} difficulty
* @param {Beatmap} map
*/
  constructor(difficulty: Difficulty, map: Beatmap);
/**
* Process the next hit object and calculate the performance attributes
* for the resulting score state.
* @param {ScoreState} state
* @returns {PerformanceAttributes | undefined}
*/
  next(state: ScoreState): PerformanceAttributes | undefined;
/**
* Process everything up to the next `n`th hitobject and calculate the
* performance attributes for the resulting score state.
*
* Note that the count is zero-indexed, so `n=0` will process 1 object,
* `n=1` will process 2, and so on.
* @param {ScoreState} state
* @param {number} n
* @returns {PerformanceAttributes | undefined}
*/
  nth(state: ScoreState, n: number): PerformanceAttributes | undefined;
/**
* Returns the amount of remaining items.
*/
  readonly nRemaining: number;
}
/**
* Builder for a performance calculation.
*/
export class Performance {
  free(): void;
/**
* Create a new performance calculator.
* @param {PerformanceArgs | undefined} [args]
*/
  constructor(args?: PerformanceArgs);
/**
* Calculate performance attributes.
*
* If a beatmap is passed as argument, difficulty attributes will have to
* be calculated internally which is a comparably expensive task. Hence,
* passing previously calculated attributes should be prefered whenever
* available.
*
* However, be careful that the passed attributes have been calculated
* for the same difficulty settings like mods, clock rate, beatmap,
* custom ar, ... otherwise the final attributes will be incorrect.
* @param {MapOrAttributes} args
* @returns {PerformanceAttributes}
*/
  calculate(args: MapOrAttributes): PerformanceAttributes;
/**
*/
  accuracy?: number;
/**
*/
  ar?: number;
/**
*/
  arWithMods?: boolean;
/**
*/
  clockRate?: number;
/**
*/
  combo?: number;
/**
*/
  cs?: number;
/**
*/
  csWithMods?: boolean;
/**
*/
  hardrockOffsets?: boolean;
/**
*/
  hitresultPriority?: HitResultPriority;
/**
*/
  hp?: number;
/**
*/
  hpWithMods?: boolean;
/**
*/
  misses?: number;
/**
*/
  mods?: Object;
/**
*/
  n100?: number;
/**
*/
  n300?: number;
/**
*/
  n50?: number;
/**
*/
  nGeki?: number;
/**
*/
  nKatu?: number;
/**
*/
  od?: number;
/**
*/
  odWithMods?: boolean;
/**
*/
  passedObjects?: number;
}
/**
* The result of a performance calculation.
*/
export class PerformanceAttributes {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
/**
* The difficulty attributes.
*/
  readonly difficulty: DifficultyAttributes;
/**
* Scaled miss count based on total hits.
*
* Only available for osu! and osu!taiko.
*/
  readonly effectiveMissCount: number | undefined;
/**
* The final performance points.
*/
  readonly pp: number;
/**
* The accuracy portion of the final pp.
*
* Only available for osu! and osu!taiko.
*/
  readonly ppAccuracy: number | undefined;
/**
* The aim portion of the final pp.
*
* Only available for osu!.
*/
  readonly ppAim: number | undefined;
/**
* The strain portion of the final pp.
*
* Only available for osu!taiko and osu!mania.
*/
  readonly ppDifficulty: number | undefined;
/**
* The flashlight portion of the final pp.
*
* Only available for osu!.
*/
  readonly ppFlashlight: number | undefined;
/**
* The speed portion of the final pp.
*
* Only available for osu!.
*/
  readonly ppSpeed: number | undefined;
/**
* The hitresult score state that was used for performance calculation.
*
* Only available if *not* created through gradual calculation.
*/
  readonly state: ScoreState | undefined;
}
/**
* The result of calculating the strains of a beatmap.
*
* Suitable to plot the difficulty over time.
*/
export class Strains {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
/**
* Strain peaks of the aim skill in osu!.
*/
  readonly aim: Float64Array | undefined;
/**
* Strain peaks of the aim skill without sliders in osu!.
*/
  readonly aimNoSliders: Float64Array | undefined;
/**
* Strain peaks of the color skill in osu!taiko.
*/
  readonly color: Float64Array | undefined;
/**
* Strain peaks of the flashlight skill in osu!.
*/
  readonly flashlight: Float64Array | undefined;
/**
* The strains' gamemode.
*/
  readonly mode: GameMode;
/**
* Strain peaks of the movement skill in osu!catch.
*/
  readonly movement: Float64Array | undefined;
/**
* Strain peaks of the rhythm skill in osu!taiko.
*/
  readonly rhythm: Float64Array | undefined;
/**
* Time inbetween two strains in ms.
*/
  readonly sectionLength: number;
/**
* Strain peaks of the speed skill in osu!.
*/
  readonly speed: Float64Array | undefined;
/**
* Strain peaks of the stamina skill in osu!taiko.
*/
  readonly stamina: Float64Array | undefined;
/**
* Strain peaks of the strain skill in osu!mania.
*/
  readonly strains: Float64Array | undefined;
}
