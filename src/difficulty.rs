use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::difficulty::{DifficultyArgs, JsDifficultyArgs},
    attributes::difficulty::JsDifficultyAttributes,
    beatmap::JsBeatmap,
    deserializer::JsDeserializer,
    gradual::{difficulty::JsGradualDifficulty, performance::JsGradualPerformance},
    mods::JsGameMods,
    strains::JsStrains,
    util, JsResult,
};

/// Builder for a difficulty calculation.
#[wasm_bindgen(js_name = Difficulty)]
#[derive(Clone)]
pub struct JsDifficulty {
    pub(crate) args: DifficultyArgs,
}

#[wasm_bindgen(js_class = Difficulty)]
impl JsDifficulty {
    /// Create a new difficulty calculator.
    #[wasm_bindgen(constructor)]
    pub fn new(args: Option<JsDifficultyArgs>) -> JsResult<JsDifficulty> {
        #[cfg(feature = "panic_hook")]
        console_error_panic_hook::set_once();

        let args = args
            .as_deref()
            .map(util::from_value::<DifficultyArgs>)
            .transpose()?
            .unwrap_or_default();

        Ok(Self { args })
    }

    /// Perform the difficulty calculation.
    pub fn calculate(&self, map: &JsBeatmap) -> JsDifficultyAttributes {
        JsDifficultyAttributes::from(self.args.to_difficulty().calculate(&map.inner))
    }

    /// Perform the difficulty calculation but instead of evaluating strain
    /// values, return them as is.
    ///
    /// Suitable to plot the difficulty over time.
    pub fn strains(&self, map: &JsBeatmap) -> JsStrains {
        self.args.to_difficulty().strains(&map.inner).into()
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

    #[wasm_bindgen(setter)]
    pub fn set_mods(&mut self, mods: Option<JsGameMods>) -> JsResult<()> {
        self.args.mods = mods
            .as_deref()
            .map(JsDeserializer::from_ref)
            .map(util::deserialize_mods)
            .transpose()?
            .unwrap_or_default();

        Ok(())
    }

    #[wasm_bindgen(setter = clockRate)]
    pub fn set_clock_rate(&mut self, clock_rate: Option<f64>) {
        self.args.clock_rate = clock_rate;
    }

    #[wasm_bindgen(setter)]
    pub fn set_ar(&mut self, ar: Option<f32>) {
        self.args.ar = ar;
    }

    #[wasm_bindgen(setter = arWithMods)]
    pub fn set_ar_with_mods(&mut self, ar_with_mods: Option<bool>) {
        self.args.ar_with_mods = ar_with_mods.unwrap_or_default();
    }

    #[wasm_bindgen(setter)]
    pub fn set_cs(&mut self, cs: Option<f32>) {
        self.args.cs = cs;
    }

    #[wasm_bindgen(setter = csWithMods)]
    pub fn set_cs_with_mods(&mut self, cs_with_mods: Option<bool>) {
        self.args.cs_with_mods = cs_with_mods.unwrap_or_default();
    }

    #[wasm_bindgen(setter)]
    pub fn set_hp(&mut self, hp: Option<f32>) {
        self.args.hp = hp;
    }

    #[wasm_bindgen(setter = hpWithMods)]
    pub fn set_hp_with_mods(&mut self, hp_with_mods: Option<bool>) {
        self.args.hp_with_mods = hp_with_mods.unwrap_or_default();
    }

    #[wasm_bindgen(setter)]
    pub fn set_od(&mut self, od: Option<f32>) {
        self.args.od = od;
    }

    #[wasm_bindgen(setter = odWithMods)]
    pub fn set_od_with_mods(&mut self, od_with_mods: Option<bool>) {
        self.args.od_with_mods = od_with_mods.unwrap_or_default();
    }

    #[wasm_bindgen(setter = passedObjects)]
    pub fn set_passed_objects(&mut self, passed_objects: Option<u32>) {
        self.args.passed_objects = passed_objects;
    }

    #[wasm_bindgen(setter = hardrockOffsets)]
    pub fn set_hardrock_offsets(&mut self, hardrock_offsets: Option<bool>) {
        self.args.hardrock_offsets = hardrock_offsets;
    }
}
