use neon::{prelude::ModuleContext, result::NeonResult};

use crate::{beatmap::Map, calculator::Calculator};

mod beatmap;
mod calculator;
mod error;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
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

// fn calculate(mut cx: FunctionContext) -> JsResult<JsValue> {
//     let arg = cx.argument::<JsValue>(0)?;
//
//     let CalculateArg { map_input, params } = neon_serde3::from_value(&mut cx, arg)
//         .map_err(|e| unwind_error("Failed to deserialize argument", &e))
//         .or_else(|e| cx.throw_error(e))?;
//
//     if params.is_empty() {
//         return Ok(JsArray::new(&mut cx, 0).as_value(&mut cx));
//     }
//
//     let mut map = map_input
//         .parse()
//         .map_err(|e| unwind_error("Failed to parse beatmap", &e))
//         .or_else(|e| cx.throw_error(e))?;
//
//     // Avoid caching if it's not necessary
//     let results: Vec<_> = if multiple_same_attributes(&params) {
//         let mut attrs_seen = HashMap::new();
//
//         params
//             .into_iter()
//             .map(|params| {
//                 let attr_key = params.as_attr_key();
//                 let mut attr_switcher = attr_key.attr_switcher;
//                 let mods = params.mods;
//                 let clock_rate = params.clock_rate.map(|rate| rate as f64);
//
//                 let difficulty = attrs_seen
//                     .entry(attr_key)
//                     .or_insert_with(|| {
//                         attr_switcher.apply(&mut map);
//                         let mut calculator = map.stars().mods(mods);
//
//                         if let Some(passed_objects) = params.passed_objects {
//                             calculator = calculator.passed_objects(passed_objects);
//                         }
//
//                         if let Some(clock_rate) = clock_rate {
//                             calculator = calculator.clock_rate(clock_rate);
//                         }
//
//                         let attrs = calculator.calculate();
//                         attr_switcher.reset(&mut map);
//
//                         attrs
//                     })
//                     .to_owned();
//
//                 attr_switcher.apply(&mut map);
//                 let attrs = params.apply(AnyPP::new(&map).attributes(difficulty));
//                 let result = CalculateResult::new(attrs, &map, mods, clock_rate);
//                 attr_switcher.reset(&mut map);
//
//                 result
//             })
//             .collect()
//     } else {
//         params
//             .into_iter()
//             .map(|params| {
//                 let mods = params.mods;
//                 let clock_rate = params.clock_rate.map(|rate| rate as f64);
//                 let mut attr_switcher = params.as_attr_switcher();
//
//                 attr_switcher.apply(&mut map);
//                 let attrs = params.apply(AnyPP::new(&map));
//                 let result = CalculateResult::new(attrs, &map, mods, clock_rate);
//                 attr_switcher.reset(&mut map);
//
//                 result
//             })
//             .collect()
//     };
//
//     neon_serde3::to_value(&mut cx, &results)
//         .map_err(|e| unwind_error("Failed to serialize results", &e))
//         .or_else(|e| cx.throw_error(e))
// }
//
// #[derive(Clone, Debug, Default, PartialEq, Serialize)]
// struct Strains {
//     mode: u8,
//     #[serde(rename = "sectionLength")]
//     section_length: f64,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     color: Option<Vec<f64>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     rhythm: Option<Vec<f64>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     stamina: Option<Vec<f64>>,
//
//     #[serde(skip_serializing_if = "Option::is_none")]
//     aim: Option<Vec<f64>>,
//     #[serde(rename = "aimNoSliders", skip_serializing_if = "Option::is_none")]
//     aim_no_sliders: Option<Vec<f64>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     speed: Option<Vec<f64>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     flashlight: Option<Vec<f64>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     strains: Option<Vec<f64>>,
//
//     #[serde(skip_serializing_if = "Option::is_none")]
//     movement: Option<Vec<f64>>,
// }
//
// #[derive(Deserialize)]
// #[serde(deny_unknown_fields)]
// struct ScoreParams {
//     #[serde(default, deserialize_with = "deserialize_mode")]
//     mode: Option<GameMode>,
//     #[serde(default)]
//     mods: u32,
//     n300: Option<usize>,
//     n100: Option<usize>,
//     n50: Option<usize>,
//     #[serde(rename = "nMisses")]
//     n_misses: Option<usize>,
//     #[serde(rename = "nKatu")]
//     n_katu: Option<usize>,
//     #[serde(rename = "nGeki")]
//     n_geki: Option<usize>,
//     acc: Option<f64>,
//     combo: Option<usize>,
//     #[serde(rename = "passedObjects")]
//     passed_objects: Option<usize>,
//     #[serde(rename = "clockRate")]
//     clock_rate: Option<f32>,
//     ar: Option<f32>,
//     cs: Option<f32>,
//     hp: Option<f32>,
//     od: Option<f32>,
// }
//
// #[derive(Default, Serialize)]
// struct CalculateResult {
//     mode: u8,
//     stars: f64,
//     pp: f64,
//     #[serde(rename = "ppAcc", skip_serializing_if = "Option::is_none")]
//     pp_acc: Option<f64>,
//     #[serde(rename = "ppAim", skip_serializing_if = "Option::is_none")]
//     pp_aim: Option<f64>,
//     #[serde(rename = "ppFlashlight", skip_serializing_if = "Option::is_none")]
//     pp_flashlight: Option<f64>,
//     #[serde(rename = "ppSpeed", skip_serializing_if = "Option::is_none")]
//     pp_speed: Option<f64>,
//     #[serde(rename = "ppDifficulty", skip_serializing_if = "Option::is_none")]
//     pp_difficulty: Option<f64>,
//
//     #[serde(rename = "nFruits", skip_serializing_if = "Option::is_none")]
//     n_fruits: Option<usize>,
//     #[serde(rename = "nDroplets", skip_serializing_if = "Option::is_none")]
//     n_droplets: Option<usize>,
//     #[serde(rename = "nTinyDropplets", skip_serializing_if = "Option::is_none")]
//     n_tiny_droplets: Option<usize>,
//
//     #[serde(rename = "aimStrain", skip_serializing_if = "Option::is_none")]
//     aim_strain: Option<f64>,
//     #[serde(rename = "speedStrain", skip_serializing_if = "Option::is_none")]
//     speed_strain: Option<f64>,
//     #[serde(rename = "flashlightStrain", skip_serializing_if = "Option::is_none")]
//     flashlight_strain: Option<f64>,
//     #[serde(rename = "sliderFactor", skip_serializing_if = "Option::is_none")]
//     slider_factor: Option<f64>,
//     #[serde(rename = "effectiveMissCount", skip_serializing_if = "Option::is_none")]
//     effective_miss_count: Option<f64>,
//     #[serde(rename = "speedNoteCount", skip_serializing_if = "Option::is_none")]
//     speed_note_count: Option<f64>,
//     #[serde(rename = "staminaStrain", skip_serializing_if = "Option::is_none")]
//     stamina_strain: Option<f64>,
//     #[serde(rename = "rhythmStrain", skip_serializing_if = "Option::is_none")]
//     rhythm_strain: Option<f64>,
//     #[serde(rename = "colorStrain", skip_serializing_if = "Option::is_none")]
//     color_strain: Option<f64>,
//     #[serde(rename = "peakStrain", skip_serializing_if = "Option::is_none")]
//     peak_strain: Option<f64>,
//
//     ar: f64,
//     cs: f64,
//     hp: f64,
//     od: f64,
//     bpm: f64,
//     #[serde(rename = "clockRate")]
//     clock_rate: f64,
//     #[serde(rename = "timePreempt", skip_serializing_if = "Option::is_none")]
//     time_preempt: Option<f64>,
//     #[serde(rename = "greatHitWindow", skip_serializing_if = "Option::is_none")]
//     great_hitwindow: Option<f64>,
//     #[serde(rename = "nCircles", skip_serializing_if = "Option::is_none")]
//     n_circles: Option<usize>,
//     #[serde(rename = "nSliders", skip_serializing_if = "Option::is_none")]
//     n_sliders: Option<usize>,
//     #[serde(rename = "nSpinners", skip_serializing_if = "Option::is_none")]
//     n_spinners: Option<usize>,
//     #[serde(rename = "maxCombo", skip_serializing_if = "Option::is_none")]
//     max_combo: Option<usize>,
// }
//
// impl CalculateResult {
//     fn new(
//         attrs: PerformanceAttributes,
//         map: &Beatmap,
//         mods: u32,
//         clock_rate: Option<f64>,
//     ) -> Self {
//         let mut attr_builder = map.attributes();
//
//         if let Some(clock_rate) = clock_rate {
//             attr_builder.clock_rate(clock_rate);
//         }
//
//         let mode = match &attrs {
//             PerformanceAttributes::Catch(_) => GameMode::Catch,
//             PerformanceAttributes::Mania(_) => GameMode::Mania,
//             PerformanceAttributes::Osu(_) => GameMode::Osu,
//             PerformanceAttributes::Taiko(_) => GameMode::Taiko,
//         };
//
//         if map.mode == GameMode::Osu && mode != GameMode::Osu {
//             attr_builder.converted(true);
//         }
//
//         let BeatmapAttributes {
//             ar,
//             cs,
//             hp,
//             od,
//             clock_rate,
//             hit_windows,
//         } = attr_builder.mods(mods).mode(mode).build();
//
//         let bpm = map.bpm() * clock_rate;
//
//         match attrs {
//             PerformanceAttributes::Catch(CatchPerformanceAttributes { pp, difficulty }) => Self {
//                 mode: 2,
//                 pp,
//                 stars: difficulty.stars,
//                 max_combo: Some(difficulty.n_fruits + difficulty.n_droplets),
//                 n_fruits: Some(difficulty.n_fruits),
//                 n_droplets: Some(difficulty.n_droplets),
//                 n_tiny_droplets: Some(difficulty.n_tiny_droplets),
//                 n_spinners: Some(map.n_spinners as usize),
//                 ar,
//                 cs,
//                 hp,
//                 od,
//                 bpm,
//                 clock_rate,
//                 ..Default::default()
//             },
//             PerformanceAttributes::Mania(ManiaPerformanceAttributes {
//                 pp,
//                 pp_difficulty,
//                 difficulty,
//             }) => Self {
//                 mode: 3,
//                 pp,
//                 pp_difficulty: Some(pp_difficulty),
//                 stars: difficulty.stars,
//                 max_combo: Some(difficulty.max_combo),
//                 n_circles: Some(map.n_circles as usize),
//                 n_sliders: Some(map.n_sliders as usize),
//                 ar,
//                 cs,
//                 hp,
//                 od,
//                 bpm,
//                 clock_rate,
//                 great_hitwindow: Some(difficulty.hit_window),
//                 ..Default::default()
//             },
//             PerformanceAttributes::Osu(OsuPerformanceAttributes {
//                 difficulty,
//                 pp,
//                 pp_acc,
//                 pp_aim,
//                 pp_flashlight,
//                 pp_speed,
//                 effective_miss_count,
//             }) => Self {
//                 mode: 0,
//                 pp,
//                 pp_acc: Some(pp_acc),
//                 pp_aim: Some(pp_aim),
//                 pp_flashlight: Some(pp_flashlight),
//                 pp_speed: Some(pp_speed),
//                 stars: difficulty.stars,
//                 max_combo: Some(difficulty.max_combo),
//                 aim_strain: Some(difficulty.aim),
//                 speed_strain: Some(difficulty.speed),
//                 flashlight_strain: Some(difficulty.flashlight),
//                 slider_factor: Some(difficulty.slider_factor),
//                 n_circles: Some(difficulty.n_circles),
//                 n_sliders: Some(difficulty.n_sliders),
//                 n_spinners: Some(difficulty.n_spinners),
//                 effective_miss_count: Some(effective_miss_count),
//                 speed_note_count: Some(difficulty.speed_note_count),
//                 ar,
//                 cs,
//                 hp,
//                 od,
//                 bpm,
//                 clock_rate,
//                 time_preempt: Some(hit_windows.ar),
//                 great_hitwindow: Some(hit_windows.od),
//                 ..Default::default()
//             },
//             PerformanceAttributes::Taiko(TaikoPerformanceAttributes {
//                 difficulty,
//                 pp,
//                 pp_acc,
//                 pp_difficulty,
//                 effective_miss_count,
//             }) => Self {
//                 mode: 1,
//                 pp,
//                 pp_acc: Some(pp_acc),
//                 pp_difficulty: Some(pp_difficulty),
//                 stars: difficulty.stars,
//                 max_combo: Some(difficulty.max_combo),
//                 stamina_strain: Some(difficulty.stamina),
//                 rhythm_strain: Some(difficulty.rhythm),
//                 color_strain: Some(difficulty.colour),
//                 peak_strain: Some(difficulty.peak),
//                 n_circles: Some(map.n_circles as usize),
//                 n_sliders: Some(map.n_sliders as usize),
//                 n_spinners: Some(map.n_spinners as usize),
//                 effective_miss_count: Some(effective_miss_count),
//                 ar,
//                 cs,
//                 hp,
//                 od,
//                 bpm,
//                 clock_rate,
//                 great_hitwindow: Some(hit_windows.od),
//                 ..Default::default()
//             },
//         }
//     }
// }
