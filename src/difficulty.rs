use rosu_pp::Difficulty;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::difficulty::{DifficultyArgs, JsDifficultyArgs},
    attributes::difficulty::{DifficultyAttributes, JsDifficultyAttributes},
    beatmap::JsBeatmap,
    gradual::{difficulty::JsGradualDifficulty, performance::JsGradualPerformance},
    strains::{JsStrains, Strains},
    util, JsResult,
};

/// Builder for a difficulty calculation.
#[wasm_bindgen(js_name = Difficulty)]
#[derive(Clone)]
pub struct JsDifficulty {
    pub(crate) inner: Difficulty,
}

#[wasm_bindgen(js_class = Difficulty)]
impl JsDifficulty {
    /// Create a new difficulty calculator.
    #[wasm_bindgen(constructor)]
    pub fn new(args: Option<JsDifficultyArgs>) -> JsResult<JsDifficulty> {
        #[cfg(feature = "panic_hook")]
        console_error_panic_hook::set_once();

        let inner = if let Some(args) = args {
            let args = DifficultyArgs::from_value(&args)?;

            args.as_difficulty()
        } else {
            Difficulty::new()
        };

        Ok(Self { inner })
    }

    /// Perform the difficulty calculation.
    pub fn calculate(&self, map: &JsBeatmap) -> JsResult<JsDifficultyAttributes> {
        let attrs = DifficultyAttributes::from(self.inner.calculate(&map.inner));

        util::to_value(&attrs).map(From::from)
    }

    /// Perform the difficulty calculation but instead of evaluating strain
    /// values, return them as is.
    ///
    /// Suitable to plot the difficulty over time.
    pub fn strains(&self, map: &JsBeatmap) -> JsResult<JsStrains> {
        let strains = Strains::from(self.inner.strains(&map.inner));

        util::to_value(&strains).map(From::from)
    }

    /// Returns a gradual difficulty calculator for the current difficulty settings.
    #[wasm_bindgen(js_name = gradualDifficulty)]
    pub fn gradual_difficulty(&self, map: &JsBeatmap) -> JsGradualDifficulty {
        JsGradualDifficulty::new(self, map)
    }

    /// Returns a gradual performance calculator for the current difficulty settings.
    #[wasm_bindgen(js_name = gradualPerformance)]
    pub fn gradual_performance(&self, map: &JsBeatmap) -> JsGradualPerformance {
        JsGradualPerformance::new(self, map)
    }
}
