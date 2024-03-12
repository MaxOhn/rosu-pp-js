#![allow(non_snake_case)]

use rosu_pp::model::mode::GameMode;
use wasm_bindgen::prelude::*;

mod attributes;
mod beatmap;
mod difficulty;
mod gradual;
mod mode;
mod performance;
mod score_state;
mod strains;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn cannot_convert(from: GameMode, to: GameMode) -> String {
    format!("Cannot convert {from:?} to {to:?}")
}
