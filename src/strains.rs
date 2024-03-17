use rosu_pp::{
    any::Strains as RosuStrains, catch::CatchStrains, mania::ManiaStrains, osu::OsuStrains,
    taiko::TaikoStrains,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::mode::JsGameMode;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = Strains)]
    pub type JsStrains;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* The result of calculating the strains of a beatmap.
*
* Suitable to plot the difficulty over time.
*/
interface Strains {
    /**
    * The strains' gamemode.
    */
    mode: GameMode,
    /**
    * Time inbetween two strains in ms.
    */
    sectionLength: number,
    /**
    * Strain peaks of the aim skill in osu!
    */
    aim?: Float64Array,
    /**
    * Strain peaks of the aim skill without sliders in osu!
    */
    aimNoSliders?: Float64Array,
    /**
    * Strain peaks of the speed skill in osu!
    */
    speed?: Float64Array,
    /**
    * Strain peaks of the flashlight skill in osu!
    */
    flashlight?: Float64Array,
    /**
    * Strain peaks of the color skill in osu!taiko.
    */
    color?: Float64Array,
    /**
    * Strain peaks of the rhythm skill in osu!taiko.
    */
    rhythm?: Float64Array,
    /**
    * Strain peaks of the stamina skill in osu!taiko.
    */
    stamina?: Float64Array,
    /**
    * Strain peaks of the movement skill in osu!catch.
    */
    movement?: Float64Array,
    /**
    * Strain peaks of the strain skill in osu!mania.
    */
    strains?: Float64Array,
}"#;

#[derive(Default, serde::Serialize)]
pub struct Strains {
    pub mode: JsGameMode,
    #[serde(rename = "sectionLength")]
    pub section_len: f64,
    pub aim: Option<Vec<f64>>,
    #[serde(rename = "aimNoSliders")]
    pub aim_no_sliders: Option<Vec<f64>>,
    pub speed: Option<Vec<f64>>,
    pub flashlight: Option<Vec<f64>>,
    pub color: Option<Vec<f64>>,
    pub rhythm: Option<Vec<f64>>,
    pub stamina: Option<Vec<f64>>,
    pub movement: Option<Vec<f64>>,
    pub strains: Option<Vec<f64>>,
}

impl From<RosuStrains> for Strains {
    fn from(strains: RosuStrains) -> Self {
        match strains {
            RosuStrains::Osu(OsuStrains {
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
            RosuStrains::Taiko(TaikoStrains {
                color,
                rhythm,
                stamina,
            }) => Self {
                mode: JsGameMode::Taiko,
                section_len: TaikoStrains::SECTION_LEN,
                color: Some(color),
                rhythm: Some(rhythm),
                stamina: Some(stamina),
                ..Self::default()
            },
            RosuStrains::Catch(CatchStrains { movement }) => Self {
                mode: JsGameMode::Catch,
                section_len: CatchStrains::SECTION_LEN,
                movement: Some(movement),
                ..Self::default()
            },
            RosuStrains::Mania(ManiaStrains { strains }) => Self {
                mode: JsGameMode::Mania,
                section_len: ManiaStrains::SECTION_LEN,
                strains: Some(strains),
                ..Self::default()
            },
        }
    }
}
