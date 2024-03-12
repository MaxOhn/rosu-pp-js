use rosu_pp::{model::mode::GameMode, Difficulty};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    attributes::difficulty::JsDifficultyAttributes,
    beatmap::JsBeatmap,
    gradual::{difficulty::JsGradualDifficulty, performance::JsGradualPerformance},
    mode::JsGameMode,
    performance::JsPerformance,
};

/// Builder for a difficulty calculation.
#[wasm_bindgen(js_name = Difficulty)]
#[derive(Clone)]
pub struct JsDifficulty {
    pub(crate) inner: Difficulty,
    pub(crate) mode: Option<GameMode>,
}

#[wasm_bindgen(js_class = Difficulty)]
impl JsDifficulty {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        #[cfg(feature = "panic_hook")]
        console_error_panic_hook::set_once();

        Self {
            inner: Difficulty::new(),
            mode: None,
        }
    }

    /// Specify a gamemode.
    pub fn mode(self, mode: JsGameMode) -> Self {
        Self {
            inner: self.inner,
            mode: Some(mode.into()),
        }
    }

    /// Specify mods through their bit values.
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub fn mods(self, mods: u32) -> Self {
        Self {
            inner: self.inner.mods(mods),
            mode: self.mode,
        }
    }

    /// Amount of passed objects for partial plays, e.g. a fail.
    ///
    /// If you want to calculate the difficulty after every few objects,
    /// instead of using `Difficulty` multiple times with different
    /// `passedObjects`, you should use `GradualDifficulty`.
    #[wasm_bindgen(js_name = passedObjects)]
    pub fn passed_objects(self, passedObjects: u32) -> Self {
        Self {
            inner: self.inner.passed_objects(passedObjects),
            mode: self.mode,
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
            mode: self.mode,
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
            mode: self.mode,
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
            mode: self.mode,
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
            mode: self.mode,
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
            mode: self.mode,
        }
    }

    /// Adjust patterns as if the HR mod is enabled.
    ///
    /// Only relevant for osu!catch.
    #[wasm_bindgen(js_name = hardrockOffsets)]
    pub fn hardrock_offsets(self, hardrockOffsets: bool) -> Self {
        Self {
            inner: self.inner.hardrock_offsets(hardrockOffsets),
            mode: self.mode,
        }
    }

    /// Perform the difficulty calculation.
    /// @throws Will throw an error if the specified mode is incompatible with the map's mode
    pub fn calculate(&self, map: &mut JsBeatmap) -> Result<JsDifficultyAttributes, String> {
        if let Some(mode) = self.mode {
            map.convert_native(mode)?;
        }

        Ok(self.inner.calculate(&map.inner).into())
    }

    /// Returns a performance calculator for the current difficulty settings.
    pub fn performance(&self) -> JsPerformance {
        JsPerformance::new().difficulty(self)
    }

    /// Returns a gradual difficulty calculator for the current difficulty settings.
    /// @throws Will throw an error if the specified mode is incompatible with the map's mode
    pub fn gradual_difficulty(&self, map: &mut JsBeatmap) -> Result<JsGradualDifficulty, String> {
        JsGradualDifficulty::new(self, map)
    }

    /// Returns a gradual performance calculator for the current difficulty settings.
    /// @throws Will throw an error if the specified mode is incompatible with the map's mode
    pub fn gradual_performance(&self, map: &mut JsBeatmap) -> Result<JsGradualPerformance, String> {
        JsGradualPerformance::new(self, map)
    }
}
