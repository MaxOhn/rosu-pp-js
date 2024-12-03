use rosu_pp::model::beatmap::BeatmapAttributes;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::beatmap::{BeatmapAttributesArgs, JsBeatmapAttributesArgs},
    beatmap::JsBeatmap,
    deserializer::JsDeserializer,
    mode::JsGameMode,
    mods::JsGameMods,
    util, JsResult,
};

#[wasm_bindgen(js_name = BeatmapAttributesBuilder)]
pub struct JsBeatmapAttributesBuilder {
    args: BeatmapAttributesArgs,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = Beatmap)]
    pub type JsBeatmapType;
}

#[wasm_bindgen(js_class = BeatmapAttributesBuilder)]
impl JsBeatmapAttributesBuilder {
    /// Create a new `BeatmapAttributesBuilder`.
    #[wasm_bindgen(constructor)]
    pub fn new(args: Option<JsBeatmapAttributesArgs>) -> JsResult<JsBeatmapAttributesBuilder> {
        let args = args
            .as_deref()
            .map(util::from_value::<BeatmapAttributesArgs>)
            .transpose()?
            .unwrap_or_default();

        Ok(Self { args })
    }

    /// Calculate the `BeatmapAttributes`.
    pub fn build(self) -> JsBeatmapAttributes {
        self.args.into_builder().build().into()
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

    #[wasm_bindgen(setter)]
    pub fn set_mode(&mut self, mode: Option<JsGameMode>) {
        self.args.mode = mode;
    }

    #[wasm_bindgen(setter = isConvert)]
    pub fn set_is_convert(&mut self, is_convert: Option<bool>) {
        self.args.is_convert = is_convert.unwrap_or_default();
    }

    #[wasm_bindgen(setter)]
    pub fn set_map(&mut self, map: Option<JsBeatmapType>) -> JsResult<()> {
        self.args.map = map
            .as_deref()
            .map(JsDeserializer::from_ref)
            .map(JsBeatmap::deserialize)
            .transpose()?;

        Ok(())
    }
}

#[wasm_bindgen(js_name = BeatmapAttributes, inspectable)]
pub struct JsBeatmapAttributes {
    /// The approach rate.
    #[wasm_bindgen(readonly)]
    pub ar: f64,
    /// The overall difficulty.
    #[wasm_bindgen(readonly)]
    pub od: f64,
    /// The circle size.
    #[wasm_bindgen(readonly)]
    pub cs: f64,
    /// The health drain rate
    #[wasm_bindgen(readonly)]
    pub hp: f64,
    /// The clock rate with respect to mods.
    #[wasm_bindgen(js_name = "clockRate", readonly)]
    pub clock_rate: f64,
    /// Hit window for approach rate i.e. TimePreempt in milliseconds.
    #[wasm_bindgen(js_name = "arHitWindow", readonly)]
    pub ar_hitwindow: f64,
    /// Hit window for overall difficulty i.e. time to hit a 300 ("Great") in
    /// milliseconds.
    #[wasm_bindgen(js_name = "odGreatHitWindow", readonly)]
    pub od_great_hitwindow: f64,
    /// Hit window for overall difficulty i.e. time to hit a 100 ("Ok") in
    /// milliseconds.
    ///
    /// Not available for osu!mania.
    #[wasm_bindgen(js_name = "odOkHitWindow", readonly)]
    pub od_ok_hitwindow: Option<f64>,
}

impl From<BeatmapAttributes> for JsBeatmapAttributes {
    fn from(attrs: BeatmapAttributes) -> Self {
        Self {
            ar: attrs.ar,
            od: attrs.od,
            cs: attrs.cs,
            hp: attrs.hp,
            clock_rate: attrs.clock_rate,
            ar_hitwindow: attrs.hit_windows.ar,
            od_great_hitwindow: attrs.hit_windows.od_great,
            od_ok_hitwindow: attrs.hit_windows.od_ok,
        }
    }
}
