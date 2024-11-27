use rosu_pp::Performance;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::performance::{
        JsHitResultPriority, JsMapOrAttributes, JsPerformanceArgs, MapOrAttrs, PerformanceArgs,
    },
    attributes::performance::JsPerformanceAttributes,
    deserializer::JsDeserializer,
    mods::JsGameMods,
    util, JsResult,
};

/// Builder for a performance calculation.
#[wasm_bindgen(js_name = Performance)]
pub struct JsPerformance {
    args: PerformanceArgs,
}

#[wasm_bindgen(js_class = Performance)]
impl JsPerformance {
    /// Create a new performance calculator.
    #[wasm_bindgen(constructor)]
    pub fn new(args: Option<JsPerformanceArgs>) -> JsResult<JsPerformance> {
        let args = args
            .as_deref()
            .map(util::from_value::<PerformanceArgs>)
            .transpose()?
            .unwrap_or_default();

        Ok(Self { args })
    }

    /// Calculate performance attributes.
    ///
    /// If a beatmap is passed as argument, difficulty attributes will have to
    /// be calculated internally which is a comparably expensive task. Hence,
    /// passing previously calculated attributes should be prefered whenever
    /// available.
    ///
    /// However, be careful that the passed attributes have been calculated
    /// for the same difficulty settings like mods, clock rate, beatmap,
    /// custom ar, ... otherwise the final attributes will be incorrect.
    pub fn calculate(&mut self, args: &JsMapOrAttributes) -> JsResult<JsPerformanceAttributes> {
        let map_or_attrs = MapOrAttrs::from_value(args)?;
        let map;

        let mut perf = match map_or_attrs {
            MapOrAttrs::Map(map_) => {
                map = map_;

                Performance::new(&map.inner)
            }
            MapOrAttrs::Attrs(attrs) => Performance::new(attrs),
        };

        perf = self.args.apply(perf);
        let state = perf.generate_state();
        let attrs = JsPerformanceAttributes::new(perf.calculate(), state);

        Ok(attrs)
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

    #[wasm_bindgen(setter)]
    pub fn set_lazer(&mut self, lazer: Option<bool>) {
        self.args.lazer = lazer;
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

    #[wasm_bindgen(setter)]
    pub fn set_accuracy(&mut self, accuracy: Option<f64>) {
        self.args.accuracy = accuracy;
    }

    #[wasm_bindgen(setter)]
    pub fn set_combo(&mut self, combo: Option<u32>) {
        self.args.combo = combo;
    }

    #[wasm_bindgen(setter = largeTickHits)]
    pub fn set_large_ticks_hits(&mut self, large_tick_hits: Option<u32>) {
        self.args.large_tick_hits = large_tick_hits;
    }

    #[wasm_bindgen(setter = sliderEndHits)]
    pub fn set_slider_ends_hit(&mut self, slider_end_hits: Option<u32>) {
        self.args.slider_end_hits = slider_end_hits;
    }

    #[wasm_bindgen(setter = nGeki)]
    pub fn set_n_geki(&mut self, n_geki: Option<u32>) {
        self.args.n_geki = n_geki;
    }

    #[wasm_bindgen(setter = nKatu)]
    pub fn set_n_katu(&mut self, n_katu: Option<u32>) {
        self.args.n_katu = n_katu;
    }

    #[wasm_bindgen(setter)]
    pub fn set_n300(&mut self, n300: Option<u32>) {
        self.args.n300 = n300;
    }

    #[wasm_bindgen(setter)]
    pub fn set_n100(&mut self, n100: Option<u32>) {
        self.args.n100 = n100;
    }

    #[wasm_bindgen(setter)]
    pub fn set_n50(&mut self, n50: Option<u32>) {
        self.args.n50 = n50;
    }

    #[wasm_bindgen(setter)]
    pub fn set_misses(&mut self, misses: Option<u32>) {
        self.args.misses = misses;
    }

    #[wasm_bindgen(setter = hitresultPriority)]
    pub fn set_hitresult_priority(&mut self, hitresult_priority: Option<JsHitResultPriority>) {
        self.args.hitresult_priority = hitresult_priority.map_or_else(Default::default, From::from);
    }
}
