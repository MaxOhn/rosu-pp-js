use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = CommonArgs)]
    pub type JsCommonArgs;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Common arguments to extend other argument interfaces.
*/
interface CommonArgs {
    /**
    * Specify mods through their bit values.
    *
    * See <https://github.com/ppy/osu-api/wiki#mods>
    */
    mods?: number;
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

#[derive(Default)]
pub struct CommonArgs {
    pub mods: u32,
    pub clock_rate: Option<f64>,
    pub ar: Option<f32>,
    pub ar_with_mods: bool,
    pub cs: Option<f32>,
    pub cs_with_mods: bool,
    pub hp: Option<f32>,
    pub hp_with_mods: bool,
    pub od: Option<f32>,
    pub od_with_mods: bool,
}

from_jsvalue! {
    CommonArgs {
        mods as mods: u32!,
        clock_rate as clockRate: f64?,
        ar as ar: f32?,
        ar_with_mods as arWithMods: bool!,
        cs as cs: f32?,
        cs_with_mods as csWithMods: bool!,
        hp as hp: f32?,
        hp_with_mods as hpWithMods: bool!,
        od as od: f32?,
        od_with_mods as odWithMods: bool!,
    }
}
