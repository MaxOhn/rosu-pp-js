mod args;
mod attributes;
mod beatmap;
mod deserializer;
mod difficulty;
mod error;
mod gradual;
mod mode;
mod mods;
mod performance;
mod score_state;
mod strains;
mod util;

use self::error::{JsError, JsResult};

#[wasm_bindgen::prelude::wasm_bindgen]
#[cfg(debug_assertions)]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
