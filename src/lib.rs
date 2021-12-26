use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{Formatter, Result as FmtResult, Write},
};

use neon::prelude::*;
use rosu_pp::{
    fruits::FruitsPerformanceAttributes, mania::ManiaPerformanceAttributes,
    osu::OsuPerformanceAttributes, taiko::TaikoPerformanceAttributes, AnyPP, Beatmap,
    BeatmapAttributes, BeatmapExt, PerformanceAttributes,
};
use serde::{
    de::{Error as DeError, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

fn calculate(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg = cx.argument::<JsValue>(0)?;

    let CalculateArg { path, params } = neon_serde2::from_value(&mut cx, arg)
        .map_err(|e| unwind_error("Failed to deserialize argument", &e))
        .or_else(|e| cx.throw_error(e))?;

    if params.is_empty() {
        return Ok(JsArray::new(&mut cx, 0).as_value(&mut cx));
    }

    let map = Beatmap::from_path(path)
        .map_err(|e| unwind_error("Failed to parse beatmap", &e))
        .or_else(|e| cx.throw_error(e))?;

    let mut mod_diffs = HashMap::new();

    let results: Vec<_> = params
        .into_iter()
        .map(|params| {
            let ScoreParams {
                mods,
                n300,
                n100,
                n50,
                n_misses,
                n_katu,
                acc,
                combo,
                score,
                passed_objects,
            } = params;

            let difficulty = mod_diffs
                .entry((mods, passed_objects))
                .or_insert_with(|| map.stars(mods, passed_objects))
                .to_owned();

            let mut calculator = AnyPP::new(&map).mods(mods).attributes(difficulty);

            if let Some(n300) = n300 {
                calculator = calculator.n300(n300);
            }

            if let Some(n100) = n100 {
                calculator = calculator.n100(n100);
            }

            if let Some(n50) = n50 {
                calculator = calculator.n50(n50);
            }

            if let Some(n_misses) = n_misses {
                calculator = calculator.misses(n_misses);
            }

            if let Some(n_katu) = n_katu {
                calculator = calculator.n_katu(n_katu);
            }

            if let Some(combo) = combo {
                calculator = calculator.combo(combo);
            }

            if let Some(passed_objects) = passed_objects {
                calculator = calculator.passed_objects(passed_objects);
            }

            if let Some(acc) = acc {
                calculator = calculator.accuracy(acc);
            }

            if let Some(score) = score {
                calculator = calculator.score(score);
            }

            CalculateResult::new(calculator.calculate(), &map, mods)
        })
        .collect();

    neon_serde2::to_value(&mut cx, &results)
        .map_err(|e| unwind_error("Failed to serialize results", &e))
        .or_else(|e| cx.throw_error(e))
}

struct CalculateArg {
    path: String,
    params: Vec<ScoreParams>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScoreParams {
    #[serde(default)]
    mods: u32,
    n300: Option<usize>,
    n100: Option<usize>,
    n50: Option<usize>,
    #[serde(rename = "nMisses")]
    n_misses: Option<usize>,
    #[serde(rename = "nKatu")]
    n_katu: Option<usize>,
    acc: Option<f64>,
    combo: Option<usize>,
    score: Option<u32>,
    #[serde(rename = "passedObjects")]
    passed_objects: Option<usize>,
}

#[derive(Default, Serialize)]
struct CalculateResult {
    mode: u8,
    stars: f64,
    pp: f64,
    #[serde(rename = "ppAcc", skip_serializing_if = "Option::is_none")]
    pp_acc: Option<f64>,
    #[serde(rename = "ppAim", skip_serializing_if = "Option::is_none")]
    pp_aim: Option<f64>,
    #[serde(rename = "ppFlashlight", skip_serializing_if = "Option::is_none")]
    pp_flashlight: Option<f64>,
    #[serde(rename = "ppSpeed", skip_serializing_if = "Option::is_none")]
    pp_speed: Option<f64>,
    #[serde(rename = "ppStrain", skip_serializing_if = "Option::is_none")]
    pp_strain: Option<f64>,

    #[serde(rename = "nFruits", skip_serializing_if = "Option::is_none")]
    n_fruits: Option<usize>,
    #[serde(rename = "nDroplets", skip_serializing_if = "Option::is_none")]
    n_droplets: Option<usize>,
    #[serde(rename = "nTinyDropplets", skip_serializing_if = "Option::is_none")]
    n_tiny_droplets: Option<usize>,

    #[serde(rename = "aimStrain", skip_serializing_if = "Option::is_none")]
    aim_strain: Option<f64>,
    #[serde(rename = "speedStrain", skip_serializing_if = "Option::is_none")]
    speed_strain: Option<f64>,
    #[serde(rename = "flashlightRating", skip_serializing_if = "Option::is_none")]
    flashlight_rating: Option<f64>,
    #[serde(rename = "sliderFactor", skip_serializing_if = "Option::is_none")]
    slider_factor: Option<f64>,

    ar: f64,
    cs: f64,
    hp: f64,
    od: f64,
    bpm: f64,
    #[serde(rename = "nCircles", skip_serializing_if = "Option::is_none")]
    n_circles: Option<usize>,
    #[serde(rename = "nSliders", skip_serializing_if = "Option::is_none")]
    n_sliders: Option<usize>,
    #[serde(rename = "nSpinners", skip_serializing_if = "Option::is_none")]
    n_spinners: Option<usize>,
    #[serde(rename = "maxCombo", skip_serializing_if = "Option::is_none")]
    max_combo: Option<usize>,
}

impl CalculateResult {
    fn new(attrs: PerformanceAttributes, map: &Beatmap, mods: u32) -> Self {
        let BeatmapAttributes {
            ar,
            cs,
            hp,
            od,
            clock_rate,
        } = map.attributes().mods(mods);

        let bpm = map.bpm() * clock_rate;

        match attrs {
            PerformanceAttributes::Fruits(FruitsPerformanceAttributes { pp, difficulty }) => Self {
                mode: 2,
                pp,
                stars: difficulty.stars,
                max_combo: Some(difficulty.n_fruits + difficulty.n_droplets),
                n_fruits: Some(difficulty.n_fruits),
                n_droplets: Some(difficulty.n_droplets),
                n_tiny_droplets: Some(difficulty.n_tiny_droplets),
                n_spinners: Some(map.n_spinners as usize),
                ar,
                cs,
                hp,
                od,
                bpm,
                ..Default::default()
            },
            PerformanceAttributes::Mania(ManiaPerformanceAttributes {
                pp,
                pp_acc,
                pp_strain,
                difficulty,
            }) => Self {
                mode: 3,
                pp,
                pp_acc: Some(pp_acc),
                pp_strain: Some(pp_strain),
                stars: difficulty.stars,
                n_circles: Some(map.n_circles as usize),
                n_sliders: Some(map.n_sliders as usize),
                ar,
                cs,
                hp,
                od,
                bpm,
                ..Default::default()
            },
            PerformanceAttributes::Osu(OsuPerformanceAttributes {
                pp,
                pp_acc,
                pp_aim,
                pp_flashlight,
                pp_speed,
                difficulty,
            }) => Self {
                mode: 0,
                pp,
                pp_acc: Some(pp_acc),
                pp_aim: Some(pp_aim),
                pp_flashlight: Some(pp_flashlight),
                pp_speed: Some(pp_speed),
                stars: difficulty.stars,
                max_combo: Some(difficulty.max_combo),
                aim_strain: Some(difficulty.aim_strain),
                speed_strain: Some(difficulty.speed_strain),
                flashlight_rating: Some(difficulty.flashlight_rating),
                slider_factor: Some(difficulty.slider_factor),
                n_circles: Some(difficulty.n_circles),
                n_sliders: Some(difficulty.n_sliders),
                n_spinners: Some(difficulty.n_spinners),
                ar,
                cs,
                hp,
                od,
                bpm,
                ..Default::default()
            },
            PerformanceAttributes::Taiko(TaikoPerformanceAttributes {
                pp,
                pp_acc,
                pp_strain,
                difficulty,
            }) => Self {
                mode: 1,
                pp,
                pp_acc: Some(pp_acc),
                pp_strain: Some(pp_strain),
                stars: difficulty.stars,
                max_combo: Some(difficulty.max_combo),
                n_circles: Some(map.n_circles as usize),
                n_sliders: Some(map.n_sliders as usize),
                n_spinners: Some(map.n_spinners as usize),
                ar,
                cs,
                hp,
                od,
                bpm,
                ..Default::default()
            },
        }
    }
}

fn unwind_error(cause: &str, mut e: &dyn StdError) -> String {
    let mut content = format!("{}: {}", cause, e);

    while let Some(src) = e.source() {
        let _ = writeln!(content, "  - caused by: {}", src);
        e = src;
    }

    content
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("calculate", calculate)?;

    Ok(())
}

impl<'de> Deserialize<'de> for CalculateArg {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(CalculateArgVisitor)
    }
}

struct CalculateArgVisitor;

impl<'de> Visitor<'de> for CalculateArgVisitor {
    type Value = CalculateArg;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("a PerformanceArg struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut path = None;
        let mut params = None;
        let mut mods = None;
        let mut n300 = None;
        let mut n100 = None;
        let mut n50 = None;
        let mut n_misses = None;
        let mut n_katu = None;
        let mut acc = None;
        let mut combo = None;
        let mut score = None;
        let mut passed_objects = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "path" => path = Some(map.next_value()?),
                "params" => params = Some(map.next_value()?),
                "mods" => mods = Some(map.next_value()?),
                "n300" => n300 = Some(map.next_value()?),
                "n100" => n100 = Some(map.next_value()?),
                "n50" => n50 = Some(map.next_value()?),
                "nMisses" => n_misses = Some(map.next_value()?),
                "nKatu" => n_katu = Some(map.next_value()?),
                "acc" => acc = Some(map.next_value()?),
                "combo" => combo = Some(map.next_value()?),
                "score" => score = Some(map.next_value()?),
                "passedObjects" => passed_objects = Some(map.next_value()?),
                _ => {
                    return Err(DeError::unknown_field(
                        key.as_str(),
                        &[
                            "path",
                            "params",
                            "mods",
                            "n300",
                            "n100",
                            "n50",
                            "nMisses",
                            "nKatu",
                            "acc",
                            "combo",
                            "score",
                            "passedObjects",
                        ],
                    ))
                }
            }
        }

        let path = path.ok_or_else(|| DeError::missing_field("path"))?;

        let params = match params {
            Some(p) => p,
            None => {
                let params = ScoreParams {
                    mods: mods.unwrap_or(0),
                    n300,
                    n100,
                    n50,
                    n_misses,
                    n_katu,
                    acc,
                    combo,
                    score,
                    passed_objects,
                };

                vec![params]
            }
        };

        Ok(CalculateArg { path, params })
    }
}
