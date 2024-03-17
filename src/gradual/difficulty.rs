use rosu_pp::GradualDifficulty;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    attributes::difficulty::{DifficultyAttributes, JsDifficultyAttributes},
    beatmap::JsBeatmap,
    difficulty::JsDifficulty,
    gradual::maybe_convert_serialize,
    util, JsResult,
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
            inner: GradualDifficulty::new(difficulty.inner.clone(), &map.inner),
        }
    }

    /// Advances the iterator and returns the next attributes.
    pub fn next(&mut self) -> JsResult<Option<JsDifficultyAttributes>> {
        maybe_convert_serialize::<DifficultyAttributes, _, _>(self.inner.next())
    }

    /// Returns the `n`th attributes of the iterator.
    ///
    /// Note that the count starts from zero, so `nth(0)` returns the first
    /// value, `nth(1)` the second, and so on.
    pub fn nth(&mut self, n: usize) -> JsResult<Option<JsDifficultyAttributes>> {
        maybe_convert_serialize::<DifficultyAttributes, _, _>(self.inner.nth(n))
    }

    /// Advances the iterator to the end to collect all remaining attributes
    /// into a list and return them.
    pub fn collect(self) -> JsResult<Vec<JsDifficultyAttributes>> {
        self.inner
            .map(DifficultyAttributes::from)
            .map(|attrs| util::to_value(&attrs).map(From::from))
            .collect()
    }

    /// Returns the exact remaining length of the iterator.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }
}
