use rosu_pp::model::mode::GameMode;
use serde::de;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_name = GameMode)]
#[derive(Copy, Clone, Default)]
pub enum JsGameMode {
    #[default]
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

impl<'de> de::Deserialize<'de> for JsGameMode {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mode = match u8::deserialize(d) {
            Ok(0) => Self::Osu,
            Ok(1) => Self::Taiko,
            Ok(2) => Self::Catch,
            Ok(3) => Self::Mania,
            _ => return Err(de::Error::custom("invalid GameMode")),
        };

        Ok(mode)
    }
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
