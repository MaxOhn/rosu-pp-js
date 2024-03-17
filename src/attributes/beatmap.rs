use rosu_pp::model::beatmap::{
    BeatmapAttributes as RosuBeatmapAttributes, BeatmapAttributesBuilder,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::beatmap::{BeatmapAttributesArgs, JsBeatmapAttributesArgs},
    beatmap::JsBeatmap,
    util, JsError, JsResult,
};

#[wasm_bindgen(js_name = BeatmapAttributesBuilder)]
pub struct JsBeatmapAttributesBuilder {
    inner: BeatmapAttributesBuilder,
}

#[wasm_bindgen(js_class = BeatmapAttributesBuilder)]
impl JsBeatmapAttributesBuilder {
    /// Create a new `BeatmapAttributesBuilder`.
    #[wasm_bindgen(constructor)]
    pub fn new(args: &JsBeatmapAttributesArgs) -> JsResult<JsBeatmapAttributesBuilder> {
        let inner =
            BeatmapAttributesArgs::from_value(args).and_then(BeatmapAttributesBuilder::try_from)?;

        Ok(Self { inner })
    }

    /// Calculate the `BeatmapAttributes`.
    pub fn build(&self) -> JsResult<JsBeatmapAttributes> {
        let attrs = BeatmapAttributes::from(self.inner.build());

        util::to_value(&attrs).map(From::from)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = BeatmapAttributes)]
    pub type JsBeatmapAttributes;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* The result of building a `BeatmapAttributesBuilder`.
*/
interface BeatmapAttributes {
    /**
    * The approach rate.
    */
    ar: number,
    /**
    * The overall difficulty.
    */
    od: number,
    /**
    * The circle size.
    */
    cs: number,
    /**
    * The health drain rate
    */
    hp: number,
    /**
    * The clock rate with respect to mods.
    */
    clockRate: number,
    /**
    * Hit window for approach rate i.e. TimePreempt in milliseconds.
    */
    arHitWindow: number,
    /**
    * Hit window for overall difficulty i.e. time to hit a 300 ("Great") in
    * milliseconds.
    */
    odHitWindow: number,
}"#;

#[derive(serde::Serialize)]
pub(crate) struct BeatmapAttributes {
    /// The approach rate.
    pub ar: f64,
    /// The overall difficulty.
    pub od: f64,
    /// The circle size.
    pub cs: f64,
    /// The health drain rate
    pub hp: f64,
    /// The clock rate with respect to mods.
    #[serde(rename = "clockRate")]
    pub clock_rate: f64,
    /// Hit window for approach rate i.e. TimePreempt in milliseconds.
    #[serde(rename = "arHitWindow")]
    pub ar_hitwindow: f64,
    /// Hit window for overall difficulty i.e. time to hit a 300 ("Great") in
    /// milliseconds.
    #[serde(rename = "odHitWindow")]
    pub od_hitwindow: f64,
}

impl From<RosuBeatmapAttributes> for BeatmapAttributes {
    fn from(attrs: RosuBeatmapAttributes) -> Self {
        Self {
            ar: attrs.ar,
            od: attrs.od,
            cs: attrs.cs,
            hp: attrs.hp,
            clock_rate: attrs.clock_rate,
            ar_hitwindow: attrs.hit_windows.ar,
            od_hitwindow: attrs.hit_windows.od,
        }
    }
}

impl TryFrom<BeatmapAttributesArgs> for BeatmapAttributesBuilder {
    type Error = JsError;

    fn try_from(args: BeatmapAttributesArgs) -> Result<Self, Self::Error> {
        let mut builder = if let Some(value) = args.map {
            let map = JsBeatmap::try_from_value(&value)?;

            Self::from_map(&map.inner)
        } else {
            Self::new()
        };

        if let Some(mode) = args.mode {
            builder = builder.mode(mode.into(), args.is_convert);
        }

        if let Some(clock_rate) = args.common.clock_rate {
            builder = builder.clock_rate(clock_rate);
        }

        if let Some(ar) = args.common.ar {
            builder = builder.ar(ar, args.common.ar_with_mods);
        }

        if let Some(cs) = args.common.cs {
            builder = builder.cs(cs, args.common.cs_with_mods);
        }

        if let Some(hp) = args.common.hp {
            builder = builder.hp(hp, args.common.hp_with_mods);
        }

        if let Some(od) = args.common.od {
            builder = builder.od(od, args.common.od_with_mods);
        }

        Ok(builder.mods(args.common.mods))
    }
}
