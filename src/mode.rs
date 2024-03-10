use rosu_pp::model::mode::GameMode;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = GameMode)]
#[derive(Copy, Clone, Debug, Default)]
pub enum JsGameMode {
    #[default]
    Osu,
    Taiko,
    Catch,
    Mania,
}

impl From<GameMode> for JsGameMode {
    fn from(mode: GameMode) -> Self {
        match mode {
            GameMode::Osu => Self::Osu,
            GameMode::Taiko => Self::Taiko,
            GameMode::Catch => Self::Catch,
            GameMode::Mania => Self::Mania,
        }
    }
}

impl From<JsGameMode> for GameMode {
    fn from(mode: JsGameMode) -> Self {
        match mode {
            JsGameMode::Osu => Self::Osu,
            JsGameMode::Taiko => Self::Taiko,
            JsGameMode::Catch => Self::Catch,
            JsGameMode::Mania => Self::Mania,
        }
    }
}
