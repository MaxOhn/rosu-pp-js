use rosu_pp::{
    any::{PerformanceAttributes as RosuPerformanceAttributes, ScoreState as RosuScoreState},
    catch::{CatchDifficultyAttributes, CatchPerformanceAttributes},
    mania::{ManiaDifficultyAttributes, ManiaPerformanceAttributes},
    osu::{OsuDifficultyAttributes, OsuPerformanceAttributes},
    taiko::{TaikoDifficultyAttributes, TaikoPerformanceAttributes},
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{mode::JsGameMode, score_state::ScoreState};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = PerformanceAttributes)]
    pub type JsPerformanceAttributes;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* The result of a performance calculation.
*/
interface PerformanceAttributes extends DifficultyAttributes {
    /**
    * The final performance points.
    */
    pp: number,
    /**
    * The aim portion of the final pp.
    *
    * Only available for osu!.
    */
    ppAim?: number,
    /**
    * The flashlight portion of the final pp.
    *
    * Only available for osu!.
    */
    ppFlashlight?: number,
    /**
    * The speed portion of the final pp.
    *
    * Only available for osu!.
    */
    ppSpeed?: number,
    /**
    * The accuracy portion of the final pp.
    *
    * Only available for osu! and osu!taiko.
    */
    ppAccuracy?: number,
    /**
    * Scaled miss count based on total hits.
    *
    * Only available for osu! and osu!taiko.
    */
    effectiveMissCount?: number,
    /**
    * The strain portion of the final pp.
    *
    * Only available for osu!taiko and osu!mania.
    */
    ppDifficulty?: number,
    /**
    * The hitresult score state that was used for performance calculation.
    *
    * Only available if *not* created through gradual calculation.
    */
    state?: ScoreState, 
}"#;

#[derive(Default, serde::Serialize)]
pub(crate) struct PerformanceAttributes {
    // serde_wasm_bindgen doesn't seem to like #[serde(flatten)] so we have
    // to add DifficultyAttributes' fields manually
    pub mode: JsGameMode,
    pub stars: f64,
    #[serde(rename = "isConvert")]
    pub is_convert: bool,
    pub aim: Option<f64>,
    pub speed: Option<f64>,
    pub flashlight: Option<f64>,
    #[serde(rename = "sliderFactor")]
    pub slider_factor: Option<f64>,
    #[serde(rename = "speedNoteCount")]
    pub speed_note_count: Option<f64>,
    pub od: Option<f64>,
    pub hp: Option<f64>,
    #[serde(rename = "nCircles")]
    pub n_circles: Option<u32>,
    #[serde(rename = "nSliders")]
    pub n_sliders: Option<u32>,
    #[serde(rename = "nSpinners")]
    pub n_spinners: Option<u32>,
    pub stamina: Option<f64>,
    pub rhythm: Option<f64>,
    pub color: Option<f64>,
    pub peak: Option<f64>,
    #[serde(rename = "nFruits")]
    pub n_fruits: Option<u32>,
    #[serde(rename = "nDroplets")]
    pub n_droplets: Option<u32>,
    #[serde(rename = "nTinyDroplets")]
    pub n_tiny_droplets: Option<u32>,
    #[serde(rename = "nObjects")]
    pub n_objects: Option<u32>,
    pub ar: Option<f64>,
    #[serde(rename = "hitWindow")]
    pub hit_window: Option<f64>,
    #[serde(rename = "maxCombo")]
    pub max_combo: u32,
    pub state: Option<ScoreState>,
    pub pp: f64,
    #[serde(rename = "ppAim")]
    pub pp_aim: Option<f64>,
    #[serde(rename = "ppFlashlight")]
    pub pp_flashlight: Option<f64>,
    #[serde(rename = "ppSpeed")]
    pub pp_speed: Option<f64>,
    #[serde(rename = "ppAccuracy")]
    pub pp_acc: Option<f64>,
    #[serde(rename = "effectiveMissCount")]
    pub effective_miss_count: Option<f64>,
    #[serde(rename = "ppDifficulty")]
    pub pp_difficulty: Option<f64>,
}

impl PerformanceAttributes {
    pub fn new(attrs: RosuPerformanceAttributes, state: RosuScoreState) -> Self {
        let mut this = Self::from(attrs);
        this.state = Some(state.into());

        this
    }
}

impl From<RosuPerformanceAttributes> for PerformanceAttributes {
    fn from(attrs: RosuPerformanceAttributes) -> Self {
        match attrs {
            RosuPerformanceAttributes::Osu(OsuPerformanceAttributes {
                difficulty:
                    OsuDifficultyAttributes {
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
                    },
                pp,
                pp_acc,
                pp_aim,
                pp_flashlight,
                pp_speed,
                effective_miss_count,
            }) => Self {
                mode: JsGameMode::Osu,
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
                stars,
                max_combo,
                pp,
                pp_acc: Some(pp_acc),
                pp_aim: Some(pp_aim),
                pp_flashlight: Some(pp_flashlight),
                pp_speed: Some(pp_speed),
                effective_miss_count: Some(effective_miss_count),
                ..Self::default()
            },
            RosuPerformanceAttributes::Taiko(TaikoPerformanceAttributes {
                difficulty:
                    TaikoDifficultyAttributes {
                        stamina,
                        rhythm,
                        color,
                        peak,
                        hit_window,
                        stars,
                        max_combo,
                        is_convert,
                    },
                pp,
                pp_acc,
                pp_difficulty,
                effective_miss_count,
            }) => Self {
                mode: JsGameMode::Taiko,
                stamina: Some(stamina),
                rhythm: Some(rhythm),
                color: Some(color),
                peak: Some(peak),
                hit_window: Some(hit_window),
                stars,
                max_combo,
                is_convert,
                pp,
                pp_acc: Some(pp_acc),
                pp_difficulty: Some(pp_difficulty),
                effective_miss_count: Some(effective_miss_count),
                ..Self::default()
            },
            RosuPerformanceAttributes::Catch(CatchPerformanceAttributes { difficulty, pp }) => {
                let max_combo = difficulty.max_combo();

                let CatchDifficultyAttributes {
                    stars,
                    ar,
                    n_fruits,
                    n_droplets,
                    n_tiny_droplets,
                    is_convert,
                } = difficulty;

                Self {
                    mode: JsGameMode::Catch,
                    stars,
                    ar: Some(ar),
                    n_fruits: Some(n_fruits),
                    n_droplets: Some(n_droplets),
                    n_tiny_droplets: Some(n_tiny_droplets),
                    max_combo,
                    is_convert,
                    pp,
                    ..Self::default()
                }
            }
            RosuPerformanceAttributes::Mania(ManiaPerformanceAttributes {
                difficulty:
                    ManiaDifficultyAttributes {
                        stars,
                        hit_window,
                        n_objects,
                        max_combo,
                        is_convert,
                    },
                pp,
                pp_difficulty,
            }) => Self {
                mode: JsGameMode::Mania,
                stars,
                hit_window: Some(hit_window),
                n_objects: Some(n_objects),
                max_combo,
                is_convert,
                pp,
                pp_difficulty: Some(pp_difficulty),
                ..Self::default()
            },
        }
    }
}
