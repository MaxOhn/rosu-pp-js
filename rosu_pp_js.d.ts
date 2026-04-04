/* tslint:disable */
/* eslint-disable */
export enum GameMode {
  Osu = 0,
  Taiko = 1,
  Catch = 2,
  Mania = 3,
}
/**
 * A specific implementation of hitresult generation.
 */
export enum HitResultGenerator {
  /**
   * Prioritize generating hitresults quickly.
   */
  Fast = 0,
  /**
   * Find the hitresults that match the given accuracy the closest.
   */
  Closest = 1,
}
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
   * "Large tick" hits for osu!standard.
   *
   * The meaning depends on the kind of score:
   * - if set on osu!stable, this field is irrelevant and can be `0`
   * - if set on osu!lazer *without* `CL`, this field is the amount of hit
   *   slider ticks and repeats
   * - if set on osu!lazer *with* `CL`, this field is the amount of hit
   *   slider heads, ticks, and repeats
   */
  osuLargeTickHits?: number;

  /**
   * "Small tick" hits for osu!standard.
   *
   * These are essentially the slider end hits for lazer scores without
   * slider accuracy.
   *
   * Only relevant for osu!lazer.
   */
  osuSmallTickHits?: number;

  /**
   * Amount of successfully hit slider ends.
   *
   * Only relevant for osu!standard in lazer.
   */
  sliderEndHits?: number;

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
  /**
   * Legacy total score.
   *
   * Only relevant for osu!stable
   */
  legacyTotalScore?: number | null;
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
  passedObjects?: number | null;
  /**
   * Adjust patterns as if the HR mod is enabled.
   *
   * Only relevant for osu!catch.
   */
  hardrockOffsets?: boolean | null;
  /**
   * Whether the calculated attributes belong to an osu!lazer or osu!stable
   * score.
   *
   * Defaults to `true`.
   */
  lazer?: boolean | null;
}

/**
 * Either previously calculated attributes or a beatmap.
 */
export type MapOrAttributes =
  | DifficultyAttributes
  | PerformanceAttributes
  | Beatmap;

/**
 * Arguments to provide the `Performance` constructor.
 */
export interface PerformanceArgs extends DifficultyArgs {
  /** Set the accuracy between `0.0` and `100.0`. */
  accuracy?: number | null;
  /**
   * Specify the max combo of the play.
   *
   * Irrelevant for osu!mania.
   */
  combo?: number | null;
  /**
   * The amount of "large tick" hits.
   *
   * Only relevant for osu!.
   *
   * The meaning depends on the kind of score:
   * - if set on osu!stable, this value is irrelevant and can be `0`
   * - if set on osu!lazer *without* `CL`, this value is the amount of hit
   *   slider ticks and repeats
   * - if set on osu!lazer *with* `CL`, this value is the amount of hit
   *   slider heads, ticks, and repeats
   */
  largeTickHits?: number | null;
  /**
   * The amount of "small tick" hits.
   *
   * These are essentially the slider end hits for lazer scores without
   * slider accuracy.
   *
   * Only relevant for osu!.
   */
  smallTickHits?: number | null;
  /**
   * The amount of slider end hits.
   *
   * Only relevant for osu! in lazer.
   */
  sliderEndHits?: number | null;
  /**
   * Specify the amount of gekis of a play.
   *
   * Only relevant for osu!mania for which it repesents the amount of n320.
   */
  nGeki?: number | null;
  /**
   * Specify the amount of katus of a play.
   *
   * Only relevant for osu!catch for which it represents the amount of tiny
   * droplet misses and osu!mania for which it repesents the amount of n200.
   */
  nKatu?: number | null;
  /** Specify the amount of 300s of a play. */
  n300?: number | null;
  /** Specify the amount of 100s of a play. */
  n100?: number | null;
  /**
   * Specify the amount of 50s of a play.
   *
   * Irrelevant for osu!taiko.
   */
  n50?: number | null;
  /** Specify the amount of misses of a play. */
  misses?: number | null;
  /**
   * Specify the legacy total score.
   *
   * Only relevant for osu!.
   */
  legacyTotalScore?: number | null;
  /**
   * Specify how hitresults should be generated.
   *
   * Defaults to `HitResultPriority.BestCase`.
   */
  hitresultPriority?: HitResultPriority;
  /** Four optional generators; one for each mode. */
  hitresultGenerators?: Array<HitResultGenerator | null> | null;
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
  clockRate?: number | null;
  /**
   * Override a beatmap's approach rate.
   *
   * | Minimum | Maximum |
   * | :-----: | :-----: |
   * | -20     | 20      |
   */
  ar?: number | null;
  /**
   * Determines if the given AR value should be used before or after accounting
   * for mods, e.g. on `true` the value will be used as is and on `false` it
   * will be modified based on the mods.
   */
  fixedAr?: boolean;
  /**
   * Override a beatmap's circle size.
   *
   * | Minimum | Maximum |
   * | :-----: | :-----: |
   * | -20     | 20      |
   */
  cs?: number | null;
  /**
   * Determines if the given CS value should be used before or after accounting
   * for mods, e.g. on `true` the value will be used as is and on `false` it
   * will be modified based on the mods.
   */
  fixedCs?: boolean;
  /**
   * Override a beatmap's drain rate.
   *
   * | Minimum | Maximum |
   * | :-----: | :-----: |
   * | -20     | 20      |
   */
  hp?: number | null;
  /**
   * Determines if the given HP value should be used before or after accounting
   * for mods, e.g. on `true` the value will be used as is and on `false` it
   * will be modified based on the mods.
   */
  fixedHp?: boolean;
  /**
   * Override a beatmap's overall difficulty.
   *
   * | Minimum | Maximum |
   * | :-----: | :-----: |
   * | -20     | 20      |
   */
  od?: number | null;
  /**
   * Determines if the given OD value should be used before or after accounting
   * for mods, e.g. on `true` the value will be used as is and on `false` it
   * will be modified based on the mods.
   */
  fixedOd?: boolean;
}

/**
 * The content of a `.osu` file either as bytes or string.
 */
export type BeatmapContent = Uint8Array | string;

/**
 * Arguments to provide the `BeatmapAttributesBuilder` constructor.
 */
export interface BeatmapAttributesArgs extends CommonArgs {
  /** Specify a gamemode. */
  mode?: GameMode | null;
  /** Specify whether it's a converted map. */
  isConvert?: boolean;
  /** Start off with a beatmap's attributes, mode, and convert status. */
  map?: Beatmap | null;
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
   * Check whether hitobjects appear too suspicious for further calculation.
   *
   * Sometimes a beatmap isn't created for gameplay but rather to test
   * the limits of osu! itself. Difficulty- and/or performance calculation
   * should likely be avoided on these maps due to potential performance
   * issues.
   */
  isSuspicious(): boolean;
  /**
   * Create a new beatmap instance by parsing an `.osu` file's content.
   * @throws Throws an error if decoding the map failed
   */
  constructor(args: BeatmapContent);
  /**
   * Convert a beatmap to a specific mode.
   * @throws Throws an error if conversion fails or mods are invalid
   */
  convert(mode: GameMode, mods?: Object | null): void;
  readonly isConvert: boolean;
  readonly nSpinners: number;
  readonly stackLeniency: number;
  readonly sliderTickRate: number;
  readonly sliderMultiplier: number;
  readonly ar: number;
  readonly cs: number;
  readonly hp: number;
  readonly od: number;
  readonly bpm: number;
  readonly mode: GameMode;
  readonly nHolds: number;
  readonly version: number;
  readonly nBreaks: number;
  readonly nCircles: number;
  readonly nObjects: number;
  readonly nSliders: number;
}
export class BeatmapAttributes {
  private constructor();
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
   * The base approach rate without considering clock rate.
   */
  readonly baseAr: number;
  /**
   * The overall difficulty.
   */
  readonly od: number;
  /**
   * The base overall difficulty without considering clock rate.
   */
  readonly baseOd: number;
  /**
   * The circle size.
   */
  readonly cs: number;
  /**
   * The health drain rate
   */
  readonly hp: number;
  /**
   * The clock rate with respect to mods.
   */
  readonly clockRate: number;
  /**
   * Hit window for approach rate i.e. TimePreempt in milliseconds.
   *
   * Only available for osu!standard and osu!catch.
   */
  readonly arHitWindow: number | undefined;
  /**
   * Perfect hit window for overall difficulty i.e. time to hit "Perfect" in
   * milliseconds.
   *
   * Only available for osu!mania.
   */
  readonly odPerfectHitWindow: number | undefined;
  /**
   * Great hit window for overall difficulty i.e. time to hit a 300 ("Great")
   * in milliseconds.
   *
   * Only available for osu!standard, osu!taiko, and osu!mania.
   */
  readonly odGreatHitWindow: number | undefined;
  /**
   * Good hit window for overall difficulty i.e. time to hit a 200 ("Good")
   * in milliseconds.
   *
   * Only available for osu!mania.
   */
  readonly odGoodHitWindow: number | undefined;
  /**
   * Ok hit window for overall difficulty i.e. time to hit a 100 ("Ok") in
   * milliseconds.
   *
   * Only available for osu!standard, osu!taiko, and osu!mania.
   */
  readonly odOkHitWindow: number | undefined;
  /**
   * Meh hit window for overall difficulty i.e. time to hit a 50 ("Meh") in
   * milliseconds.
   *
   * Only available for osu!standard and osu!mania.
   */
  readonly odMehHitWindow: number | undefined;
}
export class BeatmapAttributesBuilder {
  free(): void;
  /**
   * Create a new `BeatmapAttributesBuilder`.
   */
  constructor(args?: BeatmapAttributesArgs | null);
  /**
   * Calculate the `BeatmapAttributes`.
   */
  build(): BeatmapAttributes;
  set fixed_ar(value: boolean | null | undefined);
  set fixed_cs(value: boolean | null | undefined);
  set fixed_hp(value: boolean | null | undefined);
  set fixed_od(value: boolean | null | undefined);
  set clockRate(value: number | null | undefined);
  set isConvert(value: boolean | null | undefined);
  set ar(value: number | null | undefined);
  set cs(value: number | null | undefined);
  set hp(value: number | null | undefined);
  set od(value: number | null | undefined);
  set map(value: Beatmap | null | undefined);
  set mode(value: GameMode | null | undefined);
  set mods(value: Object | null | undefined);
}
/**
 * Builder for a difficulty calculation.
 */
export class Difficulty {
  free(): void;
  /**
   * Returns a gradual difficulty calculator for the current difficulty settings.
   */
  gradualDifficulty(map: Beatmap): GradualDifficulty;
  /**
   * Returns a gradual performance calculator for the current difficulty settings.
   */
  gradualPerformance(map: Beatmap): GradualPerformance;
  /**
   * Create a new difficulty calculator.
   */
  constructor(args?: DifficultyArgs | null);
  /**
   * Perform the difficulty calculation but instead of evaluating strain
   * values, return them as is.
   *
   * Suitable to plot the difficulty over time.
   */
  strains(map: Beatmap): Strains;
  /**
   * Perform the difficulty calculation.
   */
  calculate(map: Beatmap): DifficultyAttributes;
  set fixed_ar(value: boolean | null | undefined);
  set fixed_cs(value: boolean | null | undefined);
  set fixed_hp(value: boolean | null | undefined);
  set fixed_od(value: boolean | null | undefined);
  set clockRate(value: number | null | undefined);
  set passedObjects(value: number | null | undefined);
  set hardrockOffsets(value: boolean | null | undefined);
  set ar(value: number | null | undefined);
  set cs(value: number | null | undefined);
  set hp(value: number | null | undefined);
  set od(value: number | null | undefined);
  set mods(value: Object | null | undefined);
  set lazer(value: boolean | null | undefined);
}
/**
 * The result of a difficulty calculation.
 */
export class DifficultyAttributes {
  private constructor();
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
   * The attributes' gamemode.
   */
  readonly mode: GameMode;
  /**
   * The final star rating.
   */
  readonly stars: number;
  /**
   * Whether the map was a convert i.e. an osu! map.
   */
  readonly isConvert: boolean;
  /**
   * The difficulty of the aim skill.
   *
   * Only available for osu!.
   */
  readonly aim: number | undefined;
  /**
   * The number of sliders weighted by difficulty.
   *
   * Only available for osu!.
   */
  readonly aimDifficultSliderCount: number | undefined;
  /**
   * The difficulty of the speed skill.
   *
   * Only available for osu!.
   */
  readonly speed: number | undefined;
  /**
   * The difficulty of the flashlight skill.
   *
   * Only available for osu!.
   */
  readonly flashlight: number | undefined;
  /**
   * The ratio of the aim strain with and without considering sliders
   *
   * Only available for osu!.
   */
  readonly sliderFactor: number | undefined;
  /**
   * Describes how much of aim's difficult strain count is contributed to by sliders
   *
   * Only available for osu!.
   */
  readonly aimTopWeightedSliderFactor: number | undefined;
  /**
   * Describes how much of speed's difficult strain count is contributed to by sliders
   *
   * Only available for osu!.
   */
  readonly speedTopWeightedSliderFactor: number | undefined;
  /**
   * The number of clickable objects weighted by difficulty.
   *
   * Only available for osu!.
   */
  readonly speedNoteCount: number | undefined;
  /**
   * Weighted sum of aim strains.
   *
   * Only available for osu!.
   */
  readonly aimDifficultStrainCount: number | undefined;
  /**
   * Weighted sum of speed strains.
   *
   * Only available for osu!.
   */
  readonly speedDifficultStrainCount: number | undefined;
  /**
   * The amount of nested score per object.
   *
   * Only available for osu!.
   */
  readonly nestedScorePerObject: number | undefined;
  /**
   * The legacy score base multiplier.
   *
   * Only available for osu!.
   */
  readonly legacyScoreBaseMultiplier: number | undefined;
  /**
   * The maximum legacy combo score.
   *
   * Only available for osu!.
   */
  readonly maximumLegacyComboScore: number | undefined;
  /**
   * The health drain rate.
   *
   * Only available for osu!.
   */
  readonly hp: number | undefined;
  /**
   * The amount of circles.
   *
   * Only available for osu!.
   */
  readonly nCircles: number | undefined;
  /**
   * The amount of sliders.
   *
   * Only available for osu!.
   */
  readonly nSliders: number | undefined;
  /**
   * The amount of "large ticks".
   *
   * The meaning depends on the kind of score:
   * - if set on osu!stable, this value is irrelevant
   * - if set on osu!lazer *with* slider accuracy, this value is the amount
   *   of hit slider ticks and repeats
   * - if set on osu!lazer *without* slider accuracy, this value is the
   *   amount of hit slider heads, ticks, and repeats
   *
   * Only available for osu!.
   */
  readonly nLargeTicks: number | undefined;
  /**
   * The amount of spinners.
   *
   * Only available for osu!.
   */
  readonly nSpinners: number | undefined;
  /**
   * The difficulty of the stamina skill.
   *
   * Only available for osu!taiko.
   */
  readonly stamina: number | undefined;
  /**
   * The difficulty of the rhythm skill.
   *
   * Only available for osu!taiko.
   */
  readonly rhythm: number | undefined;
  /**
   * The difficulty of the color skill.
   *
   * Only available for osu!taiko.
   */
  readonly color: number | undefined;
  /**
   * The difficulty of the reading skill.
   *
   * Only available for osu!taiko.
   */
  readonly reading: number | undefined;
  /**
   * The amount of fruits.
   *
   * Only available for osu!catch.
   */
  readonly nFruits: number | undefined;
  /**
   * The amount of droplets.
   *
   * Only available for osu!catch.
   */
  readonly nDroplets: number | undefined;
  /**
   * The amount of tiny droplets.
   *
   * Only available for osu!catch.
   */
  readonly nTinyDroplets: number | undefined;
  /**
   * The amount of hitobjects in the map.
   *
   * Only available for osu!mania.
   */
  readonly nObjects: number | undefined;
  /**
   * The amount of hold notes in the map.
   *
   * Only available for osu!mania.
   */
  readonly nHoldNotes: number | undefined;
  /**
   * The approach rate.
   *
   * Only available for osu!.
   */
  readonly ar: number | undefined;
  /**
   * Time preempt (AR time window).
   *
   * Only available for osu!catch.
   */
  readonly preempt: number | undefined;
  /**
   * The perceived hit window for an n300 inclusive of rate-adjusting mods
   * (DT/HT/etc)
   *
   * Only available for osu! and osu!taiko.
   */
  readonly greatHitWindow: number | undefined;
  /**
   * The perceived hit window for an n100 inclusive of rate-adjusting mods
   * (DT/HT/etc)
   *
   * Only available for osu! and osu!taiko.
   */
  readonly okHitWindow: number | undefined;
  /**
   * The perceived hit window for an n50 inclusive of rate-adjusting mods
   * (DT/HT/etc)
   *
   * Only available for osu!.
   */
  readonly mehHitWindow: number | undefined;
  /**
   * The ratio of stamina difficulty from mono-color (single color) streams to total
   * stamina difficulty.
   *
   * Only available for osu!taiko.
   */
  readonly monoStaminaFactor: number | undefined;
  /**
   * The difficulty corresponding to the mechanical skills.
   *
   * This includes colour and stamina combined.
   *
   * Only available for osu!taiko.
   */
  readonly mechanicalDifficulty: number | undefined;
  /**
   * The factor corresponding to the consistency of a map.
   *
   * Only available for osu!taiko.
   */
  readonly consistencyFactor: number | undefined;
  /**
   * Return the maximum combo.
   */
  readonly maxCombo: number;
}
/**
 * Gradually calculate difficulty attributes after each hitobject.
 */
export class GradualDifficulty {
  free(): void;
  constructor(difficulty: Difficulty, map: Beatmap);
  /**
   * Returns the `n`th attributes of the iterator.
   *
   * Note that the count starts from zero, so `nth(0)` returns the first
   * value, `nth(1)` the second, and so on.
   */
  nth(n: number): DifficultyAttributes | undefined;
  /**
   * Advances the iterator and returns the next attributes.
   */
  next(): DifficultyAttributes | undefined;
  /**
   * Advances the iterator to the end to collect all remaining attributes
   * into a list and return them.
   */
  collect(): DifficultyAttributes[];
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
  constructor(difficulty: Difficulty, map: Beatmap);
  /**
   * Process everything up to the next `n`th hitobject and calculate the
   * performance attributes for the resulting score state.
   *
   * Note that the count is zero-indexed, so `n=0` will process 1 object,
   * `n=1` will process 2, and so on.
   */
  nth(state: ScoreState, n: number): PerformanceAttributes | undefined;
  /**
   * Process the next hit object and calculate the performance attributes
   * for the resulting score state.
   */
  next(state: ScoreState): PerformanceAttributes | undefined;
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
  setHitresultGenerator(
    hitresult_generator?: HitResultGenerator | null,
    mode?: GameMode | null,
  ): void;
  /**
   * Create a new performance calculator.
   */
  constructor(args?: PerformanceArgs | null);
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
   */
  calculate(args: MapOrAttributes): PerformanceAttributes;
  set misses(value: number | null | undefined);
  set nGeki(value: number | null | undefined);
  set nKatu(value: number | null | undefined);
  set accuracy(value: number | null | undefined);
  set fixed_ar(value: boolean | null | undefined);
  set fixed_cs(value: boolean | null | undefined);
  set fixed_hp(value: boolean | null | undefined);
  set fixed_od(value: boolean | null | undefined);
  set clockRate(value: number | null | undefined);
  set passedObjects(value: number | null | undefined);
  set sliderEndHits(value: number | null | undefined);
  set hardrockOffsets(value: boolean | null | undefined);
  set largeTickHits(value: number | null | undefined);
  set smallTickHits(value: number | null | undefined);
  set hitresultPriority(value: HitResultPriority | null | undefined);
  set legacy_total_score(value: number | null | undefined);
  set ar(value: number | null | undefined);
  set cs(value: number | null | undefined);
  set hp(value: number | null | undefined);
  set od(value: number | null | undefined);
  set n50(value: number | null | undefined);
  set mods(value: Object | null | undefined);
  set n100(value: number | null | undefined);
  set n300(value: number | null | undefined);
  set combo(value: number | null | undefined);
  set lazer(value: boolean | null | undefined);
}
/**
 * The result of a performance calculation.
 */
export class PerformanceAttributes {
  private constructor();
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
   * The hitresult score state that was used for performance calculation.
   *
   * Only available if *not* created through gradual calculation.
   */
  readonly state: ScoreState | undefined;
  /**
   * The final performance points.
   */
  readonly pp: number;
  /**
   * The aim portion of the final pp.
   *
   * Only available for osu!.
   */
  readonly ppAim: number | undefined;
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
   * The accuracy portion of the final pp.
   *
   * Only available for osu! and osu!taiko.
   */
  readonly ppAccuracy: number | undefined;
  /**
   * Scaled miss count based on total hits.
   *
   * Only available for osu!.
   */
  readonly effectiveMissCount: number | undefined;
  /**
   * Upper bound on the player's tap deviation.
   *
   * Only *optionally* available for osu!taiko.
   */
  readonly estimatedUnstableRate: number | undefined;
  /**
   * Approximated unstable-rate
   *
   * Only *optionally* available for osu!.
   */
  readonly speedDeviation: number | undefined;
  readonly comboBasedEstimatedMissCount: number | undefined;
  readonly scoreBasedEstimatedMissCount: number | undefined;
  readonly aimEstimatedSliderBreaks: number | undefined;
  readonly speedEstimatedSliderBreaks: number | undefined;
  /**
   * The strain portion of the final pp.
   *
   * Only available for osu!taiko and osu!mania.
   */
  readonly ppDifficulty: number | undefined;
}
/**
 * The result of calculating the strains of a beatmap.
 *
 * Suitable to plot the difficulty over time.
 */
export class Strains {
  private constructor();
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
   * The strains' gamemode.
   */
  readonly mode: GameMode;
  /**
   * Time inbetween two strains in ms.
   */
  readonly sectionLength: number;
  /**
   * Strain peaks of the aim skill in osu!.
   */
  readonly aim: Float64Array | undefined;
  /**
   * Strain peaks of the aim skill without sliders in osu!.
   */
  readonly aimNoSliders: Float64Array | undefined;
  /**
   * Strain peaks of the speed skill in osu!.
   */
  readonly speed: Float64Array | undefined;
  /**
   * Strain peaks of the flashlight skill in osu!.
   */
  readonly flashlight: Float64Array | undefined;
  /**
   * Strain peaks of the color skill in osu!taiko.
   */
  readonly color: Float64Array | undefined;
  /**
   * Strain peaks of the reading skill in osu!taiko.
   */
  readonly reading: Float64Array | undefined;
  /**
   * Strain peaks of the rhythm skill in osu!taiko.
   */
  readonly rhythm: Float64Array | undefined;
  /**
   * Strain peaks of the stamina skill in osu!taiko.
   */
  readonly stamina: Float64Array | undefined;
  /**
   * Strain peaks of the single color stamina skill in osu!taiko.
   */
  readonly singleColorStamina: Float64Array | undefined;
  /**
   * Strain peaks of the movement skill in osu!catch.
   */
  readonly movement: Float64Array | undefined;
  /**
   * Strain peaks of the strain skill in osu!mania.
   */
  readonly strains: Float64Array | undefined;
}
