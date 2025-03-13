use rosu_pp::{
    any::Strains, catch::CatchStrains, mania::ManiaStrains, osu::OsuStrains, taiko::TaikoStrains,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::mode::JsGameMode;

/// The result of calculating the strains of a beatmap.
///
/// Suitable to plot the difficulty over time.
#[wasm_bindgen(js_name = Strains, getter_with_clone, inspectable)]
#[derive(Default)]
pub struct JsStrains {
    /// The strains' gamemode.
    #[wasm_bindgen(readonly)]
    pub mode: JsGameMode,
    /// Time inbetween two strains in ms.
    #[wasm_bindgen(js_name = "sectionLength", readonly)]
    pub section_len: f64,
    /// Strain peaks of the aim skill in osu!.
    #[wasm_bindgen(readonly)]
    pub aim: Option<Vec<f64>>,
    /// Strain peaks of the aim skill without sliders in osu!.
    #[wasm_bindgen(js_name = "aimNoSliders", readonly)]
    pub aim_no_sliders: Option<Vec<f64>>,
    /// Strain peaks of the speed skill in osu!.
    #[wasm_bindgen(readonly)]
    pub speed: Option<Vec<f64>>,
    /// Strain peaks of the flashlight skill in osu!.
    #[wasm_bindgen(readonly)]
    pub flashlight: Option<Vec<f64>>,
    /// Strain peaks of the color skill in osu!taiko.
    #[wasm_bindgen(readonly)]
    pub color: Option<Vec<f64>>,
    /// Strain peaks of the reading skill in osu!taiko.
    #[wasm_bindgen(readonly)]
    pub reading: Option<Vec<f64>>,
    /// Strain peaks of the rhythm skill in osu!taiko.
    #[wasm_bindgen(readonly)]
    pub rhythm: Option<Vec<f64>>,
    /// Strain peaks of the stamina skill in osu!taiko.
    #[wasm_bindgen(readonly)]
    pub stamina: Option<Vec<f64>>,
    /// Strain peaks of the single color stamina skill in osu!taiko.
    #[wasm_bindgen(js_name = "singleColorStamina", readonly)]
    pub single_color_stamina: Option<Vec<f64>>,
    /// Strain peaks of the movement skill in osu!catch.
    #[wasm_bindgen(readonly)]
    pub movement: Option<Vec<f64>>,
    /// Strain peaks of the strain skill in osu!mania.
    #[wasm_bindgen(readonly)]
    pub strains: Option<Vec<f64>>,
}

impl From<Strains> for JsStrains {
    fn from(strains: Strains) -> Self {
        match strains {
            Strains::Osu(OsuStrains {
                aim,
                aim_no_sliders,
                speed,
                flashlight,
            }) => Self {
                mode: JsGameMode::Osu,
                section_len: OsuStrains::SECTION_LEN,
                aim: Some(aim),
                aim_no_sliders: Some(aim_no_sliders),
                speed: Some(speed),
                flashlight: Some(flashlight),
                ..Self::default()
            },
            Strains::Taiko(TaikoStrains {
                color,
                reading,
                rhythm,
                stamina,
                single_color_stamina,
            }) => Self {
                mode: JsGameMode::Taiko,
                section_len: TaikoStrains::SECTION_LEN,
                color: Some(color),
                reading: Some(reading),
                rhythm: Some(rhythm),
                stamina: Some(stamina),
                single_color_stamina: Some(single_color_stamina),
                ..Self::default()
            },
            Strains::Catch(CatchStrains { movement }) => Self {
                mode: JsGameMode::Catch,
                section_len: CatchStrains::SECTION_LEN,
                movement: Some(movement),
                ..Self::default()
            },
            Strains::Mania(ManiaStrains { strains }) => Self {
                mode: JsGameMode::Mania,
                section_len: ManiaStrains::SECTION_LEN,
                strains: Some(strains),
                ..Self::default()
            },
        }
    }
}
