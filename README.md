# rosu-pp-js

Library to calculate difficulty and performance attributes for all [osu!] gamemodes.

This is a js binding to the [Rust] library [rosu-pp] through [Wasm]. As such, its performance is
much faster than a native js library.
Since Wasm is used as intermediate layer, Rust doesn't even need to be installed.

## Usage

The library exposes multiple classes and interfaces:

### Beatmap

Class containing a parsed `.osu` file, ready to be passed to difficulty and performance calculators.

The constructor takes an object of the form

```ts
{
    // The bytes of a `.osu` file's content.
    bytes?: Uint8Array,
    // The content of a `.osu` file.
    content?: string,
    // The mode to convert the beatmap to.
    mode?: GameMode,
}
```

and throws an error if
- neither `bytes` nor `content` is specified
- decoding the beatmap failed
- the beatmap's mode cannot be converted to the specified mode

To convert a beatmap after initialization, use the `convert(GameMode): void` method.

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

### Difficulty

Class to calculate difficulty attributes, strains, or create gradual calculators.

The constructor takes an *optional* object of the form

```ts
{
    // Mod bitflags, see <https://github.com/ppy/osu-api/wiki#mods>
    mods?: number,
    // Custom clock rate between 0.01 and 100
    clockRate?: number,
    // Custom approach rate between -20 and 20
    ar?: number,
    // Whether given `ar` should be used as is or adjusted based on mods
    // I.e. `true` means "given ar is already with mods".
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

### Performance

Performance calculator whose constructor takes an object of the form

```ts
{
    // ... same fields as for `Difficulty` but also:

    // A beatmap to calculate the performance on.
    map?: Beatmap,
    // A previous result of difficulty or performances calculation *for the same difficulty parameters*
    // This should be prefered over `map` whenever possible.
    attributes?: DifficultyAttributes,
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

and throws an error if neither `map` nor `attributes` is specified.
Note that if `attributes` is not specified, they'll have to be calculated which is the most
expensive part of the whole calculation. However, be careful that the given attributes were
calculated for the same difficulty parameters e.g. same map, clock rate,
passed object count, custom ar, ...

Its only method is `calculate(): PerformanceAttributes`.

### GradualDifficulty

Class to calculate difficulty attributes after each hitobject.

Its constructor takes a `Difficulty` and a `Beatmap`, it has a getter `nRemaining`, and the methods

- `next(): DifficultyAttributes | undefined`: Process the next hitobject and return the difficulty attributes (or `undefined` if the last object has already been processed)
- `nth(number): DifficultyAttributes | undefined`: Process the next `number - 1` hitobjects, i.e. `nth(0)` will process one, `nth(1)` will proces two, ...
- `collect(): DifficultyAttributes[]`: Collect all remaining difficulty attributes into a list

### GradualPerformance

Class to calculate performance attributes after each hitresult.

Its constructor takes a `Difficulty` and a `Beatmap`, it has a getter `nRemaining`, and the methods

- `next(ScoreState): PerformanceAttributes | undefined`: Process the next hitobject and return the performance attributes (or `undefined` if the last object has already been processed)
- `nth(ScoreState, number): PerformanceAttributes | undefined`: Process the next `number - 1` hitobjects, i.e. `nth(0)` will process one, `nth(1)` will proces two, ...

`ScoreState` is an object of the form

```ts
{
    maxCombo?: number,
    nGeki?: number,
    nKatu?: number,
    n300?: number,
    n100?: number,
    n50?: number,
    misses?: number,
}
```

### BeatmapAttributesBuilder

Class to calculate beatmap attributes for various custom parameters.

Its constructor takes an object of the form

```ts
{
    // Start off with the given beatmap's attributes
    map?: Beatmap,
    // Specify a gamemode
    mode?: GameMode,
    // Whether the map is a convert, only relevant for mania
    isConvert?: boolean,

    // same fields as for `Difficulty` ...
    mods?: number,
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

## Example

### Calculating performance

```js
import * as rosu from 'rosu-pp-js';
import * as fs from 'fs';

const bytes = fs.readFileSync("/path/to/file.osu");

// Parse the map and convert it to osu!taiko if it's not already a taiko map.
// Note that this will throw an error if it's an osu!catch or osu!mania map.
const map = new rosu.Beatmap({ bytes, mode: rosu.GameMode.Taiko });

// Calculating performance attributes for a HDDT SS
const maxAttrs = new rosu.Performance({ map, mods: 8 + 64 }).calculate();

// Calculating performance attributes for a specific score.
// Note we're re-using previous attributes to speed up the calculation.
const currAttrs = new rosu.Performance({
    attributes: maxAttrs,
    // Must be the same as before in order to use the previous attributes!
    mods: 8 + 64,
    misses: 2,
    accuracy: 98.4,
    combo: 567,
    hitresultPriority: rosu.HitResultPriority.WorstCase,
}).calculate();

console.log(`PP: ${currAttrs.pp}/${maxAttrs.pp} | Stars: ${maxAttrs.stars}`);
```

### Gradual calculation

```js
import * as rosu from 'rosu-pp-js';
import * as fs from 'fs';

const content = fs.readFileSync("/path/to/file.osu", "utf-8");
const map = new rosu.Beatmap({ content });

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
    console.log(`Stars after ${i} hitobjects: ${gradualDiff.next().stars}`);
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

    console.log(`PP: ${gradualPerf.next(state).pp}`);
    j += 1;
}
```

## Installing rosu-pp-js

```sh
$ npm install rosu-pp-js
```

or

```sh
$ npm install [TODO: url to github release]
```

## Learn More
- [rosu-pp]
- [Rust]
- [Wasm]

[osu!]: https://osu.ppy.sh/home
[Rust]: https://www.rust-lang.org/
[rosu-pp]: https://github.com/MaxOhn/rosu-pp
[Wasm]: https://webassembly.org/