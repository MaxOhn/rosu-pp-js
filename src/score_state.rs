use rosu_pp::any::ScoreState as RosuScoreState;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{util, JsResult};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = ScoreState)]
    pub type JsScoreState;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
*/
interface ScoreState {
    /**
    * Maximum combo that the score has had so far. **Not** the maximum
    * possible combo of the map so far.
    *
    * Note that for osu!catch only fruits and droplets are considered for
    * combo.
    *
    * Irrelevant for osu!mania.
    */
    maxCombo?: number,
    /**
    * Amount of current gekis (n320 for osu!mania).
    */
    nGeki?: number,
    /**
    * Amount of current katus (tiny droplet misses for osu!catch / n200 for
    * osu!mania).
    */
    nKatu?: number,
    /**
    * Amount of current 300s (fruits for osu!catch).
    */
    n300?: number,
    /**
    * Amount of current 100s (droplets for osu!catch).
    */
    n100?: number,
    /**
    * Amount of current 50s (tiny droplets for osu!catch).
    */
    n50?: number,
    /**
    * Amount of current misses (fruits + droplets for osu!catch).
    */
    misses?: number,
}"#;

#[derive(Debug, Default, serde::Serialize)]
pub(crate) struct ScoreState {
    #[serde(rename = "maxCombo")]
    max_combo: u32,
    #[serde(rename = "nGeki")]
    n_geki: u32,
    #[serde(rename = "nKatu")]
    n_katu: u32,
    n300: u32,
    n100: u32,
    n50: u32,
    misses: u32,
}

impl ScoreState {
    pub fn from_value(state: &JsScoreState) -> JsResult<RosuScoreState> {
        util::from_value(state)
    }
}

from_jsvalue! {
    RosuScoreState {
        max_combo as maxCombo: u32!,
        n_geki as nGeki: u32!,
        n_katu as nKatu: u32!,
        n300 as n300: u32!,
        n100 as n100: u32!,
        n50 as n50: u32!,
        misses as misses: u32!,
    }
}

impl From<ScoreState> for RosuScoreState {
    fn from(state: ScoreState) -> Self {
        Self {
            max_combo: state.max_combo,
            n_geki: state.n_geki,
            n_katu: state.n_katu,
            n300: state.n300,
            n100: state.n100,
            n50: state.n50,
            misses: state.misses,
        }
    }
}

impl From<RosuScoreState> for ScoreState {
    fn from(state: RosuScoreState) -> Self {
        Self {
            max_combo: state.max_combo,
            n_geki: state.n_geki,
            n_katu: state.n_katu,
            n300: state.n300,
            n100: state.n100,
            n50: state.n50,
            misses: state.misses,
        }
    }
}
