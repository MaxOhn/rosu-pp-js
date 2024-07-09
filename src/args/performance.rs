use rosu_pp::{
    any::{DifficultyAttributes, HitResultPriority},
    Performance,
};
use serde::de;
use wasm_bindgen::{__rt::RefMut, prelude::wasm_bindgen, JsValue};

use crate::{
    attributes::{difficulty::JsDifficultyAttributes, performance::JsPerformanceAttributes},
    beatmap::JsBeatmap,
    deserializer::JsDeserializer,
    util, JsError, JsResult,
};

use super::difficulty::DifficultyArgs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = PerformanceArgs)]
    pub type JsPerformanceArgs;

    #[wasm_bindgen(typescript_type = "MapOrAttributes")]
    pub type JsMapOrAttributes;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `Performance` constructor.
*/
export interface PerformanceArgs extends DifficultyArgs {
    /**
    * Set the accuracy between `0.0` and `100.0`.
    */
    accuracy?: number;
    /**
    * Specify the max combo of the play.
    *
    * Irrelevant for osu!mania.
    */
    combo?: number;
    /**
    * Specify the amount of gekis of a play.
    *
    * Only relevant for osu!mania for which it repesents the amount of n320.
    */
    nGeki?: number;
    /**
    * Specify the amount of katus of a play.
    *
    * Only relevant for osu!catch for which it represents the amount of tiny
    * droplet misses and osu!mania for which it repesents the amount of n200.
    */
    nKatu?: number;
    /**
    * Specify the amount of 300s of a play.
    */
    n300?: number;
    /**
    * Specify the amount of 100s of a play.
    */
    n100?: number;
    /**
    * Specify the amount of 50s of a play.
    *
    * Irrelevant for osu!taiko.
    */
    n50?: number;
    /**
    * Specify the amount of misses of a play.
    */
    misses?: number;
    /**
    * Specify how hitresults should be generated.
    *
    * Defaults to `HitResultPriority.BestCase`.
    */
    hitresultPriority?: HitResultPriority;
}"#;

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase", rename = "Object")]
pub struct PerformanceArgs {
    #[serde(default)]
    pub mods: u32,
    pub clock_rate: Option<f64>,
    pub ar: Option<f32>,
    #[serde(default)]
    pub ar_with_mods: bool,
    pub cs: Option<f32>,
    #[serde(default)]
    pub cs_with_mods: bool,
    pub hp: Option<f32>,
    #[serde(default)]
    pub hp_with_mods: bool,
    pub od: Option<f32>,
    #[serde(default)]
    pub od_with_mods: bool,
    pub passed_objects: Option<u32>,
    pub hardrock_offsets: Option<bool>,
    pub accuracy: Option<f64>,
    pub combo: Option<u32>,
    pub n_geki: Option<u32>,
    pub n_katu: Option<u32>,
    pub n300: Option<u32>,
    pub n100: Option<u32>,
    pub n50: Option<u32>,
    pub misses: Option<u32>,
    #[serde(default, deserialize_with = "JsHitResultPriority::deserialize")]
    pub hitresult_priority: HitResultPriority,
}

/// While generating remaining hitresults, decide how they should be distributed.
#[wasm_bindgen(js_name = HitResultPriority)]
#[derive(Copy, Clone)]
pub enum JsHitResultPriority {
    /// Prioritize good hitresults over bad ones
    BestCase,
    /// Prioritize bad hitresults over good ones
    WorstCase,
}

impl From<JsHitResultPriority> for HitResultPriority {
    fn from(priority: JsHitResultPriority) -> Self {
        match priority {
            JsHitResultPriority::BestCase => Self::BestCase,
            JsHitResultPriority::WorstCase => Self::WorstCase,
        }
    }
}

impl JsHitResultPriority {
    fn deserialize<'de, D: de::Deserializer<'de>>(d: D) -> Result<HitResultPriority, D::Error> {
        let priority = match <u8 as de::Deserialize>::deserialize(d) {
            Ok(0) => HitResultPriority::BestCase,
            Ok(1) => HitResultPriority::WorstCase,
            _ => return Err(de::Error::custom("invalid HitResultPriority")),
        };

        Ok(priority)
    }
}

impl PerformanceArgs {
    pub fn apply<'a>(&self, mut perf: Performance<'a>) -> Performance<'a> {
        if let Some(accuracy) = self.accuracy {
            perf = perf.accuracy(accuracy);
        }

        if let Some(combo) = self.combo {
            perf = perf.combo(combo);
        }

        if let Some(n_geki) = self.n_geki {
            perf = perf.n_geki(n_geki);
        }

        if let Some(n_katu) = self.n_katu {
            perf = perf.n_katu(n_katu);
        }

        if let Some(n300) = self.n300 {
            perf = perf.n300(n300);
        }

        if let Some(n100) = self.n100 {
            perf = perf.n100(n100);
        }

        if let Some(n50) = self.n50 {
            perf = perf.n50(n50);
        }

        if let Some(misses) = self.misses {
            perf = perf.misses(misses);
        }

        let difficulty = DifficultyArgs {
            mods: self.mods,
            clock_rate: self.clock_rate,
            ar: self.ar,
            ar_with_mods: self.ar_with_mods,
            cs: self.cs,
            cs_with_mods: self.cs_with_mods,
            hp: self.hp,
            hp_with_mods: self.hp_with_mods,
            od: self.od,
            od_with_mods: self.od_with_mods,
            passed_objects: self.passed_objects,
            hardrock_offsets: self.hardrock_offsets,
        };

        perf.hitresult_priority(self.hitresult_priority)
            .difficulty(difficulty.as_difficulty())
    }
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Either previously calculated attributes or a beatmap.
*/
export type MapOrAttributes = DifficultyAttributes | PerformanceAttributes | Beatmap;"#;

pub enum MapOrAttrs {
    Map(RefMut<'static, JsBeatmap>),
    Attrs(DifficultyAttributes),
}

impl MapOrAttrs {
    pub fn from_value(value: &JsValue) -> JsResult<Self> {
        if let Ok(js_attrs) =
            JsPerformanceAttributes::deserialize_difficulty(JsDeserializer::from_ref(value))
        {
            return js_attrs.try_into().map(Self::Attrs);
        }

        if let Ok(js_attrs) = util::from_value::<JsDifficultyAttributes>(value) {
            return js_attrs.try_into().map(Self::Attrs);
        }

        if let Ok(map) = JsBeatmap::deserialize(JsDeserializer::from_ref(value)) {
            return Ok(Self::Map(map));
        }

        Err(JsError::from(
            "Expected either previously calculated attributes or a beatmap",
        ))
    }
}
