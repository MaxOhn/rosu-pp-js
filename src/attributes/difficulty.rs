use rosu_pp::{
    any::DifficultyAttributes, catch::CatchDifficultyAttributes, mania::ManiaDifficultyAttributes,
    osu::OsuDifficultyAttributes, taiko::TaikoDifficultyAttributes,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{JsError, mode::JsGameMode};

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
    /// The number of sliders weighted by difficulty.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub aim_difficult_slider_count: Option<f64>,
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
    /// Weighted sum of aim strains.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "aimDifficultStrainCount", readonly)]
    pub aim_difficult_strain_count: Option<f64>,
    /// Weighted sum of speed strains.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "speedDifficultStrainCount", readonly)]
    pub speed_difficult_strain_count: Option<f64>,
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
    /// The amount of "large ticks".
    ///
    /// The meaning depends on the kind of score:
    /// - if set on osu!stable, this value is irrelevant
    /// - if set on osu!lazer *with* slider accuracy, this value is the amount
    ///   of hit slider ticks and repeats
    /// - if set on osu!lazer *without* slider accuracy, this value is the
    ///   amount of hit slider heads, ticks, and repeats
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "nLargeTicks", readonly)]
    pub n_large_ticks: Option<u32>,
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
    /// The difficulty of the reading skill.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub reading: Option<f64>,
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
    /// The amount of hold notes in the map.
    ///
    /// Only available for osu!mania.
    #[wasm_bindgen(js_name = "nHoldNotes", readonly)]
    pub n_hold_notes: Option<u32>,
    /// The approach rate.
    ///
    /// Only available for osu! and osu!catch.
    #[wasm_bindgen(readonly)]
    pub ar: Option<f64>,
    /// The perceived hit window for an n300 inclusive of rate-adjusting mods
    /// (DT/HT/etc)
    ///
    /// Only available for osu! and osu!taiko.
    #[wasm_bindgen(js_name = "greatHitWindow", readonly)]
    pub great_hit_window: Option<f64>,
    /// The perceived hit window for an n100 inclusive of rate-adjusting mods
    /// (DT/HT/etc)
    ///
    /// Only available for osu! and osu!taiko.
    #[wasm_bindgen(js_name = "okHitWindow", readonly)]
    pub ok_hit_window: Option<f64>,
    /// The perceived hit window for an n50 inclusive of rate-adjusting mods
    /// (DT/HT/etc)
    ///
    /// Only available for osu!.
    #[wasm_bindgen(js_name = "mehHitWindow", readonly)]
    pub meh_hit_window: Option<f64>,
    /// The ratio of stamina difficulty from mono-color (single color) streams to total
    /// stamina difficulty.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(js_name = "monoStaminaFactor", readonly)]
    pub mono_stamina_factor: Option<f64>,
    /// Return the maximum combo.
    #[wasm_bindgen(js_name = "maxCombo", readonly)]
    pub max_combo: u32,
}

impl From<OsuDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: OsuDifficultyAttributes) -> Self {
        let OsuDifficultyAttributes {
            aim,
            aim_difficult_slider_count,
            speed,
            flashlight,
            slider_factor,
            speed_note_count,
            aim_difficult_strain_count,
            speed_difficult_strain_count,
            ar,
            great_hit_window,
            ok_hit_window,
            meh_hit_window,
            hp,
            n_circles,
            n_sliders,
            n_large_ticks,
            n_spinners,
            stars,
            max_combo,
        } = attrs;

        Self {
            mode: JsGameMode::Osu,
            stars,
            is_convert: false,
            aim: Some(aim),
            aim_difficult_slider_count: Some(aim_difficult_slider_count),
            speed: Some(speed),
            flashlight: Some(flashlight),
            slider_factor: Some(slider_factor),
            speed_note_count: Some(speed_note_count),
            aim_difficult_strain_count: Some(aim_difficult_strain_count),
            speed_difficult_strain_count: Some(speed_difficult_strain_count),
            ar: Some(ar),
            great_hit_window: Some(great_hit_window),
            ok_hit_window: Some(ok_hit_window),
            meh_hit_window: Some(meh_hit_window),
            hp: Some(hp),
            n_circles: Some(n_circles),
            n_sliders: Some(n_sliders),
            n_large_ticks: Some(n_large_ticks),
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
            reading,
            rhythm,
            color,
            great_hit_window,
            ok_hit_window,
            mono_stamina_factor,
            stars,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: JsGameMode::Taiko,
            stars,
            is_convert,
            stamina: Some(stamina),
            reading: Some(reading),
            rhythm: Some(rhythm),
            color: Some(color),
            great_hit_window: Some(great_hit_window),
            ok_hit_window: Some(ok_hit_window),
            mono_stamina_factor: Some(mono_stamina_factor),
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
            n_objects,
            n_hold_notes,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: JsGameMode::Mania,
            stars,
            is_convert,
            n_objects: Some(n_objects),
            n_hold_notes: Some(n_hold_notes),
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
            aim_difficult_slider_count,
            speed,
            flashlight,
            slider_factor,
            speed_note_count,
            aim_difficult_strain_count,
            speed_difficult_strain_count,
            hp,
            n_circles,
            n_sliders,
            n_large_ticks,
            n_spinners,
            stamina,
            reading,
            rhythm,
            color,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            n_objects,
            n_hold_notes,
            ar,
            great_hit_window,
            ok_hit_window,
            meh_hit_window,
            mono_stamina_factor,
            max_combo,
        } = attrs;

        match mode {
            JsGameMode::Osu => {
                if let (
                    Some(aim),
                    Some(aim_difficult_slider_count),
                    Some(speed),
                    Some(flashlight),
                    Some(slider_factor),
                    Some(speed_note_count),
                    Some(aim_difficult_strain_count),
                    Some(speed_difficult_strain_count),
                    Some(ar),
                    Some(great_hit_window),
                    Some(ok_hit_window),
                    Some(meh_hit_window),
                    Some(hp),
                    Some(n_circles),
                    Some(n_sliders),
                    Some(n_large_ticks),
                    Some(n_spinners),
                ) = (
                    aim,
                    aim_difficult_slider_count,
                    speed,
                    flashlight,
                    slider_factor,
                    speed_note_count,
                    aim_difficult_strain_count,
                    speed_difficult_strain_count,
                    ar,
                    great_hit_window,
                    ok_hit_window,
                    meh_hit_window,
                    hp,
                    n_circles,
                    n_sliders,
                    n_large_ticks,
                    n_spinners,
                ) {
                    return Ok(Self::Osu(OsuDifficultyAttributes {
                        aim,
                        aim_difficult_slider_count,
                        speed,
                        flashlight,
                        slider_factor,
                        speed_note_count,
                        aim_difficult_strain_count,
                        speed_difficult_strain_count,
                        ar,
                        great_hit_window,
                        ok_hit_window,
                        meh_hit_window,
                        hp,
                        n_circles,
                        n_sliders,
                        n_large_ticks,
                        n_spinners,
                        stars,
                        max_combo,
                    }));
                }
            }
            JsGameMode::Taiko => {
                if let (
                    Some(stamina),
                    Some(reading),
                    Some(rhythm),
                    Some(color),
                    Some(great_hit_window),
                    Some(ok_hit_window),
                    Some(mono_stamina_factor),
                ) = (
                    stamina,
                    reading,
                    rhythm,
                    color,
                    great_hit_window,
                    ok_hit_window,
                    mono_stamina_factor,
                ) {
                    return Ok(Self::Taiko(TaikoDifficultyAttributes {
                        stamina,
                        reading,
                        rhythm,
                        color,
                        great_hit_window,
                        ok_hit_window,
                        mono_stamina_factor,
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
                if let (Some(n_objects), Some(n_hold_notes)) = (n_objects, n_hold_notes) {
                    return Ok(Self::Mania(ManiaDifficultyAttributes {
                        stars,
                        n_objects,
                        n_hold_notes,
                        max_combo,
                        is_convert,
                    }));
                }
            }
        }

        Err(JsError::new("invalid difficulty attributes"))
    }
}
