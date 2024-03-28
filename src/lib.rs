mod args;
mod attributes;
mod beatmap;
mod deserializer;
mod difficulty;
mod error;
mod gradual;
mod mode;
mod performance;
mod score_state;
mod strains;
mod util;

use self::error::{JsError, JsResult};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen::prelude::wasm_bindgen]
#[cfg(debug_assertions)]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
