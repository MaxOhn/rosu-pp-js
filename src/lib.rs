use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    error::Error as StdError,
    fmt::{Formatter, Result as FmtResult, Write},
    hash::{Hash, Hasher},
    mem,
};

use neon::prelude::*;
use rosu_pp::{
    beatmap::BeatmapAttributes, catch::CatchPerformanceAttributes,
    mania::ManiaPerformanceAttributes, osu::OsuPerformanceAttributes,
    taiko::TaikoPerformanceAttributes, AnyPP, Beatmap, BeatmapExt, GameMode, PerformanceAttributes,
    Strains as RosuStrains,
};
use serde::{
    de::{Error as DeError, MapAccess, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};

fn strains(mut cx: FunctionContext) -> JsResult<JsValue> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);

    let map = Beatmap::from_path(path)
        .map_err(|e| unwind_error("Failed to parse beatmap", &e))
        .or_else(|e| cx.throw_error(e))?;

    let mods: u32 = match cx.argument_opt(1) {
        Some(arg) => neon_serde2::from_value(&mut cx, arg).or_else(|_| {
            cx.throw_error("The optional second argument must be an integer for mods")
        })?,
        None => 0,
    };

    let strains = Strains::from(map.strains(mods));

    neon_serde2::to_value(&mut cx, &strains)
        .map_err(|e| unwind_error("Failed to serialize results", &e))
        .or_else(|e| cx.throw_error(e))
}

fn calculate(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg = cx.argument::<JsValue>(0)?;

    let CalculateArg { path, params } = neon_serde2::from_value(&mut cx, arg)
        .map_err(|e| unwind_error("Failed to deserialize argument", &e))
        .or_else(|e| cx.throw_error(e))?;

    if params.is_empty() {
        return Ok(JsArray::new(&mut cx, 0).as_value(&mut cx));
    }

    let mut map = Beatmap::from_path(path)
        .map_err(|e| unwind_error("Failed to parse beatmap", &e))
        .or_else(|e| cx.throw_error(e))?;

    // Avoid caching if it's not necessary
    let results: Vec<_> = if multiple_same_attributes(&params) {
        let mut attrs_seen = HashMap::new();

        params
            .into_iter()
            .map(|params| {
                let attr_key = params.as_attr_key();
                let mut attr_switcher = attr_key.attr_switcher;
                let mods = params.mods;
                let clock_rate = params.clock_rate.map(|rate| rate as f64);

                let difficulty = attrs_seen
                    .entry(attr_key)
                    .or_insert_with(|| {
                        attr_switcher.apply(&mut map);
                        let mut calculator = map.stars().mods(mods);

                        if let Some(passed_objects) = params.passed_objects {
                            calculator = calculator.passed_objects(passed_objects);
                        }

                        if let Some(clock_rate) = clock_rate {
                            calculator = calculator.clock_rate(clock_rate);
                        }

                        let attrs = calculator.calculate();
                        attr_switcher.reset(&mut map);

                        attrs
                    })
                    .to_owned();

                attr_switcher.apply(&mut map);
                let attrs = params.apply(AnyPP::new(&map).attributes(difficulty));
                let result = CalculateResult::new(attrs, &map, mods, clock_rate);
                attr_switcher.reset(&mut map);

                result
            })
            .collect()
    } else {
        params
            .into_iter()
            .map(|params| {
                let mods = params.mods;
                let clock_rate = params.clock_rate.map(|rate| rate as f64);
                let mut attr_switcher = params.as_attr_switcher();

                attr_switcher.apply(&mut map);
                let attrs = params.apply(AnyPP::new(&map));
                let result = CalculateResult::new(attrs, &map, mods, clock_rate);
                attr_switcher.reset(&mut map);

                result
            })
            .collect()
    };

    neon_serde2::to_value(&mut cx, &results)
        .map_err(|e| unwind_error("Failed to serialize results", &e))
        .or_else(|e| cx.throw_error(e))
}

fn multiple_same_attributes(params: &[ScoreParams]) -> bool {
    if params.len() <= 1 {
        return false;
    }

    let mut attrs_seen = HashSet::with_capacity(params.len());

    for param in params {
        if !attrs_seen.insert(param.as_attr_key()) {
            return true;
        }
    }

    false
}

#[derive(Clone, Default, PartialEq, Serialize)]
struct Strains {
    mode: u8,
    section_length: f64,

    color: Option<Vec<f64>>,
    rhythm: Option<Vec<f64>>,
    #[serde(rename = "staminaLeft")]
    stamina_left: Option<Vec<f64>>,
    #[serde(rename = "staminaRight")]
    stamina_right: Option<Vec<f64>>,

    aim: Option<Vec<f64>>,
    #[serde(rename = "aimNoSliders")]
    aim_no_sliders: Option<Vec<f64>>,
    speed: Option<Vec<f64>>,
    flashlight: Option<Vec<f64>>,

    strains: Option<Vec<f64>>,

    movement: Option<Vec<f64>>,
}

impl From<RosuStrains> for Strains {
    #[inline]
    fn from(strains: RosuStrains) -> Self {
        match strains {
            RosuStrains::Catch(strains) => Self {
                mode: 2,
                section_length: strains.section_len,
                movement: Some(strains.movement),
                ..Default::default()
            },
            RosuStrains::Mania(strains) => Self {
                mode: 3,
                section_length: strains.section_len,
                strains: Some(strains.strains),
                ..Default::default()
            },
            RosuStrains::Osu(strains) => Self {
                mode: 0,
                section_length: strains.section_len,
                aim: Some(strains.aim),
                aim_no_sliders: Some(strains.aim_no_sliders),
                speed: Some(strains.speed),
                flashlight: Some(strains.flashlight),
                ..Default::default()
            },
            RosuStrains::Taiko(strains) => Self {
                mode: 1,
                section_length: strains.section_len,
                color: Some(strains.color),
                rhythm: Some(strains.rhythm),
                stamina_left: Some(strains.stamina_left),
                stamina_right: Some(strains.stamina_right),
                ..Default::default()
            },
        }
    }
}

struct CalculateArg {
    path: String,
    params: Vec<ScoreParams>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ScoreParams {
    #[serde(default, deserialize_with = "deserialize_mode")]
    mode: Option<GameMode>,
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
    #[serde(rename = "clockRate")]
    clock_rate: Option<f32>,
    ar: Option<f32>,
    cs: Option<f32>,
    hp: Option<f32>,
    od: Option<f32>,
}

#[derive(Eq, Hash, PartialEq)]
struct AttributeKey {
    mode: Option<GameMode>,
    mods: u32,
    passed_objects: Option<usize>,
    attr_switcher: AttributeSwitcher,
}

impl ScoreParams {
    fn as_attr_switcher(&self) -> AttributeSwitcher {
        AttributeSwitcher::new(self.ar, self.cs, self.hp, self.od, self.clock_rate)
    }

    fn as_attr_key(&self) -> AttributeKey {
        AttributeKey {
            mode: self.mode,
            mods: self.mods,
            passed_objects: self.passed_objects,
            attr_switcher: self.as_attr_switcher(),
        }
    }

    fn apply(self, mut calculator: AnyPP<'_>) -> PerformanceAttributes {
        let Self {
            mode,
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
            clock_rate,
            ..
        } = self;

        if let Some(mode) = mode {
            calculator = calculator.mode(mode);
        }

        calculator = calculator.mods(mods);

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

        if let Some(clock_rate) = clock_rate {
            calculator = calculator.clock_rate(clock_rate as f64);
        }

        calculator.calculate()
    }
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
    #[serde(rename = "clockRate")]
    clock_rate: f64,
    #[serde(rename = "timePreempt")]
    time_preempt: Option<f64>,
    #[serde(rename = "greatHitWindow")]
    great_hitwindow: Option<f64>,
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
    fn new(
        attrs: PerformanceAttributes,
        map: &Beatmap,
        mods: u32,
        clock_rate: Option<f64>,
    ) -> Self {
        let mut attr_builder = map.attributes();

        if let Some(clock_rate) = clock_rate {
            attr_builder.clock_rate(clock_rate);
        }

        let mode = match &attrs {
            PerformanceAttributes::Catch(_) => GameMode::Catch,
            PerformanceAttributes::Mania(_) => GameMode::Mania,
            PerformanceAttributes::Osu(_) => GameMode::Osu,
            PerformanceAttributes::Taiko(_) => GameMode::Taiko,
        };

        if map.mode == GameMode::Osu && mode != GameMode::Osu {
            attr_builder.converted(true);
        }

        let BeatmapAttributes {
            ar,
            cs,
            hp,
            od,
            clock_rate,
            hit_windows,
        } = attr_builder.mods(mods).mode(mode).build();

        let bpm = map.bpm() * clock_rate;

        match attrs {
            PerformanceAttributes::Catch(CatchPerformanceAttributes { pp, difficulty }) => Self {
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
                clock_rate,
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
                clock_rate,
                great_hitwindow: Some(hit_windows.od),
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
                clock_rate,
                time_preempt: Some(hit_windows.ar),
                great_hitwindow: Some(hit_windows.od),
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
                clock_rate,
                great_hitwindow: Some(hit_windows.od),
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
    cx.export_function("strains", strains)?;

    Ok(())
}

struct GameModeWrapper(Option<GameMode>);

impl<'de> Deserialize<'de> for GameModeWrapper {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        deserialize_mode(d).map(Self)
    }
}

impl From<GameModeWrapper> for Option<GameMode> {
    #[inline]
    fn from(wrapper: GameModeWrapper) -> Self {
        wrapper.0
    }
}

fn deserialize_mode<'de, D: Deserializer<'de>>(d: D) -> Result<Option<GameMode>, D::Error> {
    d.deserialize_any(GameModeVisitor).map(Some)
}

struct GameModeVisitor;

static GAMEMODE_VISITOR_EXPECTS: &str =
    r#"integer 0, 1, 2, 3 or string "o", "t", "c", "m", "osu", "taiko", "ctb", "catch", "mania""#;

impl<'de> Visitor<'de> for GameModeVisitor {
    type Value = GameMode;

    #[inline]
    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(GAMEMODE_VISITOR_EXPECTS)
    }

    #[inline]
    fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        self.visit_i64(v.try_into().unwrap_or_default())
    }

    #[inline]
    fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
        let mode = match v {
            0 => GameMode::Osu,
            1 => GameMode::Taiko,
            2 => GameMode::Catch,
            3 => GameMode::Mania,
            _ => {
                return Err(DeError::invalid_value(
                    Unexpected::Signed(v),
                    &GAMEMODE_VISITOR_EXPECTS,
                ))
            }
        };

        Ok(mode)
    }

    #[inline]
    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        let mode = match v {
            "0" | "o" | "osu" | "osu!" | "std" | "standard" => GameMode::Osu,
            "1" | "t" | "taiko" | "tko" => GameMode::Taiko,
            "2" | "c" | "ctb" | "catch" | "catch the beat" => GameMode::Catch,
            "3" | "m" | "mania" | "mna" => GameMode::Mania,
            _ => {
                return Err(DeError::invalid_value(
                    Unexpected::Str(v),
                    &GAMEMODE_VISITOR_EXPECTS,
                ))
            }
        };

        Ok(mode)
    }
}

impl<'de> Deserialize<'de> for CalculateArg {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(CalculateArgVisitor)
    }
}

struct CalculateArgVisitor;

impl<'de> Visitor<'de> for CalculateArgVisitor {
    type Value = CalculateArg;

    #[inline]
    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("a PerformanceArg struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut path = None;
        let mut params = None;
        let mut mode = None;
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
        let mut clock_rate = None;
        let mut ar = None;
        let mut cs = None;
        let mut hp = None;
        let mut od = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "path" => path = Some(map.next_value()?),
                "params" => params = Some(map.next_value()?),
                "mode" => mode = Some(map.next_value()?),
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
                "clockRate" => clock_rate = Some(map.next_value()?),
                "ar" => ar = Some(map.next_value()?),
                "cs" => cs = Some(map.next_value()?),
                "hp" => hp = Some(map.next_value()?),
                "od" => od = Some(map.next_value()?),
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
                            "clockRate",
                            "ar",
                            "cs",
                            "hp",
                            "od",
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
                    mode: mode.and_then(GameModeWrapper::into),
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
                    clock_rate,
                    ar,
                    cs,
                    hp,
                    od,
                };

                vec![params]
            }
        };

        Ok(CalculateArg { path, params })
    }
}

#[derive(Copy, Clone, Debug)]
struct AttributeSwitcher {
    ar: Option<f32>,
    cs: Option<f32>,
    hp: Option<f32>,
    od: Option<f32>,
    clock_rate: Option<f32>,
}

impl AttributeSwitcher {
    fn new(
        ar: Option<f32>,
        cs: Option<f32>,
        hp: Option<f32>,
        od: Option<f32>,
        clock_rate: Option<f32>,
    ) -> Self {
        Self {
            ar,
            cs,
            hp,
            od,
            clock_rate,
        }
    }

    fn apply(&mut self, map: &mut Beatmap) {
        if let Some(ref mut ar) = self.ar {
            mem::swap(ar, &mut map.ar);
        }
        if let Some(ref mut cs) = self.cs {
            mem::swap(cs, &mut map.cs);
        }
        if let Some(ref mut hp) = self.hp {
            mem::swap(hp, &mut map.hp);
        }
        if let Some(ref mut od) = self.od {
            mem::swap(od, &mut map.od);
        }
    }

    fn reset(&mut self, map: &mut Beatmap) {
        self.apply(map);
    }
}

impl Hash for AttributeSwitcher {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&self.ar as *const _ as *const Option<u32>).hash(state);
        (&self.cs as *const _ as *const Option<u32>).hash(state);
        (&self.hp as *const _ as *const Option<u32>).hash(state);
        (&self.od as *const _ as *const Option<u32>).hash(state);
        (&self.clock_rate as *const _ as *const Option<u32>).hash(state);
    }
}

impl PartialEq for AttributeSwitcher {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ar == other.ar
            && self.cs == other.cs
            && self.hp == other.hp
            && self.od == other.od
            && self.clock_rate == other.clock_rate
    }
}

impl Eq for AttributeSwitcher {}
