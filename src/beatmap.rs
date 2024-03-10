use std::{error, fmt::Write, io};

use rosu_pp::{
    model::mode::{ConvertStatus, GameMode},
    Beatmap,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{cannot_convert, mode::JsGameMode};

/// All beatmap data that is relevant for difficulty and performance
/// calculation.
#[wasm_bindgen(js_name = Beatmap)]
pub struct JsBeatmap {
    pub(crate) inner: Beatmap,
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
        let inner = res.map_err(|e| {
            let mut e = &e as &dyn error::Error;
            let mut content = format!("Failed to decode beatmap: {e}");

            while let Some(src) = e.source() {
                let _ = writeln!(content, "  - caused by: {src}");
                e = src;
            }

            content
        })?;

        Ok(Self { inner })
    }

    pub(crate) fn convert_native(&mut self, mode: GameMode) -> Result<(), String> {
        if let ConvertStatus::Incompatible = self.inner.convert_in_place(mode) {
            return Err(cannot_convert(self.inner.mode, mode));
        }

        Ok(())
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
