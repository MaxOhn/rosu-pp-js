use rosu_pp::GradualPerformance;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    attributes::performance::{JsPerformanceAttributes, PerformanceAttributes},
    beatmap::JsBeatmap,
    difficulty::JsDifficulty,
    gradual::maybe_convert_serialize,
    score_state::{JsScoreState, ScoreState},
    JsResult,
};

/// Gradually calculate performance attributes after each hitresult.
#[wasm_bindgen(js_name = GradualPerformance)]
pub struct JsGradualPerformance {
    inner: GradualPerformance,
}

#[wasm_bindgen(js_class = GradualPerformance)]
impl JsGradualPerformance {
    #[wasm_bindgen(constructor)]
    pub fn new(difficulty: &JsDifficulty, map: &JsBeatmap) -> JsGradualPerformance {
        Self {
            inner: GradualPerformance::new(difficulty.inner.clone(), &map.inner),
        }
    }

    /// Process the next hit object and calculate the performance attributes
    /// for the resulting score state.
    pub fn next(&mut self, state: &JsScoreState) -> JsResult<Option<JsPerformanceAttributes>> {
        let state = ScoreState::from_value(state)?;

        maybe_convert_serialize::<PerformanceAttributes, _, _>(self.inner.next(state))
    }

    /// Process everything up to the next `n`th hitobject and calculate the
    /// performance attributes for the resulting score state.
    ///
    /// Note that the count is zero-indexed, so `n=0` will process 1 object,
    /// `n=1` will process 2, and so on.
    pub fn nth(
        &mut self,
        state: &JsScoreState,
        n: usize,
    ) -> JsResult<Option<JsPerformanceAttributes>> {
        let state = ScoreState::from_value(state)?;

        maybe_convert_serialize::<PerformanceAttributes, _, _>(self.inner.nth(state, n))
    }

    /// Returns the amount of remaining items.
    #[wasm_bindgen(js_name = nRemaining, getter)]
    pub fn n_remaining(&self) -> usize {
        self.inner.len()
    }
}
