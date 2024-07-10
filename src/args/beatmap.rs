use std::fmt::{Formatter, Result as FmtResult};

use rosu_mods::GameMods;
use rosu_pp::model::beatmap::BeatmapAttributesBuilder;
use serde::de;
use wasm_bindgen::{__rt::RefMut, prelude::wasm_bindgen};

use crate::{beatmap::JsBeatmap, mode::JsGameMode, util};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = BeatmapContent)]
    pub type JsBeatmapContent;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &str = r#"/**
* The content of a `.osu` file either as bytes or string.
*/
export type BeatmapContent = Uint8Array | string;"#;

pub struct BeatmapContent {
    pub bytes: Vec<u8>,
}

impl<'de> de::Deserialize<'de> for BeatmapContent {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct BeatmapContentVisitor;

        impl<'de> de::Visitor<'de> for BeatmapContentVisitor {
            type Value = BeatmapContent;

            fn expecting(&self, f: &mut Formatter) -> FmtResult {
                f.write_str("a Uint8Array or a string")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                self.visit_string(v.to_owned())
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(BeatmapContent {
                    bytes: v.into_bytes(),
                })
            }

            fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(BeatmapContent { bytes: v })
            }
        }

        d.deserialize_any(BeatmapContentVisitor)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = BeatmapAttributesArgs)]
    pub type JsBeatmapAttributesArgs;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `BeatmapAttributesBuilder` constructor.
*/
export interface BeatmapAttributesArgs extends CommonArgs {
    /**
    * Specify a gamemode.
    */
    mode?: GameMode;
    /**
    * Specify whether it's a converted map.
    */
    isConvert?: boolean;
    /**
    * Start off with a beatmap's attributes, mode, and convert status.
    */
    map?: Beatmap;
}"#;

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase", rename = "Object")]
pub struct BeatmapAttributesArgs {
    #[serde(default, deserialize_with = "util::deserialize_mods")]
    pub mods: GameMods,
    pub clock_rate: Option<f64>,
    pub ar: Option<f32>,
    #[serde(default)]
    pub ar_with_mods: bool,
    pub cs: Option<f32>,
    #[serde(default)]
    pub cs_with_mods: bool,
    pub hp: Option<f32>,
    #[serde(default)]
    pub hp_with_mods: bool,
    pub od: Option<f32>,
    #[serde(default)]
    pub od_with_mods: bool,
    pub mode: Option<JsGameMode>,
    #[serde(default)]
    pub is_convert: bool,
    #[serde(default, deserialize_with = "deser_maybe_map")]
    pub map: Option<RefMut<'static, JsBeatmap>>,
}

impl BeatmapAttributesArgs {
    pub fn as_builder(&self) -> BeatmapAttributesBuilder {
        let mut builder = BeatmapAttributesBuilder::new();

        if let Some(ref map) = self.map {
            builder = builder.map(&map.inner);
        }

        if let Some(mode) = self.mode {
            builder = builder.mode(mode.into(), self.is_convert);
        }

        if let Some(clock_rate) = self.clock_rate {
            builder = builder.clock_rate(clock_rate);
        }

        if let Some(ar) = self.ar {
            builder = builder.ar(ar, self.ar_with_mods);
        }

        if let Some(cs) = self.cs {
            builder = builder.cs(cs, self.cs_with_mods);
        }

        if let Some(hp) = self.hp {
            builder = builder.hp(hp, self.hp_with_mods);
        }

        if let Some(od) = self.od {
            builder = builder.od(od, self.od_with_mods);
        }

        match self.mods.checked_bits() {
            Some(bits) => builder.mods(bits),
            None => builder.mods(self.mods.clone()),
        }
    }
}

fn deser_maybe_map<'de, D: de::Deserializer<'de>>(
    d: D,
) -> Result<Option<RefMut<'static, JsBeatmap>>, D::Error> {
    struct MaybeMapVisitor;

    impl<'de> de::Visitor<'de> for MaybeMapVisitor {
        type Value = Option<RefMut<'static, JsBeatmap>>;

        fn expecting(&self, f: &mut Formatter) -> FmtResult {
            f.write_str("an optional Beatmap")
        }

        fn visit_some<D: de::Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            JsBeatmap::deserialize(d).map(Some)
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    d.deserialize_option(MaybeMapVisitor)
}
