use std::fmt::{Formatter, Result as FmtResult};

use rosu_pp::{
    any::{PerformanceAttributes, ScoreState},
    catch::CatchPerformanceAttributes,
    mania::ManiaPerformanceAttributes,
    osu::OsuPerformanceAttributes,
    taiko::TaikoPerformanceAttributes,
};
use serde::de;
use wasm_bindgen::prelude::*;

use crate::{score_state::JsScoreState, util::FieldVisitor};

use super::difficulty::JsDifficultyAttributes;

/// The result of a performance calculation.
#[wasm_bindgen(js_name = PerformanceAttributes, inspectable)]
#[derive(Default)]
pub struct JsPerformanceAttributes {
    /// The difficulty attributes.
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub difficulty: JsDifficultyAttributes,
    /// The hitresult score state that was used for performance calculation.
    ///
    /// Only available if *not* created through gradual calculation.
    #[wasm_bindgen(getter_with_clone, readonly)]
    pub state: Option<JsScoreState>,
    /// The final performance points.
    #[wasm_bindgen(readonly)]
    pub pp: f64,
    /// The aim portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "ppAim", readonly)]
    pub pp_aim: Option<f64>,
    /// The flashlight portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "ppFlashlight", readonly)]
    pub pp_flashlight: Option<f64>,
    /// The speed portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "ppSpeed", readonly)]
    pub pp_speed: Option<f64>,
    /// The accuracy portion of the final pp.
    ///
    /// Only available for osu! and osu!taiko.
    #[wasm_bindgen(js_name = "ppAccuracy", readonly)]
    pub pp_acc: Option<f64>,
    /// Scaled miss count based on total hits.
    ///
    /// Only available for osu! and osu!taiko.
    #[wasm_bindgen(js_name = "effectiveMissCount", readonly)]
    pub effective_miss_count: Option<f64>,
    /// The strain portion of the final pp.
    ///
    /// Only available for osu!taiko and osu!mania.
    #[wasm_bindgen(js_name = "ppDifficulty", readonly)]
    pub pp_difficulty: Option<f64>,
}

impl JsPerformanceAttributes {
    pub fn new(attrs: PerformanceAttributes, state: ScoreState) -> Self {
        let mut this = Self::from(attrs);
        this.state = Some(state.into());

        this
    }

    pub fn deserialize_difficulty<'de, D: de::Deserializer<'de>>(
        d: D,
    ) -> Result<JsDifficultyAttributes, D::Error> {
        struct DifficultyField;

        impl DifficultyField {
            const NAME: &'static str = "difficulty";
        }

        impl<'de> de::Deserialize<'de> for DifficultyField {
            fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                d.deserialize_str(FieldVisitor::new(Self::NAME))
                    .map(|_| Self)
            }
        }

        struct DifficultyVisitor;

        impl<'de> de::Visitor<'de> for DifficultyVisitor {
            type Value = JsDifficultyAttributes;

            fn expecting(&self, f: &mut Formatter) -> FmtResult {
                f.write_str("PerformanceAttributes")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                map.next_key::<DifficultyField>()?;

                map.next_value::<JsDifficultyAttributes>()
            }
        }

        d.deserialize_struct(
            "PerformanceAttributes",
            &[DifficultyField::NAME],
            DifficultyVisitor,
        )
    }
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
