use rosu_pp::{
    any::Strains, catch::CatchStrains, mania::ManiaStrains, osu::OsuStrains, taiko::TaikoStrains,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::mode::JsGameMode;

/// The result of calculating the strains of a beatmap.
///
/// Suitable to plot the difficulty over time.
#[wasm_bindgen(js_name = Strains)]
#[derive(Default)]
pub struct JsStrains {
    /// Strain peaks of the aim skill in osu!
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub aim: Option<Vec<f64>>,
    /// Strain peaks of the aim skill without sliders in osu!
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub aim_no_sliders: Option<Vec<f64>>,
    /// Strain peaks of the speed skill in osu!
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub speed: Option<Vec<f64>>,
    /// Strain peaks of the flashlight skill in osu!
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub flashlight: Option<Vec<f64>>,
    /// Strain peaks of the color skill in osu!taiko.
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub color: Option<Vec<f64>>,
    /// Strain peaks of the rhythm skill in osu!taiko.
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub rhythm: Option<Vec<f64>>,
    /// Strain peaks of the stamina skill in osu!taiko.
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub stamina: Option<Vec<f64>>,
    /// Strain peaks of the movement skill in osu!catch.
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub movement: Option<Vec<f64>>,
    /// Strain peaks of the strain skill in osu!mania.
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub strains: Option<Vec<f64>>,
}

#[wasm_bindgen(js_class = Strains)]
impl JsStrains {
    /// The strains' gamemode.
    /// @throws Will throw an error if the strains have been modified manually
    pub fn mode(&self) -> Result<JsGameMode, String> {
        if self.aim.is_some() {
            Ok(JsGameMode::Osu)
        } else if self.color.is_some() {
            Ok(JsGameMode::Taiko)
        } else if self.movement.is_some() {
            Ok(JsGameMode::Catch)
        } else if self.strains.is_some() {
            Ok(JsGameMode::Mania)
        } else {
            Err("Invalid strains".to_owned())
        }
    }

    /// Time inbetween two strains in ms.
    /// @throws Will throw an error if the strains have been modified manually
    #[wasm_bindgen(getter)]
    pub fn section_len(&self) -> Result<f64, String> {
        let section_len = match self.mode()? {
            JsGameMode::Osu => OsuStrains::SECTION_LEN,
            JsGameMode::Taiko => TaikoStrains::SECTION_LEN,
            JsGameMode::Catch => CatchStrains::SECTION_LEN,
            JsGameMode::Mania => ManiaStrains::SECTION_LEN,
        };

        Ok(section_len)
    }
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
                aim: Some(aim),
                aim_no_sliders: Some(aim_no_sliders),
                speed: Some(speed),
                flashlight: Some(flashlight),
                ..Self::default()
            },
            Strains::Taiko(TaikoStrains {
                color,
                rhythm,
                stamina,
            }) => Self {
                color: Some(color),
                rhythm: Some(rhythm),
                stamina: Some(stamina),
                ..Self::default()
            },
            Strains::Catch(CatchStrains { movement }) => Self {
                movement: Some(movement),
                ..Self::default()
            },
            Strains::Mania(ManiaStrains { strains }) => Self {
                strains: Some(strains),
                ..Self::default()
            },
        }
    }
}
