use std::{
    error,
    fmt::{Formatter, Result as FmtResult, Write},
};

use rosu_pp::{
    Beatmap,
    model::{hit_object::HitObjectKind, mode::GameMode},
};
use serde::de;
use wasm_bindgen::{__rt::RcRef, convert::RefFromWasmAbi, prelude::wasm_bindgen};

use crate::{
    JsError, JsResult,
    args::beatmap::{BeatmapContent, JsBeatmapContent},
    deserializer::JsDeserializer,
    mode::JsGameMode,
    mods::JsGameMods,
    util::{self, FieldVisitor},
};

/// All beatmap data that is relevant for difficulty and performance
/// calculation.
///
/// It is recommended to call the method `Beatmap.free` on instances that are
/// no longer in use to avoid the risk of leaking memory.
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

                Err(JsError::new(&content))
            }
        }
    }

    /// Convert a beatmap to a specific mode.
    /// @throws Throws an error if conversion fails or mods are invalid
    pub fn convert(&mut self, mode: JsGameMode, mods: Option<JsGameMods>) -> JsResult<()> {
        let mods = mods
            .as_deref()
            .map(JsDeserializer::from_ref)
            .map(util::deserialize_mods)
            .transpose()?
            .unwrap_or_default();

        let mode = GameMode::from(mode);

        if let Err(err) = self.inner.convert_mut(mode, &mods.into()) {
            return Err(JsError::new(&err.to_string()));
        }

        Ok(())
    }

    /// Check whether hitobjects appear too suspicious for further calculation.
    ///
    /// Sometimes a beatmap isn't created for gameplay but rather to test
    /// the limits of osu! itself. Difficulty- and/or performance calculation
    /// should likely be avoided on these maps due to potential performance
    /// issues.
    #[wasm_bindgen(js_name = isSuspicious)]
    pub fn is_suspicious(&self) -> bool {
        self.inner.check_suspicion().is_err()
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
    pub fn deserialize<'de, D: de::Deserializer<'de>>(d: D) -> Result<RcRef<Self>, D::Error> {
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
            type Value = RcRef<JsBeatmap>;

            fn expecting(&self, f: &mut Formatter) -> FmtResult {
                f.write_str("a Beatmap")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                if map.next_key::<BeatmapField>()?.is_none() {
                    return Err(de::Error::custom("expected a Beatmap"));
                }

                let ptr_u32 = map.next_value::<u32>()?;
                let instance_ref = unsafe { JsBeatmap::ref_from_abi(ptr_u32) };

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
