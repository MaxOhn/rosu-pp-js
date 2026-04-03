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
}"#;
