# rosu-pp-js

Library to calculate difficulty and performance attributes for all [osu!] gamemodes.

This is a js binding to the [Rust] library [rosu-pp] through [Wasm]. As such, its performance is
much faster than a native js library.
Since Wasm is used as intermediate layer, Rust doesn't even need to be installed.

## Usage

The library exposes multiple classes and interfaces:

### [Beatmap](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L270-L335)

Class containing a parsed `.osu` file, ready to be passed to difficulty and performance calculators.

The constructor takes an object of type `Uint8Array | string` representing the content of a `.osu`
file and throws an error if decoding the beatmap fails.

⚠️WARNING⚠️

Due to [current JavaScript oddities](https://github.com/rustwasm/wasm-bindgen/issues/3917), Wasm is not always able to track down objects' lifetime meaning it is possible that memory of unused instances might not get cleared automatically. Hence, to not risk leaking memory, it is recommended to free `Beatmap` instances manually when they're no longer needed with the `free(): void` method.

To convert a beatmap use the `convert(GameMode): void` method.

`Beatmap` provides various getters:
- `ar: number`
- `cs: number`
- `hp: number`
- `isConvert: boolean`
- `mode: GameMode`
- `nBreaks: number`
- `nCircles: number`
- `nHolds: number`
- `nObjects: number`
- `nSliders: number`
- `nSpinners: number`
- `od: number`
- `sliderMultiplier: number`
- `sliderTickRate: number`
- `stackLeniency: number`
- `version: number`

### [Difficulty](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L435-L505)

Class to calculate [`DifficultyAttributes`](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L506-L656), [`Strains`](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L878-L937), or create gradual calculators.

The constructor takes [an *optional* object of the form](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L144-L162)

```ts
{
    // The type must be either
    //   - an integer for bitflags (see https://github.com/ppy/osu-api/wiki#mods)
    //   - a string for acronyms
    //   - a single mod object as described below
    //   - a sequence of types that deserialize into a single mod
    //
    // Types that deserialize into a single mod are
    //   - an integer for bitflags
    //   - a string for an acronym
    //   - a mod object
    //
    // A mod object must have an `acronym: string` property and an optional
    // `settings?: Object` property.
    mods?: Object,
    // Custom clock rate between 0.01 and 100
    clockRate?: number,
    // Custom approach rate between -20 and 20
    ar?: number,
    // Whether given `ar` should be used as is or adjusted based on mods
    // i.e. `true` means "given ar already considers mods".
    arWithMods?: boolean,
    // Custom circle size between -20 and 20
    cs?: number,
    csWithMods?: boolean,
    // Custom drain rate between -20 and 20
    hp?: number,
    hpWithMods?: boolean,
    // Custom overall difficulty between -20 and 20
    od?: number,
    odWithMods?: boolean,
    // Amount of passed objects for partial plays, e.g. a fail
    passedObjects?: number,
    // Adjust patterns as if the HR mod is enabled on osu!catch maps
    hardrockOffsets?: boolean,
}
```

The following methods are available:

- `calculate(Beatmap): DifficultyAttributes`: The difficulty attributes for the given parameters
- `strains(Beatmap): Strains`: The strain values for the given parameters, suitable to plot difficulty over time
- `gradualDifficulty(Beatmap): GradualDifficulty`: A gradual difficulty calculator
- `gradualPerformance(Beatmap): GradualPerformance`: A gradual performance calculator

### [Performance](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L725-L813)

Calculator of [`PerformanceAttributes`](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L814-L877) whose constructor takes [an object of the form](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L164-L215)

```ts
{
    // ... same fields as for `Difficulty` but also:

    // Accuracy between 0 and 100
    accuracy?: number,
    // The max combo of a play
    combo?: number,
    // The amount of gekis (n320 for mania)
    nGeki?: number,
    // The amount of katus (tiny droplet misses for catch, n200 for mania)
    nKatu?: number,
    // The amount of n300s
    n300?: number,
    // The amount of n100s
    n100?: number,
    // The amount of n50s
    n50?: number,
    // The amount of misses
    misses?: number,
    // Whether good or bad hitresults should be generated to fit the given accuracy
    hitresultPriority?: HitResultPriority,
}
```

Its only method `calculate(DifficultyAttributes | PerformanceAttributes | Beatmap): PerformanceAttributes`
produces the performance attributes. The method's argument must be either the attributes of a
previous calculation or a beatmap.

Note that if a beatmap is given, difficulty attributes have to be calculated internally which is
comparably expensive so passing attributes should be prefered whenever possible.

However, be careful that the passed attributes have been calculated for the
same difficulty settings like mods, clock rate, beatmap, custom ar, ...
otherwise the final performance attributes will be incorrect.

### [GradualDifficulty](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L657-L691)

Class to calculate difficulty attributes after each hitobject.

Its constructor takes a `Difficulty` and a `Beatmap`, it has a getter `nRemaining: number`, and the methods

- `next(): DifficultyAttributes | undefined`: Process the next hitobject and return the difficulty attributes (or `undefined` if the last object has already been processed)
- `nth(number): DifficultyAttributes | undefined`: Process the next `number - 1` hitobjects, i.e. `nth(0)` will process one, `nth(1)` will proces two, ...
- `collect(): DifficultyAttributes[]`: Collect all remaining difficulty attributes into a list

### [GradualPerformance](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L692-L724)

Class to calculate performance attributes after each hitresult.

Its constructor takes a `Difficulty` and a `Beatmap`, it has a getter `nRemaining: number`, and the methods

- `next(ScoreState): PerformanceAttributes | undefined`: Process the next hitobject and return the performance attributes (or `undefined` if the last object has already been processed)
- `nth(ScoreState, number): PerformanceAttributes | undefined`: Process the next `number - 1` hitobjects, i.e. `nth(0)` will process one, `nth(1)` will proces two, ...

[`ScoreState`](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L222-L261) is an object like

```ts
{
  maxCombo?: number;
  misses?: number;
  n100?: number;
  n300?: number;
  n50?: number;
  nGeki?: number;
  nKatu?: number;
}
```

### [BeatmapAttributesBuilder](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L378-L431)

Class to calculate [`BeatmapAttributes`](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L336-L377) for various custom parameters.

Its constructor takes [an object of the form](https://github.com/MaxOhn/rosu-pp-js/blob/23dd3325c8824bdce42ef09775b5e50813403631/rosu_pp_js.d.ts#L29-L45)

```ts
{
    // Start off with the given beatmap's attributes, mode, and convert status
    map?: Beatmap,
    // Specify a gamemode
    mode?: GameMode,
    // Whether the map is a convert, only relevant for mania
    isConvert?: boolean,
    mods?: Object,
    clockRate?: number,
    ar?: number,
    arWithMods?: boolean,
    cs?: number,
    csWithMods?: boolean,
    hp?: number,
    hpWithMods?: boolean,
    od?: number,
    odWithMods?: boolean,
}
```

Its only method is `build(): BeatmapAttributes`.

## Setters

For the `Difficulty`, `Performance`, and `BeatmapAttributesBuilder` classes, each field of their constructor's argument object is also available as a setter afterwards which takes either a value of the field's type or `undefined` to unset the previously set value.

```js
import * as rosu from "rosu-pp-js";

// Either given in the constructor
let difficulty = new rosu.Difficulty({ clockRate: 1.23 });

// Or adjusted afterwards
difficulty.mods = 8;

// Or even reset entirely
difficulty.clockRate = undefined;
```

## Examples

### Calculating performance

```js
import * as rosu from "rosu-pp-js";
import * as fs from "fs";
// or if you're using CommonJS:
const rosu = require("rosu-pp-js");
const fs = require("fs");

const bytes = fs.readFileSync("/path/to/file.osu");

// Parse the map.
let map = new rosu.Beatmap(bytes);

// Optionally convert the beatmap to a specific mode.
map.convert(rosu.GameMode.Taiko);

// Calculating performance attributes for a HDDT SS
const maxAttrs = new rosu.Performance({ mods: 8 + 64 }).calculate(map);

// Calculating performance attributes for a specific score.
const currAttrs = new rosu.Performance({
    mods: "HDDT", // Must be the same as before in order to use the previous attributes!
    misses: 2,
    accuracy: 98.4,
    combo: 567,
    hitresultPriority: rosu.HitResultPriority.WorstCase,
}).calculate(maxAttrs); // Re-using previous attributes to speed up the calculation.

console.log(`PP: ${currAttrs.pp}/${maxAttrs.pp} | Stars: ${maxAttrs.difficulty.stars}`);

// Free the beatmap manually to avoid risking memory leakage.
map.free();
```

### Gradual calculation

```js
import * as rosu from "rosu-pp-js";
import * as fs from "fs";

const content = fs.readFileSync("/path/to/file.osu", "utf-8");
const map = new rosu.Beatmap(content);

// Specifying some difficulty parameters
const difficulty = new rosu.Difficulty({
    mods: 16 + 64, // HRDT
    clockRate: 1.1,
    ar: 10.2,
    arWithMods: true,
    od: 4,
    odWithMods: false,
});

// Gradually calculating *difficulty* attributes
let gradualDiff = difficulty.gradualDifficulty(map);
let i = 1;

while (gradualDiff.nRemaining > 0) {
    console.log(`Stars after ${i} hitobjects: ${gradualDiff.next()?.stars}`);
    i += 1;
}

// Gradually calculating *performance* attributes
let gradualPerf = difficulty.gradualPerformance(map);
let j = 1;

while (gradualPerf.nRemaining > 0) {
    // Need to pass the current score state
    const state = {
        maxCombo: j,
        n300: j,
        n100: 0,
        // ...
    };

    console.log(`PP: ${gradualPerf.next(state)?.pp}`);
    j += 1;
}

map.free();
```

## Installing rosu-pp-js

```sh
$ npm install rosu-pp-js
```

or

```sh
$ npm install https://github.com/MaxOhn/rosu-pp-js/releases/download/v1.1.0/rosu_pp_js_nodejs.tar.gz
```

Note that apart from the `*_nodejs` version, the release page also includes `*_web` and `*_bundler` versions.

## Learn More
- [rosu-pp]
- [Rust]
- [Wasm]

[osu!]: https://osu.ppy.sh/home
[Rust]: https://www.rust-lang.org/
[rosu-pp]: https://github.com/MaxOhn/rosu-pp
[Wasm]: https://webassembly.org/