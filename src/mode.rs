use rosu_pp::model::mode::GameMode;
use serde::{Serialize, Serializer};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::JsError;

#[wasm_bindgen(js_name = GameMode)]
#[derive(Copy, Clone, Debug, Default)]
pub enum JsGameMode {
    #[default]
    Osu,
    Taiko,
    Catch,
    Mania,
}

impl TryFrom<i64> for JsGameMode {
    type Error = JsError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Osu),
            1 => Ok(Self::Taiko),
            2 => Ok(Self::Catch),
            3 => Ok(Self::Mania),
            _ => Err(JsError::new("invalid GameMode")),
        }
    }
}

impl Serialize for JsGameMode {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(*self as u8)
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
