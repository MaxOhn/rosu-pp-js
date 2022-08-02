# v0.8.0 (2022-08-02)
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
