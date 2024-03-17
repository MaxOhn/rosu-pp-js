use rosu_pp::any::{DifficultyAttributes as RosuDifficultyAttributes, HitResultPriority};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

use crate::{
    attributes::difficulty::DifficultyAttributes,
    util::{self, FromJsValue, JsValueExt, ObjectExt},
    JsError, JsResult,
};

use super::difficulty::DifficultyArgs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = PerformanceArgs)]
    pub type JsPerformanceArgs;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `Performance` constructor.
*/
interface PerformanceArgs extends DifficultyArgs {
    /**
    * Specify a beatmap.
    *
    * Either this or `attributes` *must* be specified.
    *
    * Note that creating `PerformanceAttributes` this way will require to
    * perform the costly computation of difficulty attributes internally. If
    * attributes for the current `DifficultyArgs` settings are already
    * available, consider specifying `attributes` instead.
    */
    map?: Beatmap,
    /**
    * Specify previously calculated difficulty attributes.
    *
    * Either this or `map` *must* be specified.
    *
    * Note that the given attributes must have been calculated for the same
    * beatmap and `DifficultyArgs` settings, otherwise the final attributes
    * will be incorrect.
    */
    attributes?: DifficultyAttributes,
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

#[derive(Debug, Default)]
pub enum MapOrAttrs {
    Map(JsValue),
    Attrs(RosuDifficultyAttributes),
    #[default]
    Neither,
}

#[derive(Debug, Default)]
pub struct PerformanceArgs {
    pub map_or_attrs: MapOrAttrs,
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

/// While generating remaining hitresults, decide how they should be distributed.
#[wasm_bindgen(js_name = HitResultPriority)]
#[derive(Copy, Clone)]
pub enum JsHitResultPriority {
    /// Prioritize good hitresults over bad ones
    BestCase,
    /// Prioritize bad hitresults over good ones
    WorstCase,
}

impl FromJsValue for PerformanceArgs {
    const FIELDS: &'static [&'static str] = &[
        "attributes",
        "accuracy",
        "combo",
        "nGeki",
        "nKatu",
        "n300",
        "n100",
        "n50",
        "misses",
        "hitresultPriority",
    ];

    fn field(&mut self, name: &str, value: JsValue) -> JsResult<()> {
        match name {
            "attributes" => {
                let attrs = DifficultyAttributes::from_value(value.unchecked_ref())?;
                self.map_or_attrs = MapOrAttrs::Attrs(attrs)
            }
            "accuracy" => match value.as_f64() {
                Some(acc) => self.accuracy = Some(acc),
                None => return Err(JsError::new("invalid accuracy")),
            },
            "combo" => match value.as_safe_integer().map(TryFrom::try_from) {
                Some(Ok(combo)) => self.combo = Some(combo),
                _ => return Err(JsError::new("invalid combo")),
            },
            "nGeki" => match value.as_safe_integer().map(TryFrom::try_from) {
                Some(Ok(n_geki)) => self.n_geki = Some(n_geki),
                _ => return Err(JsError::new("invalid nGeki")),
            },
            "nKatu" => match value.as_safe_integer().map(TryFrom::try_from) {
                Some(Ok(n_katu)) => self.n_katu = Some(n_katu),
                _ => return Err(JsError::new("invalid nKatu")),
            },
            "n300" => match value.as_safe_integer().map(TryFrom::try_from) {
                Some(Ok(n300)) => self.n300 = Some(n300),
                _ => return Err(JsError::new("invalid n300")),
            },
            "n100" => match value.as_safe_integer().map(TryFrom::try_from) {
                Some(Ok(n100)) => self.n100 = Some(n100),
                _ => return Err(JsError::new("invalid n100")),
            },
            "n50" => match value.as_safe_integer().map(TryFrom::try_from) {
                Some(Ok(n50)) => self.n50 = Some(n50),
                _ => return Err(JsError::new("invalid n50")),
            },
            "misses" => match value.as_safe_integer().map(TryFrom::try_from) {
                Some(Ok(misses)) => self.misses = Some(misses),
                _ => return Err(JsError::new("invalid misses")),
            },
            "hitresultPriority" => match value.as_safe_integer() {
                Some(0) => self.hitresult_priority = HitResultPriority::BestCase,
                Some(1) => self.hitresult_priority = HitResultPriority::WorstCase,
                _ => return Err(JsError::new("invalid hitresultPriority")),
            },
            _ => unreachable!(),
        }

        Ok(())
    }
}

impl PerformanceArgs {
    pub fn from_value(value: &JsPerformanceArgs) -> JsResult<Self> {
        let mut this = util::from_value::<Self>(value)?;

        if let MapOrAttrs::Neither = this.map_or_attrs {
            let obj = value.unchecked_ref::<ObjectExt>();
            let js_field = util::static_str_to_js("map");
            let js_value = obj.get_with_ref_key(&js_field);

            if !js_value.is_undefined() {
                this.map_or_attrs = MapOrAttrs::Map(js_value);
            }
        }

        this.difficulty = DifficultyArgs::from_value(value.unchecked_ref())?;

        Ok(this)
    }
}
