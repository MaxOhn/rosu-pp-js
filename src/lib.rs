#[macro_use]
mod macros;

mod args;
mod attributes;
mod beatmap;
mod difficulty;
mod gradual;
mod mode;
mod performance;
mod score_state;
mod strains;
mod util;

type JsError = serde_wasm_bindgen::Error;
type JsResult<T> = Result<T, JsError>;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
