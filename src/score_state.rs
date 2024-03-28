use std::fmt;

use rosu_pp::any::ScoreState;
use serde::de;
use wasm_bindgen::prelude::*;

use crate::util;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = ScoreState)]
    #[derive(Clone)]
    pub type JsScoreState;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `Difficulty` constructor.
*/
export interface ScoreState {
    /**
    * Maximum combo that the score has had so far. **Not** the maximum
    * possible combo of the map so far.
    *
    * Note that for osu!catch only fruits and droplets are considered for
    * combo.
    *
    * Irrelevant for osu!mania.
    */
    maxCombo?: number;
    /**
    * Amount of current gekis (n320 for osu!mania).
    */
    nGeki?: number;
    /**
    * Amount of current katus (tiny droplet misses for osu!catch / n200 for
    * osu!mania).
    */
    nKatu?: number;
    /**
    * Amount of current 300s (fruits for osu!catch).
    */
    n300?: number;
    /**
    * Amount of current 100s (droplets for osu!catch).
    */
    n100?: number;
    /**
    * Amount of current 50s (tiny droplets for osu!catch).
    */
    n50?: number;
    /**
    * Amount of current misses (fruits + droplets for osu!catch).
    */
    misses?: number;
}"#;

impl JsScoreState {
    pub fn deserialize<'de, D: de::Deserializer<'de>>(d: D) -> Result<ScoreState, D::Error> {
        enum ScoreStateField {
            MaxCombo,
            NGeki,
            NKatu,
            N300,
            N100,
            N50,
            Misses,
        }

        struct ScoreStateFieldVisitor;

        impl<'de> de::Visitor<'de> for ScoreStateFieldVisitor {
            type Value = ScoreStateField;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("field identifier")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                match v {
                    "maxCombo" => Ok(ScoreStateField::MaxCombo),
                    "nGeki" => Ok(ScoreStateField::NGeki),
                    "nKatu" => Ok(ScoreStateField::NKatu),
                    "n300" => Ok(ScoreStateField::N300),
                    "n100" => Ok(ScoreStateField::N100),
                    "n50" => Ok(ScoreStateField::N50),
                    "misses" => Ok(ScoreStateField::Misses),
                    // The deserializer only forwards specified fields
                    _ => unreachable!(),
                }
            }
        }

        impl<'de> de::Deserialize<'de> for ScoreStateField {
            fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                d.deserialize_identifier(ScoreStateFieldVisitor)
            }
        }

        struct ScoreStateVisitor;

        impl<'de> de::Visitor<'de> for ScoreStateVisitor {
            type Value = ScoreState;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a ScoreState")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut state = ScoreState::default();

                while let Some(key) = map.next_key()? {
                    match key {
                        ScoreStateField::MaxCombo => state.max_combo = map.next_value()?,
                        ScoreStateField::NGeki => state.n_geki = map.next_value()?,
                        ScoreStateField::NKatu => state.n_katu = map.next_value()?,
                        ScoreStateField::N300 => state.n300 = map.next_value()?,
                        ScoreStateField::N100 => state.n100 = map.next_value()?,
                        ScoreStateField::N50 => state.n50 = map.next_value()?,
                        ScoreStateField::Misses => state.misses = map.next_value()?,
                    }
                }

                Ok(state)
            }
        }

        const FIELDS: &[&str] = &[
            "maxCombo", "nGeki", "nKatu", "n300", "n100", "n50", "misses",
        ];

        d.deserialize_struct("Object", FIELDS, ScoreStateVisitor)
    }
}

impl From<ScoreState> for JsScoreState {
    fn from(state: ScoreState) -> Self {
        let map = js_sys::Map::new();
        map.set(&util::static_str_to_js("maxCombo"), &state.max_combo.into());
        map.set(&util::static_str_to_js("nGeki"), &state.n_geki.into());
        map.set(&util::static_str_to_js("nKatu"), &state.n_katu.into());
        map.set(&util::static_str_to_js("n300"), &state.n300.into());
        map.set(&util::static_str_to_js("n100"), &state.n100.into());
        map.set(&util::static_str_to_js("n50"), &state.n50.into());
        map.set(&util::static_str_to_js("misses"), &state.misses.into());

        JsValue::from(map).into()
    }
}
