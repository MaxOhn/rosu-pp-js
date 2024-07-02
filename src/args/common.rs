use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
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
}"#;
