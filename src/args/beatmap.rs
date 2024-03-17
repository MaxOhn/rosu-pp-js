use js_sys::Uint8Array;
use wasm_bindgen::{
    prelude::{wasm_bindgen, JsValue},
    JsCast,
};

use crate::{
    mode::JsGameMode,
    util::{self, FromJsValue, JsValueExt, ObjectExt},
    JsResult,
};

use super::common::CommonArgs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = BeatmapArgs)]
    pub type JsBeatmapArgs;

    #[wasm_bindgen(typescript_type = BeatmapAttributesArgs)]
    pub type JsBeatmapAttributesArgs;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `Beatmap` constructor.
*/
interface BeatmapArgs {
    /**
    * The bytes of a `.osu` file's content.
    */
    bytes?: Uint8Array,
    /**
    * The content of a `.osu` file.
    */
    content?: string,
    /**
    * The mode to convert the beatmap to.
    */
    mode?: GameMode,
}"#;

#[derive(Default)]
pub struct BeatmapArgs {
    pub bytes: Option<Vec<u8>>,
    pub content: Option<String>,
    pub mode: Option<JsGameMode>,
}

impl BeatmapArgs {
    pub fn from_value(value: &JsBeatmapArgs) -> JsResult<Self> {
        util::from_value(value)
    }
}

impl FromJsValue for BeatmapArgs {
    const FIELDS: &'static [&'static str] = &["bytes", "content", "mode"];

    fn field(&mut self, name: &str, value: JsValue) -> JsResult<()> {
        match name {
            "bytes" => self.bytes = Some(Uint8Array::new(&value).to_vec()),
            "content" => match value.as_string() {
                Some(content) => self.content = Some(content),
                None => return Err(crate::JsError::new("invalid content")),
            },
            "mode" => match value.as_safe_integer().map(TryFrom::try_from) {
                Some(Ok(mode)) => self.mode = Some(mode),
                _ => return Err(crate::JsError::new("invalid mode")),
            },
            _ => unreachable!(),
        }

        Ok(())
    }
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `BeatmapAttributesBuilder` constructor.
*/
interface BeatmapAttributesArgs extends CommonArgs {
    /**
    * Specify a gamemode.
    */
    mode?: GameMode;
    /**
    * Specify whether it's a converted map.
    */
    isConvert?: boolean,
    /**
    * Start off with a beatmap's attributes.
    */
    map?: Beatmap,
}"#;

#[derive(Default)]
pub struct BeatmapAttributesArgs {
    pub common: CommonArgs,
    pub mode: Option<JsGameMode>,
    pub is_convert: bool,
    pub map: Option<JsValue>,
}

impl BeatmapAttributesArgs {
    pub fn from_value(value: &JsBeatmapAttributesArgs) -> JsResult<Self> {
        let mut this = util::from_value::<Self>(value)?;
        this.common = util::from_value(value)?;

        let obj = value.unchecked_ref::<ObjectExt>();
        let js_field = util::static_str_to_js("map");
        let js_value = obj.get_with_ref_key(&js_field);

        if !js_value.is_undefined() {
            this.map = Some(js_value);
        }

        Ok(this)
    }
}

from_jsvalue! {
    BeatmapAttributesArgs {
        mode as mode: JsGameMode?,
        is_convert as isConvert: bool!,
    }
}
