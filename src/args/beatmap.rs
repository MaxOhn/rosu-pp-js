use std::fmt::{Formatter, Result as FmtResult};

use rosu_mods::GameMods;
use rosu_pp::model::beatmap::{BeatmapAttributes, BeatmapAttributesBuilder};
use serde::de;
use wasm_bindgen::{__rt::RcRef, prelude::wasm_bindgen};

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

        impl de::Visitor<'_> for BeatmapContentVisitor {
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
    /** Specify a gamemode. */
    mode?: GameMode | null;
    /** Specify whether it's a converted map. */
    isConvert?: boolean;
    /** Start off with a beatmap's attributes, mode, and convert status. */
    map?: Beatmap | null;
}"#;

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase", rename = "Object")]
pub struct BeatmapAttributesArgs {
    #[serde(default, deserialize_with = "util::deserialize_mods")]
    pub mods: GameMods,
    pub clock_rate: Option<f64>,
    pub ar: Option<f32>,
    #[serde(default)]
    pub fixed_ar: bool,
    pub cs: Option<f32>,
    #[serde(default)]
    pub fixed_cs: bool,
    pub hp: Option<f32>,
    #[serde(default)]
    pub fixed_hp: bool,
    pub od: Option<f32>,
    #[serde(default)]
    pub fixed_od: bool,
    pub mode: Option<JsGameMode>,
    #[serde(default)]
    pub is_convert: bool,
    #[serde(default, deserialize_with = "deser_maybe_map")]
    pub map: Option<RcRef<JsBeatmap>>,
}

impl BeatmapAttributesArgs {
    pub fn into_builder(self) -> BeatmapAttributesBuilder {
        let mut builder = BeatmapAttributes::builder();
        builder.mods(self.mods.clone());

        if let Some(ref map) = self.map {
            builder.map(&map.inner);
        }

        if let Some(mode) = self.mode {
            builder.mode(mode.into(), self.is_convert);
        }

        if let Some(clock_rate) = self.clock_rate {
            builder.clock_rate(clock_rate);
        }

        if let Some(ar) = self.ar {
            builder.ar(ar, self.fixed_ar);
        }

        if let Some(cs) = self.cs {
            builder.cs(cs, self.fixed_cs);
        }

        if let Some(hp) = self.hp {
            builder.hp(hp, self.fixed_hp);
        }

        if let Some(od) = self.od {
            builder.od(od, self.fixed_od);
        }

        builder
    }
}

fn deser_maybe_map<'de, D: de::Deserializer<'de>>(
    d: D,
) -> Result<Option<RcRef<JsBeatmap>>, D::Error> {
    struct MaybeMapVisitor;

    impl<'de> de::Visitor<'de> for MaybeMapVisitor {
        type Value = Option<RcRef<JsBeatmap>>;

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
