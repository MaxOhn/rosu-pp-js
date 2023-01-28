# rosu-pp-js

Difficulty and performance calculation for all [osu!](https://osu.ppy.sh/) modes.

This is a js binding to the Rust library [rosu-pp](https://github.com/MaxOhn/rosu-pp) which was bootstrapped through [neon](https://www.npmjs.com/package/create-neon).
Since all the heavy lifting is done by Rust, rosu-pp-js comes with a very fast performance.
Check out rosu-pp's [README](https://github.com/MaxOhn/rosu-pp/blob/main/README.md) for more info.

## How to use rosu-pp-js

The library exposes two classes: `Beatmap` and `Calculator`.

### `Beatmap`

The constructor takes an _optional_ object of the following form:

```js
{
    path?: string,                 // path to a .osu file
    content?: string | Uint8Array, // content of a .osu file
    bytes?: Uint8Array,            // content of a .osu file
    ar?: number, // modify the approach rate
    cs?: number, // modify the circle size
    hp?: number, // modify the drain rate
    od?: number, // modify the overall difficulty
}
```

Additionally, it can be modified through the following builder methods:

- `fromPath(path: string)`
- `fromContent(content: string | Uint8Array)`
- `fromBytes(bytes: Uint8Array)`
- `ar(ar: number)`
- `cs(cs: number)`
- `hp(hp: number)`
- `od(od: number)`

#### Example

```js
import { Beatmap } from require('rosu-pp');

let params = {
    path: './maps/100.osu',
    od: 10.0,
};

let map = new Beatmap(params).ar(9);

require('fs').readFile('./maps/100.osu', (_, data) => {
    let map = new Beatmap().fromBytes(data);
    map = map.cs(0);
});
```

### `Calculator`

The constructor takes an _optional_ object of the following form:

```js
{
    mode?: number | string, // 0/1/2/3 or "osu"/"taiko"/"catch"/"mania"
    mods?: number,          // bit value for mods, see https://github.com/ppy/osu-api/wiki#mods
    acc?: number,           // accuracy between 0 and 100
    nGeki?: number,         // amount of n320 for mania, otherwise irrelevant
    nKatu?: number,         // amount of n200 for mania and tiny droplet misses for catch, otherwise irrelevant
    n300?: number,
    n100?: number,
    n50?: number,           // irrelevant for taiko
    nMisses?: number,
    combo?: number,
    passedObjects?: number, // only consider this many objects, handy for partial plays like fails
    clockRate?: number,     // custom clock rate
}
```

Additionally, it can be modified through the following builder methods:

- `mode(mode: number | string)`
- `mods(mods: number)`
- `acc(acc: number)`
- `nGeki(nGeki: number)`
- `nKatu(nKatu: number)`
- `n300(n300: number)`
- `n100(n100: number)`
- `n50(n50: number)`
- `nMisses(nMisses: number)`
- `combo(combo: number)`
- `passedObjects(passedObjects: number)`
- `clockRate(clockRate: number)`

Finally, use one of the following methods to calculate values:

- `mapAttributes(map: Beatmap)` returns an object of the form
 ```js
{
    mode: number,
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
 ```
- `difficulty(map: Beatmap)` returns an object of the form
```js
{
    // all modes:
    mode: number,
    stars: number,
    maxCombo: number,
    // only osu!:
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
    // only taiko:
    stamina: number,
    rhythm: number,
    color: number,
    peak: number,
    hitWindow: number,
    // only catch:
    ar: number,
    nFruits: number,
    nDroplets: number,
    nTinyDroplets: number,
    // only mania:
    hitWindow: number,
}
```
- `performance(map: Beatmap)` returns an object of the form
```js
{
    // all modes:
    mode: number,
    pp: number,
    difficulty: Object, // same structure as above
    // only osu!:
    ppAcc: number,
    ppAim: number,
    ppFlashlight: number,
    ppSpeed: number,
    effectiveMissCount: number,
    // only taiko:
    ppAcc: number,
    ppDifficulty: number,
    effectiveMissCount: number,
    // only catch:
    ppDifficulty: number,
    // nothing additional for mania
}
```
- `strains(map: Beatmap)` returns an object of the form
```js
{
    // all modes:
    mode: number,
    sectionLength: number, // milliseconds between two strain points
    // only osu!:
    aim: Array<number>,
    aimNoSliders: Array<number>,
    speed: Array<number>,
    flashlight: Array<number>,
    // only taiko:
    color: Array<number>,
    rhythm: Array<number>,
    stamina: Array<number>,
    // only catch:
    movement: Array<number>,
    // only mania:
    strains: Array<number>,
}
```

#### Example

```js
import { Beatmap, Calculator } from require('rosu-pp');

let map = new Beatmap({ path: './maps/100.osu' });

let score = {
    mode: 2, // osu!catch
    mods: 8 + 64, // HDDT
};

let calc = new Calculator(score);

let maxAttrs = calc.performance(map);

let currAttrs = calc.n300(150)
    .n100(10)
    .nMisses(20)
    .passedObjects(200)
    .performance(map);

console.log(`stars: ${maxAttrs.difficulty.stars}`);
console.log(`pp: ${currAttrs.pp}/${maxAttrs.pp}`);
```

## Installing rosu-pp-js

Installing rosu-pp-js requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

Once [Node](https://nodejs.org) and [Rust](https://www.rust-lang.org/learn/get-started) are ready to go, you can install the project with npm. In your project directory, run:

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
