use std::{
    borrow::Cow,
    cell::{Ref, RefCell},
};

use neon::{
    prelude::{Context, FunctionContext, Object},
    result::{JsResult, NeonResult},
    types::{Finalize, JsArray, JsBox, JsNumber, JsObject, JsString, JsUndefined},
};
use rosu_pp::{
    beatmap::BeatmapAttributes,
    catch::{CatchDifficultyAttributes, CatchPerformanceAttributes, CatchStrains},
    mania::{ManiaDifficultyAttributes, ManiaPerformanceAttributes, ManiaStrains},
    osu::{OsuDifficultyAttributes, OsuPerformanceAttributes, OsuStrains},
    taiko::{TaikoDifficultyAttributes, TaikoPerformanceAttributes, TaikoStrains},
    Beatmap, BeatmapExt, DifficultyAttributes, GameMode, PerformanceAttributes, Strains,
};

use crate::beatmap::Map;

macro_rules! set_field {
    ($fun:ident, $field:ident $(as $ty:ty)?) => {
        pub fn $fun(mut cx: FunctionContext<'_>) -> JsResult<'_, JsUndefined> {
            let arg_opt = cx
                .argument_opt(0)
                .and_then(|arg| arg.downcast::<JsNumber, _>(&mut cx).ok());

            let val = if let Some(arg) = arg_opt {
                arg.value(&mut cx)
            } else {
                return cx.throw_error("The first argument must be a number");
            };

            let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
            this.inner.borrow_mut().$field = Some(val $(as $ty)?);

            Ok(JsUndefined::new(&mut cx))
        }
    };
}

pub struct Calculator {
    inner: RefCell<CalculatorInner>,
}

#[derive(Default)]
pub struct CalculatorInner {
    mode: Option<GameMode>,
    mods: Option<u32>,
    accuracy: Option<f64>,
    n_geki: Option<usize>,
    n_katu: Option<usize>,
    n300: Option<usize>,
    n100: Option<usize>,
    n50: Option<usize>,
    n_misses: Option<usize>,
    combo: Option<usize>,
    passed_objects: Option<usize>,
    clock_rate: Option<f64>,
}

macro_rules! set_calc {
    ( $calc:ident, $this:ident: $( $field:ident ,)* ) => {
        $(
            if let Some(val) = $this.$field {
                $calc = $calc.$field(val);
            }
        )*
    };
}

impl Calculator {
    pub fn js_new(mut cx: FunctionContext<'_>) -> JsResult<'_, JsBox<Self>> {
        let arg = match cx
            .argument_opt(0)
            .filter(|arg| !arg.is_a::<JsUndefined, _>(&mut cx))
        {
            Some(arg) => arg,
            None => {
                let inner = RefCell::new(CalculatorInner::default());

                return Ok(cx.boxed(Self { inner }));
            }
        };

        let obj = match arg.downcast::<JsObject, _>(&mut cx) {
            Ok(obj) => obj,
            Err(_) => return cx.throw_error("The optional first argument must be an object"),
        };

        let mut calculator = CalculatorInner::default();

        let val = obj.get_value(&mut cx, "mode")?;

        if let Ok(mode) = val.downcast::<JsNumber, _>(&mut cx) {
            let mode = match mode.value(&mut cx) as u8 {
                0 => GameMode::Osu,
                1 => GameMode::Taiko,
                2 => GameMode::Catch,
                3 => GameMode::Mania,
                _ => return cx.throw_error("Number for `mode` must be 0, 1, 2, or 3"),
            };

            calculator.mode = Some(mode);
        } else if let Ok(mode) = val.downcast::<JsString, _>(&mut cx) {
            let mut mode = mode.value(&mut cx);
            mode.make_ascii_lowercase();

            match Self::mode_from_str(mode) {
                Ok(mode) => calculator.mode = Some(mode),
                Err(err) => return cx.throw_error(err),
            }
        } else if !val.is_a::<JsUndefined, _>(&mut cx) {
            return cx.throw_error("The `mode` property must be a number or a string");
        };

        macro_rules! parse_property {
            ( $( $property:literal: $field:ident $( as $ty:ty )? ,)* ) => {
                $(
                    let val = obj.get_value(&mut cx, $property)?;

                    if let Ok(num) = val.downcast::<JsNumber, _>(&mut cx) {
                        calculator.$field = Some(num.value(&mut cx) $( as $ty )?);
                    } else if !val.is_a::<JsUndefined, _>(&mut cx) {
                        return cx.throw_error(concat!(
                            "The `", $property, "` property must be a number"
                        ));
                    }
                )*
            };
        }

        parse_property! {
            "mods": mods as u32,
            "acc": accuracy,
            "nGeki": n_geki as usize,
            "nKatu": n_katu as usize,
            "n300": n300 as usize,
            "n100": n100 as usize,
            "n50": n50 as usize,
            "nMisses": n_misses as usize,
            "combo": combo as usize,
            "passedObjects": passed_objects as usize,
            "clockRate": clock_rate,
        }

        let inner = RefCell::new(calculator);

        Ok(cx.boxed(Self { inner }))
    }

    pub fn js_map_attrs(mut cx: FunctionContext<'_>) -> JsResult<'_, JsObject> {
        let map_opt = cx
            .argument_opt(0)
            .and_then(|arg| arg.downcast::<JsObject, _>(&mut cx).ok())
            .and_then(|handle| handle.get_value(&mut cx, "map").ok())
            .and_then(|handle| handle.downcast::<JsBox<Map>, _>(&mut cx).ok());

        let map = match map_opt {
            Some(map) => map,
            None => return cx.throw_error("The first argument must be a Beatmap"),
        };

        let map = Ref::filter_map(map.inner.borrow(), Option::as_ref)
            .or_else(|_| cx.throw_error("Beatmap must be parsed first"))?;

        let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
        let this = this.inner.borrow();

        let (map, mode) = match this.mode {
            Some(mode) => (map.convert_mode(mode), mode),
            None => (Cow::Borrowed(&*map), map.mode),
        };

        let bpm = map.bpm();
        let mut calc = map.attributes();

        if let Some(mode) = this.mode {
            calc.mode(mode);

            if map.mode != mode && map.mode == GameMode::Osu {
                calc.converted(true);
            }
        }

        if let Some(mods) = this.mods {
            calc.mods(mods);
        }

        if let Some(clock_rate) = this.clock_rate {
            calc.clock_rate(clock_rate);
        }

        Self::convert_map_attrs(&mut cx, calc.build(), bpm, mode, map.as_ref())
    }

    pub fn js_difficulty(mut cx: FunctionContext<'_>) -> JsResult<'_, JsObject> {
        let map_opt = cx
            .argument_opt(0)
            .and_then(|arg| arg.downcast::<JsObject, _>(&mut cx).ok())
            .and_then(|handle| handle.get_value(&mut cx, "map").ok())
            .and_then(|handle| handle.downcast::<JsBox<Map>, _>(&mut cx).ok());

        let map = match map_opt {
            Some(map) => map,
            None => return cx.throw_error("The first argument must be a Beatmap"),
        };

        let map = Ref::filter_map(map.inner.borrow(), Option::as_ref)
            .or_else(|_| cx.throw_error("Beatmap must be parsed first"))?;

        let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
        let this = this.inner.borrow();

        let mut calc = map.stars();

        set_calc! { calc, this:
            mode,
            mods,
            passed_objects,
            clock_rate,
        };

        Self::convert_difficulty(&mut cx, calc.calculate())
    }

    pub fn js_performance(mut cx: FunctionContext<'_>) -> JsResult<'_, JsObject> {
        let map_opt = cx
            .argument_opt(0)
            .and_then(|arg| arg.downcast::<JsObject, _>(&mut cx).ok())
            .and_then(|handle| handle.get_value(&mut cx, "map").ok())
            .and_then(|handle| handle.downcast::<JsBox<Map>, _>(&mut cx).ok());

        let map = match map_opt {
            Some(map) => map,
            None => return cx.throw_error("The first argument must be a Beatmap"),
        };

        let map = Ref::filter_map(map.inner.borrow(), Option::as_ref)
            .or_else(|_| cx.throw_error("Beatmap must be parsed first"))?;

        let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
        let this = this.inner.borrow();

        let mut calc = map.pp();

        set_calc! { calc, this:
            mode,
            mods,
            accuracy,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            n_misses,
            combo,
            passed_objects,
            clock_rate,
        };

        Self::convert_performance(&mut cx, calc.calculate())
    }

    pub fn js_strains(mut cx: FunctionContext<'_>) -> JsResult<'_, JsObject> {
        let map_opt = cx
            .argument_opt(0)
            .and_then(|arg| arg.downcast::<JsObject, _>(&mut cx).ok())
            .and_then(|handle| handle.get_value(&mut cx, "map").ok())
            .and_then(|handle| handle.downcast::<JsBox<Map>, _>(&mut cx).ok());

        let map = match map_opt {
            Some(map) => map,
            None => return cx.throw_error("The first argument must be a Beatmap"),
        };

        let map = Ref::filter_map(map.inner.borrow(), Option::as_ref)
            .or_else(|_| cx.throw_error("Beatmap must be parsed first"))?;

        let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
        let this = this.inner.borrow();

        let mut calc = map.stars();

        set_calc! { calc, this:
            mode,
            mods,
            passed_objects,
            clock_rate,
        };

        Self::convert_strains(&mut cx, calc.strains())
    }

    pub fn js_mode(mut cx: FunctionContext<'_>) -> JsResult<'_, JsUndefined> {
        let arg = match cx.argument_opt(0) {
            Some(arg) => arg,
            None => return cx.throw_error("The first argument must be a GameMode"),
        };

        let mode = if let Ok(mode) = arg.downcast::<JsNumber, _>(&mut cx) {
            match mode.value(&mut cx) as u8 {
                0 => GameMode::Osu,
                1 => GameMode::Taiko,
                2 => GameMode::Catch,
                3 => GameMode::Mania,
                _ => return cx.throw_error("Number must be 0, 1, 2, or 3"),
            }
        } else if let Ok(mode) = arg.downcast::<JsString, _>(&mut cx) {
            let mut mode = mode.value(&mut cx);
            mode.make_ascii_lowercase();

            match Self::mode_from_str(mode) {
                Ok(mode) => mode,
                Err(err) => return cx.throw_error(err),
            }
        } else {
            return cx.throw_error("The first argument must be a number or a string");
        };

        let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
        this.inner.borrow_mut().mode = Some(mode);

        Ok(JsUndefined::new(&mut cx))
    }

    set_field!(js_mods, mods as u32);
    set_field!(js_acc, accuracy);
    set_field!(js_geki, n_geki as usize);
    set_field!(js_katu, n_katu as usize);
    set_field!(js_n300, n300 as usize);
    set_field!(js_n100, n100 as usize);
    set_field!(js_n50, n50 as usize);
    set_field!(js_misses, n_misses as usize);
    set_field!(js_combo, combo as usize);
    set_field!(js_passed_objects, passed_objects as usize);
    set_field!(js_clock_rate, clock_rate);
}

impl Finalize for Calculator {}

impl Calculator {
    fn mode_from_str(mode: impl AsRef<str>) -> Result<GameMode, &'static str> {
        match mode.as_ref() {
            "osu" | "o" | "std" | "standard" => Ok(GameMode::Osu),
            "taiko" | "t" | "tko" => Ok(GameMode::Taiko),
            "catch" | "c" | "ctb" | "catch the beat" => Ok(GameMode::Catch),
            "mania" | "m" | "mna" => Ok(GameMode::Mania),
            _ => Err("String for `mode` must be `osu`, `taiko`, `catch`, or `mania`"),
        }
    }

    fn convert_map_attrs<'c>(
        cx: &mut FunctionContext<'c>,
        attrs: BeatmapAttributes,
        bpm: f64,
        mode: GameMode,
        map: &Beatmap,
    ) -> JsResult<'c, JsObject> {
        let res = JsObject::new(cx);

        Self::set_number(cx, "mode", &res, mode as u8)?;
        Self::set_number(cx, "version", &res, map.version)?;
        Self::set_number(cx, "nCircles", &res, map.n_circles)?;
        Self::set_number(cx, "nSliders", &res, map.n_sliders)?;
        Self::set_number(cx, "nSpinners", &res, map.n_spinners)?;
        Self::set_number(cx, "ar", &res, attrs.ar)?;
        Self::set_number(cx, "cs", &res, attrs.cs)?;
        Self::set_number(cx, "hp", &res, attrs.hp)?;
        Self::set_number(cx, "od", &res, attrs.od)?;
        Self::set_number(cx, "arHitWindow", &res, attrs.hit_windows.ar)?;
        Self::set_number(cx, "odHitWindow", &res, attrs.hit_windows.od)?;
        Self::set_number(cx, "clockRate", &res, attrs.clock_rate)?;
        Self::set_number(cx, "bpm", &res, bpm * attrs.clock_rate)?;

        Ok(res)
    }

    fn convert_difficulty<'c>(
        cx: &mut FunctionContext<'c>,
        attrs: DifficultyAttributes,
    ) -> JsResult<'c, JsObject> {
        let res = JsObject::new(cx);

        match attrs {
            DifficultyAttributes::Osu(OsuDifficultyAttributes {
                aim,
                speed,
                flashlight,
                slider_factor,
                speed_note_count,
                ar,
                od,
                hp: _,
                n_circles,
                n_sliders,
                n_spinners,
                stars,
                max_combo,
            }) => {
                Self::set_number(cx, "mode", &res, 0)?;
                Self::set_number(cx, "aim", &res, aim)?;
                Self::set_number(cx, "speed", &res, speed)?;
                Self::set_number(cx, "flashlight", &res, flashlight)?;
                Self::set_number(cx, "sliderFactor", &res, slider_factor)?;
                Self::set_number(cx, "speedNoteCount", &res, speed_note_count)?;
                Self::set_number(cx, "ar", &res, ar)?;
                Self::set_number(cx, "od", &res, od)?;
                Self::set_number(cx, "nCircles", &res, n_circles as f64)?;
                Self::set_number(cx, "nSliders", &res, n_sliders as f64)?;
                Self::set_number(cx, "nSpinners", &res, n_spinners as f64)?;
                Self::set_number(cx, "stars", &res, stars)?;
                Self::set_number(cx, "maxCombo", &res, max_combo as f64)?;
            }
            DifficultyAttributes::Taiko(TaikoDifficultyAttributes {
                stamina,
                rhythm,
                colour,
                peak,
                hit_window,
                stars,
                max_combo,
            }) => {
                Self::set_number(cx, "mode", &res, 1)?;
                Self::set_number(cx, "stamina", &res, stamina)?;
                Self::set_number(cx, "rhythm", &res, rhythm)?;
                Self::set_number(cx, "color", &res, colour)?;
                Self::set_number(cx, "peak", &res, peak)?;
                Self::set_number(cx, "hitWindow", &res, hit_window)?;
                Self::set_number(cx, "stars", &res, stars)?;
                Self::set_number(cx, "maxCombo", &res, max_combo as f64)?;
            }
            DifficultyAttributes::Catch(CatchDifficultyAttributes {
                stars,
                ar,
                n_fruits,
                n_droplets,
                n_tiny_droplets,
            }) => {
                Self::set_number(cx, "mode", &res, 2)?;
                Self::set_number(cx, "stars", &res, stars)?;
                Self::set_number(cx, "ar", &res, ar)?;
                Self::set_number(cx, "nFruits", &res, n_fruits as f64)?;
                Self::set_number(cx, "nDroplets", &res, n_droplets as f64)?;
                Self::set_number(cx, "nTinyDroplets", &res, n_tiny_droplets as f64)?;
                Self::set_number(cx, "maxCombo", &res, (n_fruits + n_droplets) as f64)?;
            }
            DifficultyAttributes::Mania(ManiaDifficultyAttributes {
                stars,
                hit_window,
                max_combo,
            }) => {
                Self::set_number(cx, "mode", &res, 3)?;
                Self::set_number(cx, "stars", &res, stars)?;
                Self::set_number(cx, "hitWindow", &res, hit_window)?;
                Self::set_number(cx, "maxCombo", &res, max_combo as f64)?;
            }
        }

        Ok(res)
    }

    fn convert_performance<'c>(
        cx: &mut FunctionContext<'c>,
        attrs: PerformanceAttributes,
    ) -> JsResult<'c, JsObject> {
        let res = JsObject::new(cx);

        match attrs {
            PerformanceAttributes::Osu(OsuPerformanceAttributes {
                difficulty,
                pp,
                pp_acc,
                pp_aim,
                pp_flashlight,
                pp_speed,
                effective_miss_count,
            }) => {
                Self::set_number(cx, "mode", &res, 0)?;
                Self::set_number(cx, "pp", &res, pp)?;
                Self::set_number(cx, "ppAcc", &res, pp_acc)?;
                Self::set_number(cx, "ppAim", &res, pp_aim)?;
                Self::set_number(cx, "ppFlashlight", &res, pp_flashlight)?;
                Self::set_number(cx, "ppSpeed", &res, pp_speed)?;
                Self::set_number(cx, "effectiveMissCount", &res, effective_miss_count)?;

                let diff = Self::convert_difficulty(cx, DifficultyAttributes::Osu(difficulty))?;
                res.set(cx, "difficulty", diff)?;
            }
            PerformanceAttributes::Taiko(TaikoPerformanceAttributes {
                difficulty,
                pp,
                pp_acc,
                pp_difficulty,
                effective_miss_count,
            }) => {
                Self::set_number(cx, "mode", &res, 1)?;
                Self::set_number(cx, "pp", &res, pp)?;
                Self::set_number(cx, "ppAcc", &res, pp_acc)?;
                Self::set_number(cx, "ppDifficulty", &res, pp_difficulty)?;
                Self::set_number(cx, "effectiveMissCount", &res, effective_miss_count)?;

                let diff = Self::convert_difficulty(cx, DifficultyAttributes::Taiko(difficulty))?;
                res.set(cx, "difficulty", diff)?;
            }
            PerformanceAttributes::Catch(CatchPerformanceAttributes { difficulty, pp }) => {
                Self::set_number(cx, "mode", &res, 2)?;
                Self::set_number(cx, "pp", &res, pp)?;

                let diff = Self::convert_difficulty(cx, DifficultyAttributes::Catch(difficulty))?;
                res.set(cx, "difficulty", diff)?;
            }
            PerformanceAttributes::Mania(ManiaPerformanceAttributes {
                difficulty,
                pp,
                pp_difficulty,
            }) => {
                Self::set_number(cx, "mode", &res, 3)?;
                Self::set_number(cx, "pp", &res, pp)?;
                Self::set_number(cx, "ppDifficulty", &res, pp_difficulty)?;

                let diff = Self::convert_difficulty(cx, DifficultyAttributes::Mania(difficulty))?;
                res.set(cx, "difficulty", diff)?;
            }
        };

        Ok(res)
    }

    fn convert_strains<'c>(
        cx: &mut FunctionContext<'c>,
        strains: Strains,
    ) -> JsResult<'c, JsObject> {
        let res = JsObject::new(cx);

        match strains {
            Strains::Osu(OsuStrains {
                section_len,
                aim,
                aim_no_sliders,
                speed,
                flashlight,
            }) => {
                Self::set_number(cx, "mode", &res, 0)?;
                Self::set_number(cx, "sectionLength", &res, section_len)?;
                Self::set_array(cx, "aim", &res, aim)?;
                Self::set_array(cx, "aimNoSliders", &res, aim_no_sliders)?;
                Self::set_array(cx, "speed", &res, speed)?;
                Self::set_array(cx, "flashlight", &res, flashlight)?;
            }
            Strains::Taiko(TaikoStrains {
                section_len,
                color,
                rhythm,
                stamina,
            }) => {
                Self::set_number(cx, "mode", &res, 1)?;
                Self::set_number(cx, "sectionLength", &res, section_len)?;
                Self::set_array(cx, "color", &res, color)?;
                Self::set_array(cx, "rhythm", &res, rhythm)?;
                Self::set_array(cx, "stamina", &res, stamina)?;
            }
            Strains::Catch(CatchStrains {
                section_len,
                movement,
            }) => {
                Self::set_number(cx, "mode", &res, 2)?;
                Self::set_number(cx, "sectionLength", &res, section_len)?;
                Self::set_array(cx, "movement", &res, movement)?;
            }
            Strains::Mania(ManiaStrains {
                section_len,
                strains,
            }) => {
                Self::set_number(cx, "mode", &res, 3)?;
                Self::set_number(cx, "sectionLength", &res, section_len)?;
                Self::set_array(cx, "strains", &res, strains)?;
            }
        }

        Ok(res)
    }

    fn set_number(
        cx: &mut FunctionContext<'_>,
        name: &str,
        obj: &JsObject,
        num: impl Into<f64>,
    ) -> NeonResult<()> {
        let property = JsNumber::new(cx, num);
        obj.set(cx, name, property)?;

        Ok(())
    }

    fn set_array(
        cx: &mut FunctionContext<'_>,
        name: &str,
        obj: &JsObject,
        strains: Vec<f64>,
    ) -> NeonResult<()> {
        let arr = JsArray::new(cx, strains.len() as u32);

        for (strain, i) in strains.into_iter().zip(0..) {
            let strain = JsNumber::new(cx, strain);
            arr.set(cx, i, strain)?;
        }

        obj.set(cx, name, arr)?;

        Ok(())
    }
}
