use rosu_pp::GradualPerformance;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    attributes::performance::JsPerformanceAttributes, beatmap::JsBeatmap, difficulty::JsDifficulty,
    score_state::JsScoreState,
};

/// Gradually calculate performance attributes after each hitresult.
#[wasm_bindgen(js_name = GradualPerformance)]
pub struct JsGradualPerformance {
    inner: GradualPerformance,
}

#[wasm_bindgen(js_class = GradualPerformance)]
impl JsGradualPerformance {
    /// @throws Will throw an error if the specified mode is incompatible with the map's mode
    #[wasm_bindgen(constructor)]
    pub fn new(
        difficulty: JsDifficulty,
        map: &mut JsBeatmap,
    ) -> Result<JsGradualPerformance, String> {
        if let Some(mode) = difficulty.mode {
            map.convert_native(mode)?;
        }

        Ok(Self {
            inner: GradualPerformance::new(difficulty.inner, &map.inner),
        })
    }

    /// Process the next hit object and calculate the performance attributes
    /// for the resulting score state.
    pub fn next(&mut self, state: JsScoreState) -> Option<JsPerformanceAttributes> {
        self.inner
            .next(state.into())
            .map(JsPerformanceAttributes::from)
    }

    /// Process everything up to the next `n`th hitobject and calculate the
    /// performance attributes for the resulting score state.
    ///
    /// Note that the count is zero-indexed, so `n=0` will process 1 object,
    /// `n=1` will process 2, and so on.
    pub fn nth(&mut self, state: JsScoreState, n: usize) -> Option<JsPerformanceAttributes> {
        self.inner
            .nth(state.into(), n)
            .map(JsPerformanceAttributes::from)
    }

    /// Returns the amount of remaining objects.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }
}
