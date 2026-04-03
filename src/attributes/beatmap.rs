use rosu_pp::model::beatmap::{AdjustedBeatmapAttributes, BeatmapAttributes, HitWindows};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    JsResult,
    args::beatmap::{BeatmapAttributesArgs, JsBeatmapAttributesArgs},
    beatmap::JsBeatmap,
    deserializer::JsDeserializer,
    mode::JsGameMode,
    mods::JsGameMods,
    util,
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

    #[wasm_bindgen(setter)]
    pub fn set_fixed_ar(&mut self, fixed: Option<bool>) {
        self.args.fixed_ar = fixed.unwrap_or_default();
    }

    #[wasm_bindgen(setter)]
    pub fn set_cs(&mut self, cs: Option<f32>) {
        self.args.cs = cs;
    }

    #[wasm_bindgen(setter)]
    pub fn set_fixed_cs(&mut self, fixed: Option<bool>) {
        self.args.fixed_cs = fixed.unwrap_or_default();
    }

    #[wasm_bindgen(setter)]
    pub fn set_hp(&mut self, hp: Option<f32>) {
        self.args.hp = hp;
    }

    #[wasm_bindgen(setter)]
    pub fn set_fixed_hp(&mut self, fixed: Option<bool>) {
        self.args.fixed_hp = fixed.unwrap_or_default();
    }

    #[wasm_bindgen(setter)]
    pub fn set_od(&mut self, od: Option<f32>) {
        self.args.od = od;
    }

    #[wasm_bindgen(setter)]
    pub fn set_fixed_od(&mut self, fixed: Option<bool>) {
        self.args.fixed_od = fixed.unwrap_or_default();
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
    /// The base approach rate without considering clock rate.
    #[wasm_bindgen(js_name = "baseAr", readonly)]
    pub base_ar: f32,
    /// The overall difficulty.
    #[wasm_bindgen(readonly)]
    pub od: f64,
    /// The base overall difficulty without considering clock rate.
    #[wasm_bindgen(js_name = "baseOd", readonly)]
    pub base_od: f32,
    /// The circle size.
    #[wasm_bindgen(readonly)]
    pub cs: f32,
    /// The health drain rate
    #[wasm_bindgen(readonly)]
    pub hp: f32,
    /// The clock rate with respect to mods.
    #[wasm_bindgen(js_name = "clockRate", readonly)]
    pub clock_rate: f64,
    /// Hit window for approach rate i.e. TimePreempt in milliseconds.
    ///
    /// Only available for osu!standard and osu!catch.
    #[wasm_bindgen(js_name = "arHitWindow", readonly)]
    pub ar_hitwindow: Option<f64>,
    /// Perfect hit window for overall difficulty i.e. time to hit "Perfect" in
    /// milliseconds.
    ///
    /// Only available for osu!mania.
    #[wasm_bindgen(js_name = "odPerfectHitWindow", readonly)]
    pub od_perfect_hitwindow: Option<f64>,
    /// Great hit window for overall difficulty i.e. time to hit a 300 ("Great")
    /// in milliseconds.
    ///
    /// Only available for osu!standard, osu!taiko, and osu!mania.
    #[wasm_bindgen(js_name = "odGreatHitWindow", readonly)]
    pub od_great_hitwindow: Option<f64>,
    /// Good hit window for overall difficulty i.e. time to hit a 200 ("Good")
    /// in milliseconds.
    ///
    /// Only available for osu!mania.
    #[wasm_bindgen(js_name = "odGoodHitWindow", readonly)]
    pub od_good_hitwindow: Option<f64>,
    /// Ok hit window for overall difficulty i.e. time to hit a 100 ("Ok") in
    /// milliseconds.
    ///
    /// Only available for osu!standard, osu!taiko, and osu!mania.
    #[wasm_bindgen(js_name = "odOkHitWindow", readonly)]
    pub od_ok_hitwindow: Option<f64>,
    /// Meh hit window for overall difficulty i.e. time to hit a 50 ("Meh") in
    /// milliseconds.
    ///
    /// Only available for osu!standard and osu!mania.
    #[wasm_bindgen(js_name = "odMehHitWindow", readonly)]
    pub od_meh_hitwindow: Option<f64>,
}

impl From<BeatmapAttributes> for JsBeatmapAttributes {
    fn from(attrs: BeatmapAttributes) -> Self {
        let HitWindows {
            ar: ar_hitwindow,
            od_perfect,
            od_great,
            od_good,
            od_ok,
            od_meh,
        } = attrs.hit_windows();

        let AdjustedBeatmapAttributes { ar, cs, hp, od } = attrs.apply_clock_rate();

        Self {
            ar,
            base_ar: attrs.ar(),
            od,
            base_od: attrs.od(),
            cs,
            hp,
            clock_rate: attrs.clock_rate(),
            ar_hitwindow,
            od_perfect_hitwindow: od_perfect,
            od_great_hitwindow: od_great,
            od_good_hitwindow: od_good,
            od_ok_hitwindow: od_ok,
            od_meh_hitwindow: od_meh,
        }
    }
}
