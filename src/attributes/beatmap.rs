use rosu_pp::model::beatmap::{BeatmapAttributes, BeatmapAttributesBuilder, HitWindows};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{beatmap::JsBeatmap, difficulty::JsDifficulty, mode::JsGameMode};

#[wasm_bindgen(js_name = BeatmapAttributesBuilder)]
pub struct JsBeatmapAttributesBuilder {
    inner: BeatmapAttributesBuilder,
}

#[wasm_bindgen(js_class = BeatmapAttributesBuilder)]
impl JsBeatmapAttributesBuilder {
    /// Create a new `BeatmapAttributesBuilder`.
    ///
    /// The mode will be `GameMode.Osu` and attributes are set to `5.0`.
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: BeatmapAttributesBuilder::new(),
        }
    }

    /// Create a new `BeatmapAttributesBuilder` from a beatmap.
    #[wasm_bindgen(js_name = fromMap)]
    pub fn from_map(map: &JsBeatmap) -> Self {
        Self {
            inner: BeatmapAttributesBuilder::from_map(&map.inner),
        }
    }

    /// Specify a gamemode and whether it's a converted map.
    pub fn mode(self, mode: JsGameMode, isConvert: bool) -> Self {
        Self {
            inner: self.inner.mode(mode.into(), isConvert),
        }
    }

    /// Specify all settings through a `JsDifficulty` object.
    ///
    /// **Note** if `difficulty` has a specified `mode` it will be ignored.
    pub fn difficulty(self, difficulty: &JsDifficulty) -> Self {
        Self {
            inner: self.inner.difficulty(&difficulty.inner),
        }
    }

    /// Specify mods through their bit values.
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub fn mods(self, mods: u32) -> Self {
        Self {
            inner: self.inner.mods(mods),
        }
    }

    /// Adjust the clock rate used in the calculation.
    ///
    /// If none is specified, it will take the clock rate based on the mods
    /// i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | 0.01    | 100     |
    #[wasm_bindgen(js_name = clockRate)]
    pub fn clock_rate(self, clockRate: f64) -> Self {
        Self {
            inner: self.inner.clock_rate(clockRate),
        }
    }

    /// Override a beatmap's set AR.
    ///
    /// Only relevant for osu! and osu!catch.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn ar(self, ar: f32, withMods: bool) -> Self {
        Self {
            inner: self.inner.ar(ar, withMods),
        }
    }

    /// Override a beatmap's set CS.
    ///
    /// Only relevant for osu! and osu!catch.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn cs(self, cs: f32, withMods: bool) -> Self {
        Self {
            inner: self.inner.cs(cs, withMods),
        }
    }

    /// Override a beatmap's set HP.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn hp(self, hp: f32, withMods: bool) -> Self {
        Self {
            inner: self.inner.hp(hp, withMods),
        }
    }

    /// Override a beatmap's set OD.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn od(self, od: f32, withMods: bool) -> Self {
        Self {
            inner: self.inner.od(od, withMods),
        }
    }

    /// Calculate the AR and OD hit windows.
    #[wasm_bindgen(js_name = hitWindows)]
    pub fn hit_windows(&self) -> JsHitWindows {
        self.inner.hit_windows().into()
    }

    /// Calculate the `BeatmapAttributes`.
    pub fn build(&self) -> JsBeatmapAttributes {
        self.inner.build().into()
    }
}

/// AR and OD hit windows
#[wasm_bindgen(js_name = HitWindows, inspectable)]
#[derive(Copy, Clone)]
pub struct JsHitWindows {
    /// Hit window for approach rate i.e. TimePreempt in milliseconds.
    pub ar: f64,
    /// Hit window for overall difficulty i.e. time to hit a 300 ("Great") in
    /// milliseconds.
    pub od: f64,
}

impl From<HitWindows> for JsHitWindows {
    fn from(hit_windows: HitWindows) -> Self {
        Self {
            ar: hit_windows.ar,
            od: hit_windows.od,
        }
    }
}

/// Summary object for a beatmap's attributes.
#[wasm_bindgen(js_name = BeatmapAttributes, inspectable)]
pub struct JsBeatmapAttributes {
    /// The approach rate.
    pub ar: f64,
    /// The overall difficulty.
    pub od: f64,
    /// The circle size.
    pub cs: f64,
    /// The health drain rate
    pub hp: f64,
    /// The clock rate with respect to mods.
    pub clock_rate: f64,
    /// The hit windows for approach rate and overall difficulty.
    pub hit_windows: JsHitWindows,
}

impl From<BeatmapAttributes> for JsBeatmapAttributes {
    fn from(attrs: BeatmapAttributes) -> Self {
        Self {
            ar: attrs.ar,
            od: attrs.od,
            cs: attrs.cs,
            hp: attrs.hp,
            clock_rate: attrs.clock_rate,
            hit_windows: attrs.hit_windows.into(),
        }
    }
}
