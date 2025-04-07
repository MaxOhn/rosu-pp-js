# v3.0.0 (2025-04-07)

Updated all modes' difficulty and performance calculation. See osu!'s newspost for more info: <https://osu.ppy.sh/home/news/2025-03-06-performance-points-star-rating-updates>

rosu-pp changelog: <https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v300-2025-04-07>

- Removed fields:
  - `DifficultyAttributes.od`
  - `DifficultyAttributes.peak`

- Added fields:
  - `DifficultyAttributes.aimDifficultSliderCount` (osu!standard)
  - `DifficultyAttributes.reading` (osu!taiko)
  - `DifficultyAttributes.mehHitWindow` (osu!standard)
  - `PerformanceAttributes.speedDeviation` (osu!standard)
  - `Strains.reading` (osu!taiko)
  - `BeatmapAttributes.odMehHitWindow` (osu!standard)

- Adjustments:
  - The field `DifficultyAttributes.greatHitWindow` is no longer available for osu!mania but it is now available for osu!standard
  - The field `DifficultyAttributes.okHitWindow` is now also available for osu!standard

## v2.0.0 (2024-12-04)

Updated all modes' difficulty and performance calculation. See osu!'s newspost for more info: <https://osu.ppy.sh/home/news/2024-10-28-performance-points-star-rating-updates>

rosu-pp changelog: <https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v200-2024-12-03>


- __Breaking changes:__
  - `BeatmapAttributesBuilder.build` *consumes* the builder, meaning its instance
    cannot be used anymore after calling the `build` method. ([#23])
  - Renamed the field `BeatmapAttributes.odHitWindow` to `odGreatHitWindow`
  - Renamed the field `DifficultyAttributes.hitWindow` to `greatHitWindow`

- __Additions:__
  - `Difficulty` and `Performance` now accept the field `lazer?: bool` (defaults to `true` if unspecified);
    Performance calculation for osu!standard and osu!mania now differs between lazer and stable so this is
    important to specify.
  - `Performance` now accepts the fields `largeTickHits?: number`, `smallTickHits?: number`, `sliderEndHits?: number`;
    each of them being necessary to specify for osu!standard scores on lazer.
  - `ScoreState` now has the additional fields
    - `osuLargeTickHits?: number`
    - `osuSmallTickHits?: number`
    - `sliderEndHits?: number`
  - The method `Beatmap.convert` now takes an optional second argument for gamemods
  - Added the field `BeatmapAttributes.odOkHitWindow`
  - Added fields to `DifficultyAttributes`:
    - `aimDifficultStrainCount` (osu!standard)
    - `speedDifficultStrainCount` (osu!standard)
    - `monoStaminaFactor` (osu!taiko)
    - `nHoldNotes` (osu!mania)
    - `nLargeTicks` (osu!standard)
    - `okHitWindow` (osu!taiko)
  - Added the field `PerformanceAttributes.estimatedUnstableRate` (osu!taiko)
  - Added the field `Strains.singleColorStamina` (osu!taiko)

- __Fixes:__
  - Mod settings of legacy mods are now considered correctly ([#24] & [#25])
  - The type name is no longer checked when deserializing JS objects ([#27])

## v1.1.1 (2024-10-15)

- Stack trace for thrown errors is now preserved ([#19])
- Fixed occasional "recursive use of an object" error when using a `Beatmap` instance multiple times ([#21])

## v1.1.0 (2024-07-12)

- Added a ⚠️WARNING⚠️ in the readme to call `Beatmap.free` whenever a `Beatmap` instance is no longer of use to avoid risking memory leakage.
  ```js
  const beatmap = new rosu.Beatmap(content);
  const attrs = new rosu.Performance().calculate(beatmap);
  beatmap.free();
  ```
- Updated to [rosu-pp v1.1.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v110-2024-07-10)
- Mods can now be specified through more types than just `number` ([#16]):
  - an integer for bitflags
  - a string for acronyms
  - a single mod object as described below
  - a sequence of types that deserialize into a single mod

  Types that deserialize into a single mod are
  - an integer for bitflags
  - a string for an acronym
  - a mod object

  A mod object must have an `acronym: string` property and an optional `settings?: Object` property.
- For `Difficulty`, `Performance`, and `BeatmapAttributesBuilder`, all fields of their constructors' fields are now also available as setters after initialization. ([#14])
  ```js
  let calc = new rosu.Difficulty({ clockRate: 1.7 });
  calc.mods = "HDDT";
  calc.clockRate = undefined;
  ```

## v1.0.2 (2024-04-16)

- Fixed the `state` field of `PerformanceAttributes` ([#9])
- Removed the `wee_alloc` feature because its repository is unmaintained.

## v1.0.1 (2024-04-03)

- Fixed difficulty arguments not being considered as performance attributes

## v1.0.0 (2024-04-03)

- Updated to [rosu-pp v1.0.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v100-2024-04-02)
- The binding is no longer imported under the name `rosu-pp` but as `rosu-pp-js`
- The binding is now powered by [`Wasm`](https://webassembly.org/) instead of [`Neon`](https://neon-bindings.com/). As a result, Rust is no longer required to be installed and the library now works in non-nodejs environments like browsers.
- Breaking changes ahead! There are now multiple different calculators:
  - `Difficulty` to calculate `DifficultyAttributes`, `Strains`, or create gradual calculators
  - `Performance` to calculate `PerformanceAttributes`
  - `BeatmapAttributesBuilder` to calculate `BeatmapAttributes`
  - `GradualDifficulty` to calculate `DifficultyAttributes` for each hitobject
  - `GradualPerformance` to calculate `PerformanceAttributes` for each hitresult

Check out the [`rosu_pp_js.d.ts`](https://github.com/MaxOhn/rosu-pp-js/blob/e7e5ad1d128ac488aa3a72f9582db4c2f2804afb/rosu_pp_js.d.ts) file to see available types, methods, arguments, and fields. The `README.md` file provides some more explanations and examples.

## v0.9.4 (2023-02-09)

- Updated to [rosu-pp v0.9.4](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v094-2023-02-09).

## v0.9.3 (2023-01-28)

- Updated to [rosu-pp v0.9.3](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v093-2023-01-28). Only includes some bug fixes.
- Map attributes now also include the properties `mode`, `version`, `nCircles`, `nSliders`, and `nSpinners`.

## v0.9.1 (2022-10-26)

- Updated to [rosu-pp v0.9.1](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v091-2022-10-26) including the big changes in [v0.9.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v090-2022-10-24)
- The binding interface is rewritten completely, see the readme.

## v0.8.0 (2022-08-02)
- Updated to [rosu-pp v0.8.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v080-2022-08-02)
- The calculation result now contains a `timePreempt` field for osu!standard and `greatHitWindow` for
osu!standard, osu!taiko, and osu!mania.
- Fixed map attributes when mods were interacting with custom clock rates

## v0.7.1 (2022-07-12)
- Updated to [rosu-pp v0.7.1](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v071-2022-07-12)

## v0.7.0 (2022-07-07)
- Updated to [rosu-pp v0.7.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v070-2022-07-06)
- Added `strains` method to calculate the strain values for all skills of the map's game mode

## v0.6.0 (2022-07-05)
- Updated to [rosu-pp v0.6.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v060-2022-07-05)
- The field `mode` can now be specified to convert maps to different modes
- Added the typescript type `enum GameMode { Osu, Taiko, Catch, Mania }`

## v0.5.4 (2022-06-14)
- Bumped patch version of dependencies, including a [rosu-pp](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v052-2022-06-14) update

## v0.5.3 (2022-04-21)
- Added typings for TypeScript ([#1] - [@minidomo])

## v0.5.2 (2022-03-21)
- Now considering custom map attributes properly

## v0.5.1 (2022-03-21)
- Updated to [rosu-pp v0.5.1](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v051-2022-03-21)
- `ScoreParams` may now include additional fields: `clockRate`, `ar`, `cs`, `hp`, and `od`
- The return object of a calculation now also includes a `clockRate` field

## v0.4.0 (2021-12-26)
- Initial release with rosu-pp v0.4.0

[@minidomo]: https://github.com/minidomo

[#1]: https://github.com/MaxOhn/rosu-pp-js/pull/1
[#9]: https://github.com/MaxOhn/rosu-pp-js/pull/9
[#14]: https://github.com/MaxOhn/rosu-pp-js/pull/14
[#16]: https://github.com/MaxOhn/rosu-pp-js/pull/16
[#19]: https://github.com/MaxOhn/rosu-pp-js/pull/19
[#21]: https://github.com/MaxOhn/rosu-pp-js/pull/21
[#23]: https://github.com/MaxOhn/rosu-pp-js/pull/23
[#24]: https://github.com/MaxOhn/rosu-pp-js/pull/24
[#25]: https://github.com/MaxOhn/rosu-pp-js/pull/25
[#27]: https://github.com/MaxOhn/rosu-pp-js/pull/27