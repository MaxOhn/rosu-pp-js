use std::{error, fmt::Write, ops::DerefMut};

use js_sys::Reflect;
use rosu_pp::{
    model::{
        hit_object::HitObjectKind,
        mode::{ConvertStatus, GameMode},
    },
    Beatmap,
};
use wasm_bindgen::{convert::RefMutFromWasmAbi, prelude::wasm_bindgen, JsValue};

use crate::{
    args::beatmap::{BeatmapArgs, JsBeatmapArgs},
    mode::JsGameMode,
    util, JsError, JsResult,
};

/// All beatmap data that is relevant for difficulty and performance
/// calculation.
#[wasm_bindgen(js_name = Beatmap)]
pub struct JsBeatmap {
    pub(crate) inner: Beatmap,
}

#[wasm_bindgen(js_class = Beatmap)]
impl JsBeatmap {
    /// Create a new beatmap instance by parsing an `.osu` file's content.
    /// @throws Throws an error if `bytes` or `content` are not specified, decoding the map failed, or the specified mode is incompatible with the map's mode
    #[wasm_bindgen(constructor)]
    pub fn new(args: &JsBeatmapArgs) -> JsResult<JsBeatmap> {
        #[cfg(feature = "panic_hook")]
        console_error_panic_hook::set_once();

        let args = BeatmapArgs::from_value(args)?;

        let map_res = if let Some(bytes) = args.bytes {
            Beatmap::from_bytes(&bytes)
        } else if let Some(content) = args.content {
            content.parse()
        } else {
            return Err(JsError::new("`bytes` or `content` must be specified"));
        };

        let map = match map_res {
            Ok(map) => map,
            Err(err) => {
                let mut e = &err as &dyn error::Error;
                let mut content = format!("Failed to decode beatmap: {e}");

                while let Some(src) = e.source() {
                    let _ = writeln!(content, "  - caused by: {src}");
                    e = src;
                }

                return Err(JsError::new(content));
            }
        };

        let mut this = Self { inner: map };

        if let Some(mode) = args.mode {
            this.convert(mode)?;
        }

        Ok(this)
    }

    /// Convert a beatmap to a specific mode.
    /// @throws Throws an error if the specified mode is incompatible with the map's mode
    pub fn convert(&mut self, mode: JsGameMode) -> JsResult<()> {
        let mode = GameMode::from(mode);

        if let ConvertStatus::Incompatible = self.inner.convert_in_place(mode) {
            return Err(JsError::new(format!(
                "Cannot convert {:?} to {mode:?}",
                self.inner.mode
            )));
        }

        Ok(())
    }

    #[wasm_bindgen(getter)]
    pub fn bpm(&self) -> f64 {
        self.inner.bpm()
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> JsGameMode {
        JsGameMode::from(self.inner.mode)
    }

    #[wasm_bindgen(js_name = nBreaks, getter)]
    pub fn n_breaks(&self) -> usize {
        self.inner.breaks.len()
    }

    #[wasm_bindgen(js_name = nObjects, getter)]
    pub fn n_objects(&self) -> usize {
        self.inner.hit_objects.len()
    }

    #[wasm_bindgen(js_name = nCircles, getter)]
    pub fn n_circles(&self) -> usize {
        self.inner
            .hit_objects
            .iter()
            .filter(|h| h.is_circle())
            .count()
    }

    #[wasm_bindgen(js_name = nSliders, getter)]
    pub fn n_sliders(&self) -> usize {
        self.inner
            .hit_objects
            .iter()
            .filter(|h| h.is_slider())
            .count()
    }

    #[wasm_bindgen(js_name = nSpinners, getter)]
    pub fn n_spinners(&self) -> usize {
        self.inner
            .hit_objects
            .iter()
            .filter(|h| h.is_spinner())
            .count()
    }

    #[wasm_bindgen(js_name = nHolds, getter)]
    pub fn n_holds(&self) -> usize {
        self.inner
            .hit_objects
            .iter()
            .filter(|h| matches!(h.kind, HitObjectKind::Hold(_)))
            .count()
    }
}

impl JsBeatmap {
    pub(crate) fn try_from_value(js: &JsValue) -> JsResult<impl DerefMut<Target = Self>> {
        const EXPECTED_BEATMAP: &str = "Expected Beatmap instance";

        if !js.is_object() {
            return Err(JsError::new(EXPECTED_BEATMAP));
        }

        let constructor = Reflect::get(js, &util::static_str_to_js("constructor").into())?;

        let correct_classname = Reflect::get(&constructor, &util::static_str_to_js("name").into())?
            .as_string()
            .is_some_and(|name| name == "Beatmap");

        if !correct_classname {
            return Err(JsError::new(EXPECTED_BEATMAP));
        }

        let ptr = Reflect::get(js, &JsValue::from_str("__wbg_ptr"))?;
        let ptr_u32 = ptr.as_f64().ok_or(JsValue::NULL)? as u32;
        let instance_ref = unsafe { JsBeatmap::ref_mut_from_abi(ptr_u32) };

        Ok(instance_ref)
    }
}

beatmap_getters! {
    version as version: i32,
    is_convert as isConvert: bool,
    stack_leniency as stackLeniency: f32,
    ar as ar: f32,
    cs as cs: f32,
    hp as hp: f32,
    od as od: f32,
    slider_multiplier as sliderMultiplier: f64,
    slider_tick_rate as sliderTickRate: f64,
}
