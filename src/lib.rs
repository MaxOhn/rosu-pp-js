#![deny(clippy::all, nonstandard_style, rust_2018_idioms, unused, warnings)]

use neon::{prelude::ModuleContext, result::NeonResult};

use crate::{beatmap::Map, calculator::Calculator};

mod beatmap;
mod calculator;
mod error;

#[neon::main]
fn main(mut cx: ModuleContext<'_>) -> NeonResult<()> {
    cx.export_function("beatmapNew", Map::js_new)?;
    cx.export_function("beatmapFromPath", Map::js_from_path)?;
    cx.export_function("beatmapFromContent", Map::js_from_content)?;
    cx.export_function("beatmapFromBytes", Map::js_from_bytes)?;
    cx.export_function("beatmapAr", Map::js_ar)?;
    cx.export_function("beatmapCs", Map::js_cs)?;
    cx.export_function("beatmapHp", Map::js_hp)?;
    cx.export_function("beatmapOd", Map::js_od)?;

    cx.export_function("calculatorNew", Calculator::js_new)?;
    cx.export_function("calculatorMapAttrs", Calculator::js_map_attrs)?;
    cx.export_function("calculatorDifficulty", Calculator::js_difficulty)?;
    cx.export_function("calculatorPerformance", Calculator::js_performance)?;
    cx.export_function("calculatorStrains", Calculator::js_strains)?;
    cx.export_function("calculatorMode", Calculator::js_mode)?;
    cx.export_function("calculatorMods", Calculator::js_mods)?;
    cx.export_function("calculatorAcc", Calculator::js_acc)?;
    cx.export_function("calculatorGeki", Calculator::js_geki)?;
    cx.export_function("calculatorKatu", Calculator::js_katu)?;
    cx.export_function("calculatorN300", Calculator::js_n300)?;
    cx.export_function("calculatorN100", Calculator::js_n100)?;
    cx.export_function("calculatorN50", Calculator::js_n50)?;
    cx.export_function("calculatorMisses", Calculator::js_misses)?;
    cx.export_function("calculatorCombo", Calculator::js_combo)?;
    cx.export_function("calculatorPassedObjects", Calculator::js_passed_objects)?;
    cx.export_function("calculatorClockRate", Calculator::js_clock_rate)?;

    Ok(())
}
