#![allow(non_snake_case)]

use std::io;

use rosu_pp::{
    any::{DifficultyAttributes, HitResultPriority, PerformanceAttributes},
    catch::{CatchDifficultyAttributes, CatchPerformanceAttributes},
    mania::{ManiaDifficultyAttributes, ManiaPerformanceAttributes},
    model::mode::{ConvertStatus, GameMode},
    osu::{OsuDifficultyAttributes, OsuPerformanceAttributes},
    taiko::{TaikoDifficultyAttributes, TaikoPerformanceAttributes},
    Beatmap, Difficulty, Performance,
};
use wasm_bindgen::prelude::*;

use self::error::ErrorExt;

mod error;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const INVALID_DIFF_ATTRS: &str = "Invalid difficulty attributes";

/// All beatmap data that is relevant for difficulty and performance
/// calculation.
#[wasm_bindgen(js_name = Beatmap)]
pub struct JsBeatmap {
    inner: Beatmap,
}

#[wasm_bindgen(js_class = Beatmap)]
impl JsBeatmap {
    /// Parse a map by providing the content of a `.osu` file as a `Uint8Array`.
    /// @throws Will throw an error if decoding the map fails
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<JsBeatmap, String> {
        Self::new(Beatmap::from_bytes(bytes))
    }

    /// Parse a map by providing the content of a `.osu` file as a `string`.
    /// @throws Will throw an error if decoding the map fails
    #[wasm_bindgen(js_name = fromString)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<JsBeatmap, String> {
        Self::new(s.parse())
    }

    /// Convert the map to a given mode.
    /// @throws Will throw an error if the mode is incompatible e.g. cannot convert mania to taiko.
    pub fn convert(&mut self, mode: JsGameMode) -> Result<(), String> {
        self.convert_native(mode.into())
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> JsGameMode {
        self.inner.mode.into()
    }
}

impl JsBeatmap {
    fn new(res: Result<Beatmap, io::Error>) -> Result<JsBeatmap, String> {
        let inner = res.map_err(|e| e.unwind("Failed to decode beatmap"))?;

        Ok(Self { inner })
    }

    fn convert_native(&mut self, mode: GameMode) -> Result<(), String> {
        if let ConvertStatus::Incompatible = self.inner.convert_in_place(mode) {
            return Err(cannot_convert(self.inner.mode, mode));
        }

        Ok(())
    }
}

fn cannot_convert(from: GameMode, to: GameMode) -> String {
    format!("Cannot convert {from:?} to {to:?}")
}

#[wasm_bindgen(js_name = GameMode)]
#[derive(Copy, Clone, Debug, Default)]
pub enum JsGameMode {
    #[default]
    Osu,
    Taiko,
    Catch,
    Mania,
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

macro_rules! getters {
    // all getter names are specified
    ( $( $field:ident as $getter:ident: $ty:ty, )+ ) => {
        #[wasm_bindgen(js_class = Beatmap)]
        impl JsBeatmap {
            $(
                #[wasm_bindgen(getter)]
                pub fn $getter(&self) -> $ty {
                    self.inner.$field
                }
            )*
        }
    };
    // some getter name not specified, suffix "! !" and cycle through items
    ( $( $field:ident $( as $getter:ident )?: $ty:ty, )+ ) => {
        getters!($( $field $(as $getter)?: $ty,)* ! !);
    };
    // item at front misses getter name, append it with getter name and continue
    (
        $field:ident: $ty:ty, $(! $tt:tt)?
        $( $rest_field:ident $( as $rest_getter:ident )?: $rest_ty:ty, $(! $rest_tt:tt)? )+
    ) => {
        getters!( $(! $tt)? $( $rest_field $( as $rest_getter)?: $rest_ty, $(! $rest_tt)? )* $field as $field: $ty, );
    };
    // item at front has getter name, append it and continue
    (
        $field:ident as $getter:ident: $ty:ty, $(! $tt:tt)?
        $( $rest_field:ident $( as $rest_getter:ident )?: $rest_ty:ty, $(! $rest_tt:tt)? )+
    ) => {
        getters!( $(! $tt)? $( $rest_field $( as $rest_getter)?: $rest_ty, $(! $rest_tt)? )* $field as $getter: $ty, );
    };
    // initially suffixed "! !" are at the start so we cycled through everything
    ( ! ! $(  $rest_field:ident as $rest_getter:ident: $rest_ty:ty, )+) => {
        getters!( $( $rest_field as $rest_getter: $rest_ty, )* );
    };
}

getters! {
    version: i32,
    is_convert as isConvert: bool,
    stack_leniency as stackLeniency: f32,
    ar: f32,
    cs: f32,
    hp: f32,
    od: f32,
    slider_multiplier as sliderMultiplier: f64,
    slider_tick_rate as sliderTickRate: f64,

    // TODO
    // breaks: Vec<BreakPeriod>,
    // timing_points: Vec<TimingPoint>,
    // difficulty_points: Vec<DifficultyPoint>,
    // effect_points: Vec<EffectPoint>,
    // hit_objects: Vec<HitObject>,
    // hit_sounds: Vec<HitSoundType>,
}

/// Builder for a difficulty calculation.
#[wasm_bindgen(js_name = Difficulty)]
pub struct JsDifficulty {
    inner: Difficulty,
    mode: Option<GameMode>,
}

#[wasm_bindgen(js_class = Difficulty)]
impl JsDifficulty {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: Difficulty::new(),
            mode: None,
        }
    }

    /// Specify a gamemode.
    pub fn mode(self, mode: JsGameMode) -> Self {
        Self {
            inner: self.inner,
            mode: Some(mode.into()),
        }
    }

    /// Specify mods through their bit values.
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub fn mods(self, mods: u32) -> Self {
        Self {
            inner: self.inner.mods(mods),
            mode: self.mode,
        }
    }

    /// Amount of passed objects for partial plays, e.g. a fail.
    ///
    /// If you want to calculate the difficulty after every few objects,
    /// instead of using `Difficulty` multiple times with different
    /// `passedObjects`, you should use `GradualDifficulty`.
    pub fn passedObjects(self, passedObjects: u32) -> Self {
        Self {
            inner: self.inner.passed_objects(passedObjects),
            mode: self.mode,
        }
    }

    /// Adjust the clock rate used in the calculation.
    ///
    /// If none is specified, it will take the clock rate based on the mods
    /// i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | 0.01    | 100     |
    pub fn clockRate(self, clockRate: f64) -> Self {
        Self {
            inner: self.inner.clock_rate(clockRate),
            mode: self.mode,
        }
    }

    /// Override a beatmap's set AR.
    ///
    /// Only relevant for osu! and osu!catch.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn ar(self, ar: f32, withMods: bool) -> Self {
        Self {
            inner: self.inner.ar(ar, withMods),
            mode: self.mode,
        }
    }

    /// Override a beatmap's set CS.
    ///
    /// Only relevant for osu! and osu!catch.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn cs(self, cs: f32, withMods: bool) -> Self {
        Self {
            inner: self.inner.cs(cs, withMods),
            mode: self.mode,
        }
    }

    /// Override a beatmap's set HP.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn hp(self, hp: f32, withMods: bool) -> Self {
        Self {
            inner: self.inner.hp(hp, withMods),
            mode: self.mode,
        }
    }

    /// Override a beatmap's set OD.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn od(self, od: f32, withMods: bool) -> Self {
        Self {
            inner: self.inner.od(od, withMods),
            mode: self.mode,
        }
    }

    /// Adjust patterns as if the HR mod is enabled.
    ///
    /// Only relevant for osu!catch.
    pub fn hardrockOffsets(self, hardrockOffsets: bool) -> Self {
        Self {
            inner: self.inner.hardrock_offsets(hardrockOffsets),
            mode: self.mode,
        }
    }

    /// Perform the difficulty calculation.
    /// @throws Will throw an error if the specified mode is incompatible with the map's mode
    pub fn calculate(&self, map: &mut JsBeatmap) -> Result<JsDifficultyAttributes, String> {
        if let Some(mode) = self.mode {
            map.convert_native(mode)?;
        }

        Ok(self.inner.calculate(&map.inner).into())
    }

    pub fn performance(self) -> JsPerformance {
        JsPerformance::new(Some(self))
    }
}

#[wasm_bindgen(js_name = DifficultyAttributes, inspectable)]
#[derive(Clone, Debug, Default)]
pub struct JsDifficultyAttributes {
    /// The attributes' gamemode.
    #[wasm_bindgen(readonly)]
    pub mode: JsGameMode,

    /// The final star rating.
    #[wasm_bindgen(readonly)]
    pub stars: f64,

    /// Whether the map was a convert i.e. an osu! map.
    #[wasm_bindgen(readonly, js_name = isConvert)]
    pub is_convert: bool,

    /// The difficulty of the aim skill.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub aim: Option<f64>,

    /// The difficulty of the speed skill.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub speed: Option<f64>,

    /// The difficulty of the flashlight skill.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub flashlight: Option<f64>,

    /// The ratio of the aim strain with and without considering sliders
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly, js_name = sliderFactor)]
    pub slider_factor: Option<f64>,

    /// The number of clickable objects weighted by difficulty.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly, js_name = speedNoteCount)]
    pub speed_note_count: Option<f64>,

    /// The overall difficulty
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub od: Option<f64>,

    /// The health drain rate.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub hp: Option<f64>,

    /// The amount of circles.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly, js_name = nCircles)]
    pub n_circles: Option<u32>,

    /// The amount of sliders.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly, js_name = nSliders)]
    pub n_sliders: Option<u32>,

    /// The amount of spinners.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly, js_name = nSpinners)]
    pub n_spinners: Option<u32>,

    /// The difficulty of the stamina skill.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub stamina: Option<f64>,

    /// The difficulty of the rhythm skill.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub rhythm: Option<f64>,

    /// The difficulty of the color skill.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub color: Option<f64>,

    /// The difficulty of the hardest parts of the map.
    ///
    /// Only available for osu!taiko.
    #[wasm_bindgen(readonly)]
    pub peak: Option<f64>,

    /// The amount of fruits.
    ///
    /// Only available for osu!catch.
    #[wasm_bindgen(readonly, js_name = nFruits)]
    pub n_fruits: Option<u32>,

    /// The amount of droplets.
    ///
    /// Only available for osu!catch.
    #[wasm_bindgen(readonly, js_name = nDroplets)]
    pub n_droplets: Option<u32>,

    /// The amount of tiny droplets.
    ///
    /// Only available for osu!catch.
    #[wasm_bindgen(readonly, js_name = nTinyDroplets)]
    pub n_tiny_droplets: Option<u32>,

    /// The amount of hitobjects in the map.
    ///
    /// Only available for osu!mania.
    #[wasm_bindgen(readonly, js_name = nObjects)]
    pub n_objects: Option<u32>,

    /// The approach rate.
    ///
    /// Only available for osu! and osu!catch.
    #[wasm_bindgen(readonly)]
    pub ar: Option<f64>,

    /// The perceived hit window for an n300 inclusive of rate-adjusting mods (DT/HT/etc)
    ///
    /// Only available for osu!taiko and osu!mania.
    #[wasm_bindgen(readonly, js_name = hitWindow)]
    pub hit_window: Option<f64>,

    /// The maximum combo.
    ///
    /// Only available for osu!, osu!taiko, and osu!mania.
    /// It's recommended to use the `maxCombo` method instead.
    #[wasm_bindgen(readonly, js_name = maxCombo)]
    pub max_combo: Option<u32>,
}

#[wasm_bindgen(js_class = DifficultyAttributes)]
impl JsDifficultyAttributes {
    /// Return the maximum combo.
    /// @throws Will throw an error if the attributes have been modified manually
    pub fn maxCombo(&self) -> Result<u32, String> {
        if let Some(max_combo) = self.max_combo {
            Ok(max_combo)
        } else if let (Some(n_fruits), Some(n_droplets)) = (self.n_fruits, self.n_droplets) {
            Ok(n_fruits + n_droplets)
        } else {
            Err(INVALID_DIFF_ATTRS.to_owned())
        }
    }
}

impl From<OsuDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: OsuDifficultyAttributes) -> Self {
        let OsuDifficultyAttributes {
            aim,
            speed,
            flashlight,
            slider_factor,
            speed_note_count,
            ar,
            od,
            hp,
            n_circles,
            n_sliders,
            n_spinners,
            stars,
            max_combo,
        } = attrs;

        Self {
            mode: JsGameMode::Osu,
            stars,
            is_convert: false,
            aim: Some(aim),
            speed: Some(speed),
            flashlight: Some(flashlight),
            slider_factor: Some(slider_factor),
            speed_note_count: Some(speed_note_count),
            ar: Some(ar),
            od: Some(od),
            hp: Some(hp),
            n_circles: Some(n_circles),
            n_sliders: Some(n_sliders),
            n_spinners: Some(n_spinners),
            max_combo: Some(max_combo),
            ..Self::default()
        }
    }
}

impl From<TaikoDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: TaikoDifficultyAttributes) -> Self {
        let TaikoDifficultyAttributes {
            stamina,
            rhythm,
            color,
            peak,
            hit_window,
            stars,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: JsGameMode::Taiko,
            stars,
            is_convert,
            stamina: Some(stamina),
            rhythm: Some(rhythm),
            color: Some(color),
            peak: Some(peak),
            hit_window: Some(hit_window),
            max_combo: Some(max_combo),
            ..Self::default()
        }
    }
}

impl From<CatchDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: CatchDifficultyAttributes) -> Self {
        let CatchDifficultyAttributes {
            stars,
            ar,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            is_convert,
        } = attrs;

        Self {
            mode: JsGameMode::Catch,
            stars,
            is_convert,
            ar: Some(ar),
            n_fruits: Some(n_fruits),
            n_droplets: Some(n_droplets),
            n_tiny_droplets: Some(n_tiny_droplets),
            ..Self::default()
        }
    }
}

impl From<ManiaDifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: ManiaDifficultyAttributes) -> Self {
        let ManiaDifficultyAttributes {
            stars,
            hit_window,
            n_objects,
            max_combo,
            is_convert,
        } = attrs;

        Self {
            mode: JsGameMode::Mania,
            stars,
            is_convert,
            hit_window: Some(hit_window),
            n_objects: Some(n_objects),
            max_combo: Some(max_combo),
            ..Self::default()
        }
    }
}

impl From<DifficultyAttributes> for JsDifficultyAttributes {
    fn from(attrs: DifficultyAttributes) -> Self {
        match attrs {
            DifficultyAttributes::Osu(attrs) => attrs.into(),
            DifficultyAttributes::Taiko(attrs) => attrs.into(),
            DifficultyAttributes::Catch(attrs) => attrs.into(),
            DifficultyAttributes::Mania(attrs) => attrs.into(),
        }
    }
}

impl TryFrom<JsDifficultyAttributes> for DifficultyAttributes {
    type Error = String;

    fn try_from(attrs: JsDifficultyAttributes) -> Result<Self, Self::Error> {
        let JsDifficultyAttributes {
            mode,
            stars,
            is_convert,
            aim,
            speed,
            flashlight,
            slider_factor,
            speed_note_count,
            od,
            hp,
            n_circles,
            n_sliders,
            n_spinners,
            stamina,
            rhythm,
            color,
            peak,
            n_fruits,
            n_droplets,
            n_tiny_droplets,
            n_objects,
            ar,
            hit_window,
            max_combo,
        } = attrs;

        match mode {
            JsGameMode::Osu => {
                if let (
                    Some(aim),
                    Some(speed),
                    Some(flashlight),
                    Some(slider_factor),
                    Some(speed_note_count),
                    Some(ar),
                    Some(od),
                    Some(hp),
                    Some(n_circles),
                    Some(n_sliders),
                    Some(n_spinners),
                    Some(max_combo),
                ) = (
                    aim,
                    speed,
                    flashlight,
                    slider_factor,
                    speed_note_count,
                    ar,
                    od,
                    hp,
                    n_circles,
                    n_sliders,
                    n_spinners,
                    max_combo,
                ) {
                    return Ok(Self::Osu(OsuDifficultyAttributes {
                        aim,
                        speed,
                        flashlight,
                        slider_factor,
                        speed_note_count,
                        ar,
                        od,
                        hp,
                        n_circles,
                        n_sliders,
                        n_spinners,
                        stars,
                        max_combo,
                    }));
                }
            }
            JsGameMode::Taiko => {
                if let (
                    Some(stamina),
                    Some(rhythm),
                    Some(color),
                    Some(peak),
                    Some(hit_window),
                    Some(max_combo),
                ) = (stamina, rhythm, color, peak, hit_window, max_combo)
                {
                    return Ok(Self::Taiko(TaikoDifficultyAttributes {
                        stamina,
                        rhythm,
                        color,
                        peak,
                        hit_window,
                        stars,
                        max_combo,
                        is_convert,
                    }));
                }
            }
            JsGameMode::Catch => {
                if let (Some(ar), Some(n_fruits), Some(n_droplets), Some(n_tiny_droplets)) =
                    (ar, n_fruits, n_droplets, n_tiny_droplets)
                {
                    return Ok(Self::Catch(CatchDifficultyAttributes {
                        stars,
                        ar,
                        n_fruits,
                        n_droplets,
                        n_tiny_droplets,
                        is_convert,
                    }));
                }
            }
            JsGameMode::Mania => {
                if let (Some(hit_window), Some(n_objects), Some(max_combo)) =
                    (hit_window, n_objects, max_combo)
                {
                    return Ok(Self::Mania(ManiaDifficultyAttributes {
                        stars,
                        hit_window,
                        n_objects,
                        max_combo,
                        is_convert,
                    }));
                }
            }
        }

        Err(INVALID_DIFF_ATTRS.to_owned())
    }
}

/// While generating remaining hitresults, decide how they should be distributed.
#[wasm_bindgen(js_name = HitResultPriority)]
pub enum JsHitResultPriority {
    /// Prioritize good hitresults over bad ones
    BestCase,
    /// Prioritize bad hitresults over good ones
    WorstCase,
}

impl From<HitResultPriority> for JsHitResultPriority {
    fn from(priority: HitResultPriority) -> Self {
        match priority {
            HitResultPriority::BestCase => Self::BestCase,
            HitResultPriority::WorstCase => Self::WorstCase,
        }
    }
}

impl From<JsHitResultPriority> for HitResultPriority {
    fn from(priority: JsHitResultPriority) -> Self {
        match priority {
            JsHitResultPriority::BestCase => Self::BestCase,
            JsHitResultPriority::WorstCase => Self::WorstCase,
        }
    }
}

/// Builder for a performance calculation.
#[wasm_bindgen(js_name = Performance)]
pub struct JsPerformance {
    difficulty: JsDifficulty,
    acc: Option<f64>,
    combo: Option<u32>,
    n_geki: Option<u32>,
    n_katu: Option<u32>,
    n300: Option<u32>,
    n100: Option<u32>,
    n50: Option<u32>,
    misses: Option<u32>,
    hitresult_priority: HitResultPriority,
}

#[wasm_bindgen(js_class = Performance)]
impl JsPerformance {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new(difficulty: Option<JsDifficulty>) -> Self {
        Self {
            difficulty: difficulty.unwrap_or_else(JsDifficulty::new),
            acc: None,
            combo: None,
            n_geki: None,
            n_katu: None,
            n300: None,
            n100: None,
            n50: None,
            misses: None,
            hitresult_priority: HitResultPriority::default(),
        }
    }

    /// Use the specified settings of the given `Difficulty`.
    pub fn difficulty(mut self, difficulty: JsDifficulty) -> Self {
        self.difficulty = difficulty;

        self
    }

    /// Specify mods through their bit values.
    ///
    /// See <https://github.com/ppy/osu-api/wiki#mods>
    pub fn mods(mut self, mods: u32) -> Self {
        self.difficulty = self.difficulty.mods(mods);

        self
    }

    /// Amount of passed objects for partial plays, e.g. a fail.
    ///
    /// If you want to calculate the performance after every few objects,
    /// instead of using `Performance` multiple times with different
    /// `passedObjects`, you should use `GradualPerformance`.
    pub fn passedObjects(mut self, passedObjects: u32) -> Self {
        self.difficulty = self.difficulty.passedObjects(passedObjects);

        self
    }

    /// Adjust the clock rate used in the calculation.
    ///
    /// If none is specified, it will take the clock rate based on the mods
    /// i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | 0.01    | 100     |
    pub fn clockRate(mut self, clockRate: f64) -> Self {
        self.difficulty = self.difficulty.clockRate(clockRate);

        self
    }

    /// Override a beatmap's set AR.
    ///
    /// Only relevant for osu! and osu!catch.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn ar(mut self, ar: f32, withMods: bool) -> Self {
        self.difficulty = self.difficulty.ar(ar, withMods);

        self
    }

    /// Override a beatmap's set CS.
    ///
    /// Only relevant for osu! and osu!catch.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn cs(mut self, cs: f32, withMods: bool) -> Self {
        self.difficulty = self.difficulty.cs(cs, withMods);

        self
    }

    /// Override a beatmap's set HP.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn hp(mut self, hp: f32, withMods: bool) -> Self {
        self.difficulty = self.difficulty.hp(hp, withMods);

        self
    }

    /// Override a beatmap's set OD.
    ///
    /// `withMods` determines if the given value should be used before
    /// or after accounting for mods, e.g. on `true` the value will be
    /// used as is and on `false` it will be modified based on the mods.
    ///
    /// | Minimum | Maximum |
    /// | :-----: | :-----: |
    /// | -20     | 20      |
    pub fn od(mut self, od: f32, withMods: bool) -> Self {
        self.difficulty = self.difficulty.od(od, withMods);

        self
    }

    /// Adjust patterns as if the HR mod is enabled.
    ///
    /// Only relevant for osu!catch.
    pub fn hardrockOffsets(mut self, hardrockOffsets: bool) -> Self {
        self.difficulty = self.difficulty.hardrockOffsets(hardrockOffsets);

        self
    }

    /// Set the accuracy between `0.0` and `100.0`.
    pub fn accuracy(mut self, acc: f64) -> Self {
        self.acc = Some(acc);

        self
    }

    /// Specify the max combo of the play.
    ///
    /// Irrelevant for osu!mania.
    pub fn combo(mut self, combo: u32) -> Self {
        self.combo = Some(combo);

        self
    }

    /// Specify the amount of gekis of a play.
    ///
    /// Only relevant for osu!mania for which it repesents the amount of n320.
    pub fn nGeki(mut self, nGeki: u32) -> Self {
        self.n_geki = Some(nGeki);

        self
    }

    /// Specify the amount of katus of a play.
    ///
    /// Only relevant for osu!catch for which it represents the amount of tiny
    /// droplet misses and osu!mania for which it repesents the amount of n200.
    pub fn nKatu(mut self, nKatu: u32) -> Self {
        self.n_katu = Some(nKatu);

        self
    }

    /// Specify the amount of 300s of a play.
    pub fn n300(mut self, n300: u32) -> Self {
        self.n300 = Some(n300);

        self
    }

    /// Specify the amount of 100s of a play.
    pub fn n100(mut self, n100: u32) -> Self {
        self.n100 = Some(n100);

        self
    }

    /// Specify the amount of 50s of a play.
    ///
    /// Irrelevant for osu!taiko.
    pub fn n50(mut self, n50: u32) -> Self {
        self.n50 = Some(n50);

        self
    }

    /// Specify the amount of misses of a play.
    pub fn misses(mut self, misses: u32) -> Self {
        self.misses = Some(misses);

        self
    }

    /// Specify how hitresults should be generated.
    ///
    /// Defauls to `HitResultPriority.BestCase`.
    pub fn hitresultPriority(mut self, priority: JsHitResultPriority) -> Self {
        self.hitresult_priority = priority.into();

        self
    }

    /// Calculate performance attributes for a map.
    ///
    /// Note that using this method will perform the costly computation of
    /// difficulty attributes internally. If attributes for the current
    /// `Difficulty` settings are already available, consider using the method
    /// `calculate_with_attributes` instead.
    /// @throws Will throw an error if the specified mode is incompatible with the map's mode
    pub fn calculate_with_map(
        self,
        map: &mut JsBeatmap,
    ) -> Result<JsPerformanceAttributes, String> {
        if let Some(mode) = self.difficulty.mode {
            map.convert_native(mode)?;
        }

        Ok(self.calculate(Performance::from_map(&map.inner)))
    }

    /// Calculate performance attributes.
    ///
    /// Note that `attrs` must have been calculated for the same map and
    /// `Difficulty` settings, otherwise the final attributes will be incorrect.
    /// @throws Will throw an error if the specified mode is incompatible with the attributes or if the attributes have been modified manually
    pub fn calculate_with_attributes(
        self,
        attrs: JsDifficultyAttributes,
    ) -> Result<JsPerformanceAttributes, String> {
        if let Some(to) = self.difficulty.mode {
            let from = GameMode::from(attrs.mode);

            if from != to {
                return Err(cannot_convert(from, to));
            }
        }

        let attrs = DifficultyAttributes::try_from(attrs)?;

        Ok(self.calculate(Performance::from_attributes(attrs)))
    }
}

impl JsPerformance {
    fn calculate(self, performance: Performance<'_>) -> JsPerformanceAttributes {
        let mut performance = performance
            .difficulty(self.difficulty.inner)
            .hitresult_priority(self.hitresult_priority);

        if let Some(acc) = self.acc {
            performance = performance.accuracy(acc);
        }

        if let Some(combo) = self.combo {
            performance = performance.combo(combo);
        }

        if let Some(n_geki) = self.n_geki {
            performance = performance.n_geki(n_geki);
        }

        if let Some(n_katu) = self.n_katu {
            performance = performance.n_katu(n_katu);
        }

        if let Some(n300) = self.n300 {
            performance = performance.n300(n300);
        }

        if let Some(n100) = self.n100 {
            performance = performance.n100(n100);
        }

        if let Some(n50) = self.n50 {
            performance = performance.n50(n50);
        }

        if let Some(misses) = self.misses {
            performance = performance.misses(misses);
        }

        performance.calculate().into()
    }
}

#[wasm_bindgen(js_name = PerformanceAttributes, inspectable)]
#[derive(Debug, Default)]
pub struct JsPerformanceAttributes {
    /// The difficulty attributes that were used for the performance
    /// calculation.
    #[wasm_bindgen(readonly, getter_with_clone)]
    pub difficulty: JsDifficultyAttributes,

    /// The final performance points.
    #[wasm_bindgen(readonly)]
    pub pp: f64,

    /// The aim portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub pp_aim: Option<f64>,

    /// The flashlight portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub pp_flashlight: Option<f64>,

    /// The speed portion of the final pp.
    ///
    /// Only available for osu!.
    #[wasm_bindgen(readonly)]
    pub pp_speed: Option<f64>,

    /// The accuracy portion of the final pp.
    ///
    /// Only available for osu! and osu!taiko.
    #[wasm_bindgen(readonly)]
    pub pp_acc: Option<f64>,

    /// Scaled miss count based on total hits.
    ///
    /// Only available for osu! and osu!taiko.
    #[wasm_bindgen(readonly)]
    pub effective_miss_count: Option<f64>,

    /// The strain portion of the final pp.
    ///
    /// Only available for osu!taiko and osu!mania.
    #[wasm_bindgen(readonly)]
    pub pp_difficulty: Option<f64>,
}

impl From<PerformanceAttributes> for JsPerformanceAttributes {
    fn from(attrs: PerformanceAttributes) -> Self {
        match attrs {
            PerformanceAttributes::Osu(OsuPerformanceAttributes {
                difficulty,
                pp,
                pp_acc,
                pp_aim,
                pp_flashlight,
                pp_speed,
                effective_miss_count,
            }) => Self {
                difficulty: difficulty.into(),
                pp,
                pp_acc: Some(pp_acc),
                pp_aim: Some(pp_aim),
                pp_flashlight: Some(pp_flashlight),
                pp_speed: Some(pp_speed),
                effective_miss_count: Some(effective_miss_count),
                ..Self::default()
            },
            PerformanceAttributes::Taiko(TaikoPerformanceAttributes {
                difficulty,
                pp,
                pp_acc,
                pp_difficulty,
                effective_miss_count,
            }) => Self {
                difficulty: difficulty.into(),
                pp,
                pp_acc: Some(pp_acc),
                pp_difficulty: Some(pp_difficulty),
                effective_miss_count: Some(effective_miss_count),
                ..Self::default()
            },
            PerformanceAttributes::Catch(CatchPerformanceAttributes { difficulty, pp }) => Self {
                difficulty: difficulty.into(),
                pp,
                ..Self::default()
            },
            PerformanceAttributes::Mania(ManiaPerformanceAttributes {
                difficulty,
                pp,
                pp_difficulty,
            }) => Self {
                difficulty: difficulty.into(),
                pp,
                pp_difficulty: Some(pp_difficulty),
                ..Self::default()
            },
        }
    }
}
