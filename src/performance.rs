use rosu_pp::{
    any::{DifficultyAttributes, HitResultPriority},
    model::mode::GameMode,
    Performance,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    attributes::{difficulty::JsDifficultyAttributes, performance::JsPerformanceAttributes},
    beatmap::JsBeatmap,
    cannot_convert,
    difficulty::JsDifficulty,
};

/// Builder for a performance calculation.
#[wasm_bindgen(js_name = Performance)]
pub struct JsPerformance {
    difficulty: JsDifficulty,
    acc: Option<f64>,
    combo: Option<u32>,
    n_geki: Option<u32>,
    n_katu: Option<u32>,
    n300: Option<u32>,
    n100: Option<u32>,
    n50: Option<u32>,
    misses: Option<u32>,
    hitresult_priority: HitResultPriority,
}

#[wasm_bindgen(js_class = Performance)]
impl JsPerformance {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new(difficulty: Option<JsDifficulty>) -> Self {
        Self {
            difficulty: difficulty.unwrap_or_else(JsDifficulty::new),
            acc: None,
            combo: None,
            n_geki: None,
            n_katu: None,
            n300: None,
            n100: None,
            n50: None,
            misses: None,
            hitresult_priority: HitResultPriority::default(),
        }
    }

    /// Use the specified settings of the given `Difficulty`.
    pub fn difficulty(mut self, difficulty: JsDifficulty) -> Self {
        self.difficulty = difficulty;

        self
    }

    /// Specify mods through their bit values.
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub fn mods(mut self, mods: u32) -> Self {
        self.difficulty = self.difficulty.mods(mods);

        self
    }

    /// Amount of passed objects for partial plays, e.g. a fail.
    ///
    /// If you want to calculate the performance after every few objects,
    /// instead of using `Performance` multiple times with different
    /// `passedObjects`, you should use `GradualPerformance`.
    pub fn passedObjects(mut self, passedObjects: u32) -> Self {
        self.difficulty = self.difficulty.passedObjects(passedObjects);

        self
    }

    /// Adjust the clock rate used in the calculation.
    ///
    /// If none is specified, it will take the clock rate based on the mods
    /// i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | 0.01    | 100     |
    pub fn clockRate(mut self, clockRate: f64) -> Self {
        self.difficulty = self.difficulty.clockRate(clockRate);

        self
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
    pub fn ar(mut self, ar: f32, withMods: bool) -> Self {
        self.difficulty = self.difficulty.ar(ar, withMods);

        self
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
    pub fn cs(mut self, cs: f32, withMods: bool) -> Self {
        self.difficulty = self.difficulty.cs(cs, withMods);

        self
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
    pub fn hp(mut self, hp: f32, withMods: bool) -> Self {
        self.difficulty = self.difficulty.hp(hp, withMods);

        self
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
    pub fn od(mut self, od: f32, withMods: bool) -> Self {
        self.difficulty = self.difficulty.od(od, withMods);

        self
    }

    /// Adjust patterns as if the HR mod is enabled.
    ///
    /// Only relevant for osu!catch.
    pub fn hardrockOffsets(mut self, hardrockOffsets: bool) -> Self {
        self.difficulty = self.difficulty.hardrockOffsets(hardrockOffsets);

        self
    }

    /// Set the accuracy between `0.0` and `100.0`.
    pub fn accuracy(mut self, acc: f64) -> Self {
        self.acc = Some(acc);

        self
    }

    /// Specify the max combo of the play.
    ///
    /// Irrelevant for osu!mania.
    pub fn combo(mut self, combo: u32) -> Self {
        self.combo = Some(combo);

        self
    }

    /// Specify the amount of gekis of a play.
    ///
    /// Only relevant for osu!mania for which it repesents the amount of n320.
    pub fn nGeki(mut self, nGeki: u32) -> Self {
        self.n_geki = Some(nGeki);

        self
    }

    /// Specify the amount of katus of a play.
    ///
    /// Only relevant for osu!catch for which it represents the amount of tiny
    /// droplet misses and osu!mania for which it repesents the amount of n200.
    pub fn nKatu(mut self, nKatu: u32) -> Self {
        self.n_katu = Some(nKatu);

        self
    }

    /// Specify the amount of 300s of a play.
    pub fn n300(mut self, n300: u32) -> Self {
        self.n300 = Some(n300);

        self
    }

    /// Specify the amount of 100s of a play.
    pub fn n100(mut self, n100: u32) -> Self {
        self.n100 = Some(n100);

        self
    }

    /// Specify the amount of 50s of a play.
    ///
    /// Irrelevant for osu!taiko.
    pub fn n50(mut self, n50: u32) -> Self {
        self.n50 = Some(n50);

        self
    }

    /// Specify the amount of misses of a play.
    pub fn misses(mut self, misses: u32) -> Self {
        self.misses = Some(misses);

        self
    }

    /// Specify how hitresults should be generated.
    ///
    /// Defauls to `HitResultPriority.BestCase`.
    pub fn hitresultPriority(mut self, priority: JsHitResultPriority) -> Self {
        self.hitresult_priority = priority.into();

        self
    }

    /// Calculate performance attributes for a map.
    ///
    /// Note that using this method will perform the costly computation of
    /// difficulty attributes internally. If attributes for the current
    /// `Difficulty` settings are already available, consider using the method
    /// `calculate_with_attributes` instead.
    /// @throws Will throw an error if the specified mode is incompatible with the map's mode
    pub fn calculate_with_map(
        self,
        map: &mut JsBeatmap,
    ) -> Result<JsPerformanceAttributes, String> {
        if let Some(mode) = self.difficulty.mode {
            map.convert_native(mode)?;
        }

        Ok(self.calculate(Performance::from_map(&map.inner)))
    }

    /// Calculate performance attributes.
    ///
    /// Note that `attrs` must have been calculated for the same map and
    /// `Difficulty` settings, otherwise the final attributes will be incorrect.
    /// @throws Will throw an error if the specified mode is incompatible with the attributes or if the attributes have been modified manually
    pub fn calculate_with_attributes(
        self,
        attrs: JsDifficultyAttributes,
    ) -> Result<JsPerformanceAttributes, String> {
        if let Some(to) = self.difficulty.mode {
            let from = GameMode::from(attrs.mode);

            if from != to {
                return Err(cannot_convert(from, to));
            }
        }

        let attrs = DifficultyAttributes::try_from(attrs)?;

        Ok(self.calculate(Performance::from_attributes(attrs)))
    }
}

impl JsPerformance {
    fn calculate(self, performance: Performance<'_>) -> JsPerformanceAttributes {
        let mut performance = performance
            .difficulty(self.difficulty.inner)
            .hitresult_priority(self.hitresult_priority);

        if let Some(acc) = self.acc {
            performance = performance.accuracy(acc);
        }

        if let Some(combo) = self.combo {
            performance = performance.combo(combo);
        }

        if let Some(n_geki) = self.n_geki {
            performance = performance.n_geki(n_geki);
        }

        if let Some(n_katu) = self.n_katu {
            performance = performance.n_katu(n_katu);
        }

        if let Some(n300) = self.n300 {
            performance = performance.n300(n300);
        }

        if let Some(n100) = self.n100 {
            performance = performance.n100(n100);
        }

        if let Some(n50) = self.n50 {
            performance = performance.n50(n50);
        }

        if let Some(misses) = self.misses {
            performance = performance.misses(misses);
        }

        performance.calculate().into()
    }
}

/// While generating remaining hitresults, decide how they should be distributed.
#[wasm_bindgen(js_name = HitResultPriority)]
pub enum JsHitResultPriority {
    /// Prioritize good hitresults over bad ones
    BestCase,
    /// Prioritize bad hitresults over good ones
    WorstCase,
}

impl From<HitResultPriority> for JsHitResultPriority {
    fn from(priority: HitResultPriority) -> Self {
        match priority {
            HitResultPriority::BestCase => Self::BestCase,
            HitResultPriority::WorstCase => Self::WorstCase,
        }
    }
}

impl From<JsHitResultPriority> for HitResultPriority {
    fn from(priority: JsHitResultPriority) -> Self {
        match priority {
            JsHitResultPriority::BestCase => Self::BestCase,
            JsHitResultPriority::WorstCase => Self::WorstCase,
        }
    }
}
