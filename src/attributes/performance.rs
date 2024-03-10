use rosu_pp::{
    any::PerformanceAttributes, catch::CatchPerformanceAttributes,
    mania::ManiaPerformanceAttributes, osu::OsuPerformanceAttributes,
    taiko::TaikoPerformanceAttributes,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::attributes::difficulty::JsDifficultyAttributes;

#[wasm_bindgen(js_name = PerformanceAttributes, inspectable)]
#[derive(Debug, Default)]
pub struct JsPerformanceAttributes {
    /// The difficulty attributes that were used for the performance
    /// calculation.
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub difficulty: JsDifficultyAttributes,

    /// The final performance points.
    #[wasm_bindgen(readonly)]
    pub pp: f64,

    /// The aim portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub pp_aim: Option<f64>,

    /// The flashlight portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub pp_flashlight: Option<f64>,

    /// The speed portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub pp_speed: Option<f64>,

    /// The accuracy portion of the final pp.
    ///
    /// Only available for osu! and osu!taiko.
    #[wasm_bindgen(readonly)]
    pub pp_acc: Option<f64>,

    /// Scaled miss count based on total hits.
    ///
    /// Only available for osu! and osu!taiko.
    #[wasm_bindgen(readonly)]
    pub effective_miss_count: Option<f64>,

    /// The strain portion of the final pp.
    ///
    /// Only available for osu!taiko and osu!mania.
    #[wasm_bindgen(readonly)]
    pub pp_difficulty: Option<f64>,
}

impl From<PerformanceAttributes> for JsPerformanceAttributes {
    fn from(attrs: PerformanceAttributes) -> Self {
        match attrs {
            PerformanceAttributes::Osu(OsuPerformanceAttributes {
                difficulty,
                pp,
                pp_acc,
                pp_aim,
                pp_flashlight,
                pp_speed,
                effective_miss_count,
            }) => Self {
                difficulty: difficulty.into(),
                pp,
                pp_acc: Some(pp_acc),
                pp_aim: Some(pp_aim),
                pp_flashlight: Some(pp_flashlight),
                pp_speed: Some(pp_speed),
                effective_miss_count: Some(effective_miss_count),
                ..Self::default()
            },
            PerformanceAttributes::Taiko(TaikoPerformanceAttributes {
                difficulty,
                pp,
                pp_acc,
                pp_difficulty,
                effective_miss_count,
            }) => Self {
                difficulty: difficulty.into(),
                pp,
                pp_acc: Some(pp_acc),
                pp_difficulty: Some(pp_difficulty),
                effective_miss_count: Some(effective_miss_count),
                ..Self::default()
            },
            PerformanceAttributes::Catch(CatchPerformanceAttributes { difficulty, pp }) => Self {
                difficulty: difficulty.into(),
                pp,
                ..Self::default()
            },
            PerformanceAttributes::Mania(ManiaPerformanceAttributes {
                difficulty,
                pp,
                pp_difficulty,
            }) => Self {
                difficulty: difficulty.into(),
                pp,
                pp_difficulty: Some(pp_difficulty),
                ..Self::default()
            },
        }
    }
}
