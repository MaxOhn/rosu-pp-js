use rosu_pp::GradualDifficulty;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    attributes::difficulty::JsDifficultyAttributes, beatmap::JsBeatmap, difficulty::JsDifficulty,
};

/// Gradually calculate difficulty attributes after each hitobject.
#[wasm_bindgen(js_name = GradualDifficulty)]
pub struct JsGradualDifficulty {
    inner: GradualDifficulty,
}

#[wasm_bindgen(js_class = GradualDifficulty)]
impl JsGradualDifficulty {
    #[wasm_bindgen(constructor)]
    pub fn new(difficulty: &JsDifficulty, map: &JsBeatmap) -> JsGradualDifficulty {
        Self {
            inner: GradualDifficulty::new(difficulty.args.as_difficulty(), &map.inner),
        }
    }

    /// Advances the iterator and returns the next attributes.
    pub fn next(&mut self) -> Option<JsDifficultyAttributes> {
        self.inner.next().map(From::from)
    }

    /// Returns the `n`th attributes of the iterator.
    ///
    /// Note that the count starts from zero, so `nth(0)` returns the first
    /// value, `nth(1)` the second, and so on.
    pub fn nth(&mut self, n: usize) -> Option<JsDifficultyAttributes> {
        self.inner.nth(n).map(From::from)
    }

    /// Advances the iterator to the end to collect all remaining attributes
    /// into a list and return them.
    pub fn collect(self) -> Vec<JsDifficultyAttributes> {
        self.inner.map(From::from).collect()
    }

    /// Returns the amount of remaining items.
    #[wasm_bindgen(js_name = nRemaining, getter)]
    pub fn n_remaining(&self) -> usize {
        self.inner.len()
    }
}
