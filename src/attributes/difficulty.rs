use rosu_pp::{
    any::DifficultyAttributes as RosuDifficultyAttributes, catch::CatchDifficultyAttributes,
    mania::ManiaDifficultyAttributes, osu::OsuDifficultyAttributes,
    taiko::TaikoDifficultyAttributes,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};

use crate::{
    mode::JsGameMode,
    util::{self, JsValueExt, ObjectExt},
    JsError, JsResult,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = DifficultyAttributes)]
    pub type JsDifficultyAttributes;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* The result of a difficulty calculation.
*/
interface DifficultyAttributes {
    /**
    * The attributes' gamemode.
    */
    mode: GameMode,
    /**
    * The final star rating.
    */
    stars: number,
    /**
    * Whether the map was a convert i.e. an osu! map.
    */
    isConvert: boolean,
    /**
    * The difficulty of the aim skill.
    *
    * Only available for osu!.
    */
    aim?: number,
    /**
    * The difficulty of the speed skill.
    *
    * Only available for osu!.
    */
    speed?: number,
    /**
    * The difficulty of the flashlight skill.
    *
    * Only available for osu!.
    */
    flashlight?: number,
    /**
    * The ratio of the aim strain with and without considering sliders
    *
    * Only available for osu!.
    */
    sliderFactor?: number,
    /**
    * The number of clickable objects weighted by difficulty.
    *
    * Only available for osu!.
    */
    speedNoteCount?: number,
    /**
    * The overall difficulty
    *
    * Only available for osu!.
    */
    od?: number,
    /**
    * The health drain rate.
    *
    * Only available for osu!.
    */
    hp?: number,
    /**
    * The amount of circles.
    *
    * Only available for osu!.
    */
    nCircles?: number,
    /**
    * The amount of sliders.
    *
    * Only available for osu!.
    */
    nSliders?: number,
    /**
    * The amount of spinners.
    *
    * Only available for osu!.
    */
    nSpinners?: number,
    /**
    * The difficulty of the stamina skill.
    *
    * Only available for osu!taiko.
    */
    stamina?: number,
    /**
    * The difficulty of the rhythm skill.
    *
    * Only available for osu!taiko.
    */
    rhythm?: number,
    /**
    * The difficulty of the color skill.
    *
    * Only available for osu!taiko.
    */
    color?: number,
    /**
    * The difficulty of the hardest parts of the map.
    *
    * Only available for osu!taiko.
    */
    peak?: number,
    /**
    * The amount of fruits.
    *
    * Only available for osu!catch.
    */
    nFruits?: number,
    /**
    * The amount of droplets.
    *
    * Only available for osu!catch.
    */
    nDroplets?: number,
    /**
    * The amount of tiny droplets.
    *
    * Only available for osu!catch.
    */
    nTinyDroplets?: number,
    /**
    * The amount of hitobjects in the map.
    *
    * Only available for osu!mania.
    */
    nObjects?: number,
    /**
    * The approach rate.
    *
    * Only available for osu! and osu!catch.
    */
    ar?: number,
    /**
    * The perceived hit window for an n300 inclusive of rate-adjusting mods (DT/HT/etc)
    *
    * Only available for osu!taiko and osu!mania.
    */
    hitWindow?: number,
    /**
    * Return the maximum combo.
    */
    maxCombo: number,
}"#;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub(crate) struct DifficultyAttributes {
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
}

impl DifficultyAttributes {
    pub fn from_value(value: &JsDifficultyAttributes) -> JsResult<RosuDifficultyAttributes> {
        if !value.is_object() {
            return Err(JsError::new("argument must be an object"));
        }

        let obj = value.unchecked_ref::<ObjectExt>();
        let js_field = util::static_str_to_js("mode");
        let js_value = obj.get_with_ref_key(&js_field);

        if js_value.is_undefined() {
            return Err(JsError::new("missing mode"));
        }

        let mode = match js_value.as_safe_integer().map(TryFrom::try_from) {
            Some(Ok(mode)) => mode,
            _ => return Err(JsError::new("invalid mode")),
        };

        let attrs = match mode {
            JsGameMode::Osu => RosuDifficultyAttributes::Osu(util::from_value(value)?),
            JsGameMode::Taiko => RosuDifficultyAttributes::Taiko(util::from_value(value)?),
            JsGameMode::Catch => RosuDifficultyAttributes::Catch(util::from_value(value)?),
            JsGameMode::Mania => RosuDifficultyAttributes::Mania(util::from_value(value)?),
        };

        Ok(attrs)
    }
}

from_jsvalue! {
    OsuDifficultyAttributes {
        stars as stars: f64!,
        aim as aim: f64!,
        speed as speed: f64!,
        flashlight as flashlight: f64!,
        slider_factor as sliderFactor: f64!,
        speed_note_count as speedNoteCount: f64!,
        ar as ar: f64!,
        od as od: f64!,
        hp as hp: f64!,
        n_circles as nCircles: u32!,
        n_sliders as nSliders: u32!,
        n_spinners as nSpinners: u32!,
        max_combo as maxCombo: u32!,
    }
}

from_jsvalue! {
    TaikoDifficultyAttributes {
        stars as stars: f64!,
        is_convert as isConvert: bool!,
        stamina as stamina: f64!,
        rhythm as rhythm: f64!,
        color as color: f64!,
        peak as peak: f64!,
        hit_window as hitWindow: f64!,
        max_combo as maxCombo: u32!,
    }
}

from_jsvalue! {
    CatchDifficultyAttributes {
        stars as stars: f64!,
        is_convert as isConvert: bool!,
        n_fruits as nFruits: u32!,
        n_droplets as nDroplets: u32!,
        n_tiny_droplets as nTinyDroplets: u32!,
        ar as ar: f64!,
    }
}

from_jsvalue! {
    ManiaDifficultyAttributes {
        stars as stars: f64!,
        is_convert as isConvert: bool!,
        n_objects as nObjects: u32!,
        hit_window as hitWindow: f64!,
        max_combo as maxCombo: u32!,
    }
}

impl From<OsuDifficultyAttributes> for DifficultyAttributes {
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

impl From<TaikoDifficultyAttributes> for DifficultyAttributes {
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

impl From<CatchDifficultyAttributes> for DifficultyAttributes {
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

impl From<ManiaDifficultyAttributes> for DifficultyAttributes {
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

impl From<RosuDifficultyAttributes> for DifficultyAttributes {
    fn from(attrs: RosuDifficultyAttributes) -> Self {
        match attrs {
            RosuDifficultyAttributes::Osu(attrs) => attrs.into(),
            RosuDifficultyAttributes::Taiko(attrs) => attrs.into(),
            RosuDifficultyAttributes::Catch(attrs) => attrs.into(),
            RosuDifficultyAttributes::Mania(attrs) => attrs.into(),
        }
    }
}

impl TryFrom<DifficultyAttributes> for RosuDifficultyAttributes {
    type Error = JsError;

    fn try_from(attrs: DifficultyAttributes) -> Result<Self, Self::Error> {
        let DifficultyAttributes {
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

        Err(JsError::new("invalid difficulty attributes"))
    }
}
