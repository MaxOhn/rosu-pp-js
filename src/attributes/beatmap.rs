use rosu_pp::model::beatmap::{BeatmapAttributes, BeatmapAttributesBuilder};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::beatmap::{BeatmapAttributesArgs, JsBeatmapAttributesArgs},
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
    pub fn new(args: Option<JsBeatmapAttributesArgs>) -> JsResult<JsBeatmapAttributesBuilder> {
        let inner = if let Some(ref args) = args {
            util::from_value::<BeatmapAttributesArgs>(args)
                .and_then(BeatmapAttributesBuilder::try_from)?
        } else {
            BeatmapAttributesBuilder::new()
        };

        Ok(Self { inner })
    }

    /// Calculate the `BeatmapAttributes`.
    pub fn build(&self) -> JsBeatmapAttributes {
        self.inner.build().into()
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
    #[wasm_bindgen(js_name = "odHitWindow", readonly)]
    pub od_hitwindow: f64,
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
            od_hitwindow: attrs.hit_windows.od,
        }
    }
}

impl TryFrom<BeatmapAttributesArgs> for BeatmapAttributesBuilder {
    type Error = JsError;

    fn try_from(args: BeatmapAttributesArgs) -> Result<Self, Self::Error> {
        let mut builder = Self::new();

        if let Some(map) = args.map {
            builder = builder.map(&map.inner);
        }

        if let Some(mode) = args.mode {
            builder = builder.mode(mode.into(), args.is_convert);
        }

        if let Some(clock_rate) = args.clock_rate {
            builder = builder.clock_rate(clock_rate);
        }

        if let Some(ar) = args.ar {
            builder = builder.ar(ar, args.ar_with_mods);
        }

        if let Some(cs) = args.cs {
            builder = builder.cs(cs, args.cs_with_mods);
        }

        if let Some(hp) = args.hp {
            builder = builder.hp(hp, args.hp_with_mods);
        }

        if let Some(od) = args.od {
            builder = builder.od(od, args.od_with_mods);
        }

        Ok(builder.mods(args.mods))
    }
}
