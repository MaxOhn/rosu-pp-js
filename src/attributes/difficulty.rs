use rosu_pp::{
    any::DifficultyAttributes, catch::CatchDifficultyAttributes, mania::ManiaDifficultyAttributes,
    osu::OsuDifficultyAttributes, taiko::TaikoDifficultyAttributes,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{mode::JsGameMode, JsError};

/// The result of a difficulty calculation.
#[wasm_bindgen(js_name = DifficultyAttributes, inspectable)]
#[derive(Clone, Default, serde::Deserialize)]
#[serde(rename = "DifficultyAttributes", rename_all = "camelCase")]
pub struct JsDifficultyAttributes {
    /// The attributes' gamemode.
    #[wasm_bindgen(readonly)]
    pub mode: JsGameMode,
    /// The final star rating.
    #[wasm_bindgen(readonly)]
    pub stars: f64,
    /// Whether the map was a convert i.e. an osu! map.
    #[wasm_bindgen(js_name = "isConvert", readonly)]
    pub is_convert: bool,
    /// The difficulty of the aim skill.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub aim: Option<f64>,
    /// The difficulty of the speed skill.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub speed: Option<f64>,
    /// The difficulty of the flashlight skill.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub flashlight: Option<f64>,
    /// The ratio of the aim strain with and without considering sliders
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "sliderFactor", readonly)]
    pub slider_factor: Option<f64>,
    /// The number of clickable objects weighted by difficulty.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "speedNoteCount", readonly)]
    pub speed_note_count: Option<f64>,
    /// The overall difficulty
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub od: Option<f64>,
    /// The health drain rate.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub hp: Option<f64>,
    /// The amount of circles.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "nCircles", readonly)]
    pub n_circles: Option<u32>,
    /// The amount of sliders.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "nSliders", readonly)]
    pub n_sliders: Option<u32>,
    /// The amount of spinners.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "nSpinners", readonly)]
    pub n_spinners: Option<u32>,
    /// The difficulty of the stamina skill.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub stamina: Option<f64>,
    /// The difficulty of the rhythm skill.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub rhythm: Option<f64>,
    /// The difficulty of the color skill.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub color: Option<f64>,
    /// The difficulty of the hardest parts of the map.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub peak: Option<f64>,
    /// The amount of fruits.
    ///
    /// Only available for osu!catch.
    #[wasm_bindgen(js_name = "nFruits", readonly)]
    pub n_fruits: Option<u32>,
    /// The amount of droplets.
    ///
    /// Only available for osu!catch.
    #[wasm_bindgen(js_name = "nDroplets", readonly)]
    pub n_droplets: Option<u32>,
    /// The amount of tiny droplets.
    ///
    /// Only available for osu!catch.
    #[wasm_bindgen(js_name = "nTinyDroplets", readonly)]
    pub n_tiny_droplets: Option<u32>,
    /// The amount of hitobjects in the map.
    ///
    /// Only available for osu!mania.
    #[wasm_bindgen(js_name = "nObjects", readonly)]
    pub n_objects: Option<u32>,
    /// The approach rate.
    ///
    /// Only available for osu! and osu!catch.
    #[wasm_bindgen(readonly)]
    pub ar: Option<f64>,
    /// The perceived hit window for an n300 inclusive of rate-adjusting mods
    /// (DT/HT/etc)
    ///
    /// Only available for osu!taiko and osu!mania.
    #[wasm_bindgen(js_name = "hitWindow", readonly)]
    pub hit_window: Option<f64>,
    /// Return the maximum combo.
    #[wasm_bindgen(js_name = "maxCombo", readonly)]
    pub max_combo: u32,
}

impl From<OsuDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: OsuDifficultyAttributes) -> Self {
        let OsuDifficultyAttributes {
            aim,
            speed,
            flashlight,
            slider_factor,
            speed_note_count,
            ar,
            od,
            hp,
            n_circles,
            n_sliders,
            n_spinners,
            stars,
            max_combo,
        } = attrs;

        Self {
            mode: JsGameMode::Osu,
            stars,
            is_convert: false,
            aim: Some(aim),
            speed: Some(speed),
            flashlight: Some(flashlight),
            slider_factor: Some(slider_factor),
            speed_note_count: Some(speed_note_count),
            ar: Some(ar),
            od: Some(od),
            hp: Some(hp),
            n_circles: Some(n_circles),
            n_sliders: Some(n_sliders),
            n_spinners: Some(n_spinners),
            max_combo,
            ..Self::default()
        }
    }
}

impl From<TaikoDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: TaikoDifficultyAttributes) -> Self {
        let TaikoDifficultyAttributes {
            stamina,
            rhythm,
            color,
            peak,
            hit_window,
            stars,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: JsGameMode::Taiko,
            stars,
            is_convert,
            stamina: Some(stamina),
            rhythm: Some(rhythm),
            color: Some(color),
            peak: Some(peak),
            hit_window: Some(hit_window),
            max_combo,
            ..Self::default()
        }
    }
}

impl From<CatchDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: CatchDifficultyAttributes) -> Self {
        let max_combo = attrs.max_combo();

        let CatchDifficultyAttributes {
            stars,
            ar,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            is_convert,
        } = attrs;

        Self {
            mode: JsGameMode::Catch,
            stars,
            is_convert,
            ar: Some(ar),
            n_fruits: Some(n_fruits),
            n_droplets: Some(n_droplets),
            n_tiny_droplets: Some(n_tiny_droplets),
            max_combo,
            ..Self::default()
        }
    }
}

impl From<ManiaDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: ManiaDifficultyAttributes) -> Self {
        let ManiaDifficultyAttributes {
            stars,
            hit_window,
            n_objects,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: JsGameMode::Mania,
            stars,
            is_convert,
            hit_window: Some(hit_window),
            n_objects: Some(n_objects),
            max_combo,
            ..Self::default()
        }
    }
}

impl From<DifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: DifficultyAttributes) -> Self {
        match attrs {
            DifficultyAttributes::Osu(attrs) => attrs.into(),
            DifficultyAttributes::Taiko(attrs) => attrs.into(),
            DifficultyAttributes::Catch(attrs) => attrs.into(),
            DifficultyAttributes::Mania(attrs) => attrs.into(),
        }
    }
}

impl TryFrom<JsDifficultyAttributes> for DifficultyAttributes {
    type Error = JsError;

    fn try_from(attrs: JsDifficultyAttributes) -> Result<Self, Self::Error> {
        let JsDifficultyAttributes {
            mode,
            stars,
            is_convert,
            aim,
            speed,
            flashlight,
            slider_factor,
            speed_note_count,
            od,
            hp,
            n_circles,
            n_sliders,
            n_spinners,
            stamina,
            rhythm,
            color,
            peak,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            n_objects,
            ar,
            hit_window,
            max_combo,
        } = attrs;

        match mode {
            JsGameMode::Osu => {
                if let (
                    Some(aim),
                    Some(speed),
                    Some(flashlight),
                    Some(slider_factor),
                    Some(speed_note_count),
                    Some(ar),
                    Some(od),
                    Some(hp),
                    Some(n_circles),
                    Some(n_sliders),
                    Some(n_spinners),
                ) = (
                    aim,
                    speed,
                    flashlight,
                    slider_factor,
                    speed_note_count,
                    ar,
                    od,
                    hp,
                    n_circles,
                    n_sliders,
                    n_spinners,
                ) {
                    return Ok(Self::Osu(OsuDifficultyAttributes {
                        aim,
                        speed,
                        flashlight,
                        slider_factor,
                        speed_note_count,
                        ar,
                        od,
                        hp,
                        n_circles,
                        n_sliders,
                        n_spinners,
                        stars,
                        max_combo,
                    }));
                }
            }
            JsGameMode::Taiko => {
                if let (Some(stamina), Some(rhythm), Some(color), Some(peak), Some(hit_window)) =
                    (stamina, rhythm, color, peak, hit_window)
                {
                    return Ok(Self::Taiko(TaikoDifficultyAttributes {
                        stamina,
                        rhythm,
                        color,
                        peak,
                        hit_window,
                        stars,
                        max_combo,
                        is_convert,
                    }));
                }
            }
            JsGameMode::Catch => {
                if let (Some(ar), Some(n_fruits), Some(n_droplets), Some(n_tiny_droplets)) =
                    (ar, n_fruits, n_droplets, n_tiny_droplets)
                {
                    return Ok(Self::Catch(CatchDifficultyAttributes {
                        stars,
                        ar,
                        n_fruits,
                        n_droplets,
                        n_tiny_droplets,
                        is_convert,
                    }));
                }
            }
            JsGameMode::Mania => {
                if let (Some(hit_window), Some(n_objects)) = (hit_window, n_objects) {
                    return Ok(Self::Mania(ManiaDifficultyAttributes {
                        stars,
                        hit_window,
                        n_objects,
                        max_combo,
                        is_convert,
                    }));
                }
            }
        }

        Err(JsError::from("invalid difficulty attributes"))
    }
}
