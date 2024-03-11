use rosu_pp::any::ScoreState;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = ScoreState, inspectable)]
pub struct JsScoreState {
    /// Maximum combo that the score has had so far. **Not** the maximum
    /// possible combo of the map so far.
    ///
    /// Note that for osu!catch only fruits and droplets are considered for
    /// combo.
    ///
    /// Irrelevant for osu!mania.
    #[wasm_bindgen(js_name = maxCombo)]
    pub max_combo: u32,
    /// Amount of current gekis (n320 for osu!mania).
    #[wasm_bindgen(js_name = nGeki)]
    pub n_geki: u32,
    /// Amount of current katus (tiny droplet misses for osu!catch / n200 for
    /// osu!mania).
    #[wasm_bindgen(js_name = nKatu)]
    pub n_katu: u32,
    /// Amount of current 300s (fruits for osu!catch).
    pub n300: u32,
    /// Amount of current 100s (droplets for osu!catch).
    pub n100: u32,
    /// Amount of current 50s (tiny droplets for osu!catch).
    pub n50: u32,
    /// Amount of current misses (fruits + droplets for osu!catch).
    pub misses: u32,
}

#[wasm_bindgen(js_class = ScoreState)]
impl JsScoreState {
    #[wasm_bindgen(constructor)]
    pub fn new(
        maxCombo: u32,
        nGeki: u32,
        nKatu: u32,
        n300: u32,
        n100: u32,
        n50: u32,
        misses: u32,
    ) -> Self {
        JsScoreState {
            max_combo: maxCombo,
            n_geki: nGeki,
            n_katu: nKatu,
            n300,
            n100,
            n50,
            misses,
        }
    }
}

impl From<JsScoreState> for ScoreState {
    fn from(state: JsScoreState) -> Self {
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

impl From<ScoreState> for JsScoreState {
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
