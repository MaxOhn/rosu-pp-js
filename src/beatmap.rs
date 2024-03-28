use std::{
    error,
    fmt::{Formatter, Result as FmtResult, Write},
};

use rosu_pp::{
    model::{
        hit_object::HitObjectKind,
        mode::{ConvertStatus, GameMode},
    },
    Beatmap,
};
use serde::de;
use wasm_bindgen::{__rt::RefMut, convert::RefMutFromWasmAbi, prelude::wasm_bindgen};

use crate::{
    args::beatmap::{BeatmapContent, JsBeatmapContent},
    mode::JsGameMode,
    util::{self, FieldVisitor},
    JsError, JsResult,
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
    /// @throws Throws an error if decoding the map failed
    #[wasm_bindgen(constructor)]
    pub fn new(args: &JsBeatmapContent) -> JsResult<JsBeatmap> {
        #[cfg(feature = "panic_hook")]
        console_error_panic_hook::set_once();

        let content = util::from_value::<BeatmapContent>(args)?;

        match Beatmap::from_bytes(&content.bytes) {
            Ok(inner) => Ok(Self { inner }),
            Err(err) => {
                let mut e = &err as &dyn error::Error;
                let mut content = format!("Failed to decode beatmap: {e}");

                while let Some(src) = e.source() {
                    let _ = writeln!(content, "  - caused by: {src}");
                    e = src;
                }

                Err(JsError::new(content))
            }
        }
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
    pub fn deserialize<'de, D: de::Deserializer<'de>>(
        d: D,
    ) -> Result<RefMut<'static, Self>, D::Error> {
        struct BeatmapField;

        impl BeatmapField {
            const NAME: &'static str = "__wbg_ptr";
        }

        impl<'de> de::Deserialize<'de> for BeatmapField {
            fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                d.deserialize_str(FieldVisitor::new(Self::NAME))
                    .map(|_| Self)
            }
        }

        struct BeatmapVisitor;

        impl<'de> de::Visitor<'de> for BeatmapVisitor {
            type Value = RefMut<'static, JsBeatmap>;

            fn expecting(&self, f: &mut Formatter) -> FmtResult {
                f.write_str("a Beatmap")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                map.next_key::<BeatmapField>()?;

                let ptr_u32 = map.next_value::<u32>()?;
                let instance_ref = unsafe { JsBeatmap::ref_mut_from_abi(ptr_u32) };

                Ok(instance_ref)
            }
        }

        d.deserialize_struct("Beatmap", &[BeatmapField::NAME], BeatmapVisitor)
    }
}

macro_rules! beatmap_getters {
    ( $( $field:ident as $getter:ident: $ty:ty, )+ ) => {
        #[wasm_bindgen(js_class = Beatmap)]
        impl JsBeatmap {
            $(
                #[wasm_bindgen(js_name = $getter, getter)]
                pub fn $field(&self) -> $ty {
                    self.inner.$field
                }
            )*
        }
    };
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
