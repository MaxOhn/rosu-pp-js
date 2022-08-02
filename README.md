# rosu-pp-js

Difficulty and performance calculation for all [osu!](https://osu.ppy.sh/) modes.

This is a js binding to the Rust library [rosu-pp](https://github.com/MaxOhn/rosu-pp) which was bootstrapped through [neon](https://www.npmjs.com/package/create-neon).
Since all the heavy lifting is done by Rust, rosu-pp-js comes with a very fast performance.
Check out rosu-pp's README for more info.

## How to use rosu-pp-js

The library has a very simple interface, namely only two function: `calculate` and `strains`. The `calculate` function takes one argument which can have two forms:
- calculate only one score on a map:
```js
{
    path: "/path/to/file.osu", // the only mandatory field, everything else can be omitted
    mode: integer or string,   // convert the map to a specific mode; accepts 0/1/2/3 or "o"/"t"/"c"/"m"/various variations
    mods: integer,             // bit value for mods, defaults to 0 (NM) see https://github.com/ppy/osu-api/wiki#mods
    acc: number,               // if neither acc nor hitresults are specified, acc defaults to 100.0
    n300: integer,             // defaults to value based on acc
    n100: integer,             // defaults to value based on acc
    n50: integer,              // defaults to value based on acc
    nMisses: integer,          // defaults to 0
    nKatu: integer,            // only relevant for osu!ctb
    combo: integer,            // defaults to full combo
    score: integer,            // only relevant for osu!mania
    passedObjects: integer,    // only consider this many hit objects; useful for failed scores; defaults to all objects
    clockRate: number,         // defaults to value based on mods i.e. 1.5 for DT, 0.75 for HT, 1.0 for NM
    ar: number,                // defaults to beatmap's value
    cs: number,                // defaults to beatmap's value
    hp: number,                // defaults to beatmap's value
    od: number,                // defaults to beatmap's value
}
```
- calculate multiple scores on a map:
```js
{
    path: "/path/to/file.osu",
    params: [
        { // everything in here is optional
            mode: integer or string,
            mods: integer,
            acc: number,
            n300: integer,
            n100: integer,
            n50: integer,
            nMisses: integer,
            nKatu: integer,
            combo: integer,
            score: integer,
            passedObjects: integer,
            clockRate: number,
            ar: number,
            cs: number,
            hp: number,
            od: number,
        }, ...
    ]
}
```

## Example

```js
const rosu = require('rosu-pp')

let arg = {
    path: "./maps/1980365.osu",
    params: [
        {
            mods: 8 + 16, // HDHR
        }, // everything else is put on default i.e. the best possible score on HDHR
        {
            mods: 24,
            acc: 97.89,
            nMisses: 13,
            combo: 1388,
        }
    ]
}

let results = rosu.calculate(arg)
```

## Return object structure

The `calculate` function will provide you a **list of objects**, one for each score you specified parameters for. The exact structure of the contained objects depends on the mode of the map.

In the following code block, fields will be denoted with O/T/C/M for osu!standard, taiko, catch the beat, and mania. If a field is denoted with a mode, that field is guaranteed to be included in the object for maps of that mode, otherwise it is omitted.

```js
{
    mode: integer,            // O/T/C/M (0=O, 1=T, 2=C, 3=M)
    stars: number,            // O/T/C/M
    pp: number,               // O/T/C/M
    ppAcc: number,            // O/T/M
    ppAim: number,            // O
    ppFlashlight: number,     // O
    ppSpeed: number,          // O
    ppStrain: number,         // T/M
    nFruits: integer,         // C
    nDroplets: integer,       // C
    nTinyDroplets: integer,   // C
    aimStrain: number,        // O
    speedStrain: number,      // O
    flashlightRating: number, // O
    sliderFactor: number,     // O
    ar: number,               // O/T/C/M
    cs: number,               // O/T/C/M
    hp: number,               // O/T/C/M
    od: number,               // O/T/C/M
    bpm: number,              // O/T/C/M
    clockRate: number,        // O/T/C/M
    timePreempt: number,      // O
    greatHitWindow: number,   // O/T/M
    nCircles: integer,        // O/T/M
    nSliders: integer,        // O/T/M
    nSpinners: integer,       // O/T/C
    maxCombo: integer,        // O/T/C
}
```

## Calculating strains

If you want to plot the difficulty of a map over time, you can calculate the strain values through the `strains` function.
This function requires at least one argument, namely the path to a `.osu` file,
and an optional second argument, namely the mods.

The returned object's attributes depend on the map's game mode again and look as follows:
```js
{
    mode: integer,               // O/T/C/M (0=O, 1=T, 2=C, 3=M)
    sectionLength: number,       // O/T/C/M
    aim: Array<number>,          // O
    aimNoSliders: Array<number>, // O
    speed: Array<number>,        // O
    flashlight: Array<number>,   // O
    color: Array<number>,        // T
    rhythm: Array<number>,       // T
    staminaLeft: Array<number>,  // T
    staminaRight: Array<number>, // T
    strains: Array<number>,      // M
    movement: Array<number>,     // C
}
```
`sectionLength` is the amount of milliseconds between two strain values
and all other fields are lists with one strain value per section.

Here's a small example
```js
const rosu = require('rosu-pp')

let strains = rosu.strains("./maps/1980365.osu", 8 + 16); // HDHR
let highest_aim_strain = Math.max(...strains.aim);
```

## Installing rosu-pp-js

Installing rosu-pp-js requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

Once [node](https://nodejs.org) and [Rust](https://www.rust-lang.org/learn/get-started) are ready to go, you can install the project with npm. In your project directory, run:

```sh
$ npm install rosu-pp
```

or

```sh
$ npm install https://github.com/MaxOhn/rosu-pp-js
```

This fully installs the project, including installing any dependencies and running the build.

## Learn More
- [rosu-pp documentation](https://docs.rs/rosu-pp)
- [Rust documentation](https://www.rust-lang.org).
- [Neon documentation](https://neon-bindings.com).
- [Node documentation](https://nodejs.org).
