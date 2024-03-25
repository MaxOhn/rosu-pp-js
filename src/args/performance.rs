use rosu_pp::{
    any::{DifficultyAttributes as RosuDifficultyAttributes, HitResultPriority},
    Performance,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{
    attributes::difficulty::DifficultyAttributes,
    util::{self, JsValueExt, ObjectExt},
    JsError, JsResult,
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
interface PerformanceArgs extends DifficultyArgs {
    /**
    * Set the accuracy between `0.0` and `100.0`.
    */
    accuracy?: number,
    /**
    * Specify the max combo of the play.
    *
    * Irrelevant for osu!mania.
    */
    combo?: number,
    /**
    * Specify the amount of gekis of a play.
    *
    * Only relevant for osu!mania for which it repesents the amount of n320.
    */
    nGeki?: number,
    /**
    * Specify the amount of katus of a play.
    *
    * Only relevant for osu!catch for which it represents the amount of tiny
    * droplet misses and osu!mania for which it repesents the amount of n200.
    */
    nKatu?: number,
    /**
    * Specify the amount of 300s of a play.
    */
    n300?: number,
    /**
    * Specify the amount of 100s of a play.
    */
    n100?: number,
    /**
    * Specify the amount of 50s of a play.
    *
    * Irrelevant for osu!taiko.
    */
    n50?: number,
    /**
    * Specify the amount of misses of a play.
    */
    misses?: number,
    /**
    * Specify how hitresults should be generated.
    *
    * Defaults to `HitResultPriority.BestCase`.
    */
    hitresultPriority?: HitResultPriority,
}"#;

#[derive(Default)]
pub struct PerformanceArgs {
    pub difficulty: DifficultyArgs,
    pub accuracy: Option<f64>,
    pub combo: Option<u32>,
    pub n_geki: Option<u32>,
    pub n_katu: Option<u32>,
    pub n300: Option<u32>,
    pub n100: Option<u32>,
    pub n50: Option<u32>,
    pub misses: Option<u32>,
    pub hitresult_priority: HitResultPriority,
}

from_jsvalue! {
    PerformanceArgs {
        accuracy as accuracy: f64?,
        combo as combo: u32?,
        n_geki as nGeki: u32?,
        n_katu as nKatu: u32?,
        n300 as n300: u32?,
        n100 as n100: u32?,
        n50 as n50: u32?,
        misses as misses: u32?,
    }
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

impl PerformanceArgs {
    pub fn from_value(value: &JsPerformanceArgs) -> JsResult<Self> {
        let mut this = util::from_value::<Self>(value)?;

        let obj = value.unchecked_ref::<ObjectExt>();
        let js_field = util::static_str_to_js("hitresultPriority");
        let js_value = obj.get_with_ref_key(&js_field);

        if !js_value.is_undefined() {
            match js_value.as_safe_integer() {
                Some(0) => this.hitresult_priority = HitResultPriority::BestCase,
                Some(1) => this.hitresult_priority = HitResultPriority::WorstCase,
                _ => return Err(JsError::new("invalid hitresultPriority")),
            }
        }

        this.difficulty = DifficultyArgs::from_value(value.unchecked_ref())?;

        Ok(this)
    }

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

        perf.hitresult_priority(self.hitresult_priority)
            .difficulty(self.difficulty.as_difficulty())
    }
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Either previously calculated attributes or a beatmap.
*/
type MapOrAttributes = DifficultyAttributes | Beatmap;"#;

pub enum MapOrAttrs<'a> {
    Map(&'a JsValue),
    Attrs(RosuDifficultyAttributes),
}

impl<'a> MapOrAttrs<'a> {
    pub fn from_value(value: &'a JsValue) -> Self {
        if let Some(Ok(attrs)) = value.dyn_ref().map(DifficultyAttributes::from_value) {
            Self::Attrs(attrs)
        } else {
            Self::Map(value)
        }
    }
}
