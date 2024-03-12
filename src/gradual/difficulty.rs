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
    /// @throws Will throw an error if the specified mode is incompatible with the map's mode
    #[wasm_bindgen(constructor)]
    pub fn new(
        difficulty: &JsDifficulty,
        map: &mut JsBeatmap,
    ) -> Result<JsGradualDifficulty, String> {
        if let Some(mode) = difficulty.mode {
            map.convert_native(mode)?;
        }

        Ok(Self {
            inner: GradualDifficulty::new(difficulty.inner.clone(), &map.inner),
        })
    }

    /// Advances the iterator and returns the next attributes.
    pub fn next(&mut self) -> Option<JsDifficultyAttributes> {
        self.inner.next().map(JsDifficultyAttributes::from)
    }

    /// Returns the `n`th attributes of the iterator.
    ///
    /// Note that the count starts from zero, so `nth(0)` returns the first
    /// value, `nth(1)` the second, and so on.
    pub fn nth(&mut self, n: usize) -> Option<JsDifficultyAttributes> {
        self.inner.nth(n).map(JsDifficultyAttributes::from)
    }

    /// Advances the iterator to the end to collect all remaining attributes
    /// into a list and return them.
    pub fn collect(self) -> Vec<JsDifficultyAttributes> {
        self.inner.map(JsDifficultyAttributes::from).collect()
    }

    /// Returns the exact remaining length of the iterator.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }
}
