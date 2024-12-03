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
    * "Large tick" hits for osu!standard.
    *
    * The meaning depends on the kind of score:
    * - if set on osu!stable, this field is irrelevant and can be `0`
    * - if set on osu!lazer *without* `CL`, this field is the amount of hit
    *   slider ticks and repeats
    * - if set on osu!lazer *with* `CL`, this field is the amount of hit
    *   slider heads, ticks, and repeats
    */
    osuLargeTickHits?: number;

    /**
    * "Small tick" hits for osu!standard.
    *
    * These are essentially the slider end hits for lazer scores without
    * slider accuracy.
    *
    * Only relevant for osu!lazer.
    */ 
    osuSmallTickHits?: number;

    /**
    * Amount of successfully hit slider ends.
    *
    * Only relevant for osu!standard in lazer.
    */
    sliderEndHits?: number;
    
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
        macro_rules! impl_deserialize {
            ( $( $js_field:ident: $rs_field:ident, )* ) => {{
                #[allow(non_camel_case_types)]
                enum ScoreStateField {
                    $( $js_field, )*
                }

                struct ScoreStateFieldVisitor;

                impl<'de> de::Visitor<'de> for ScoreStateFieldVisitor {
                    type Value = ScoreStateField;

                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                        match v {
                            $( stringify!($js_field) => Ok(ScoreStateField::$js_field), )*
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
                                $( ScoreStateField::$js_field => state.$rs_field = map.next_value()?, )*
                            }
                        }

                        Ok(state)
                    }
                }

                const FIELDS: &[&str] = &[ $( stringify!($js_field), )* ];

                d.deserialize_struct("Object", FIELDS, ScoreStateVisitor)
            }};
        }

        impl_deserialize! {
            maxCombo: max_combo,
            osuLargeTickHits: osu_large_tick_hits,
            osuSmallTickHits: osu_small_tick_hits,
            sliderEndHits: slider_end_hits,
            nGeki: n_geki,
            nKatu: n_katu,
            n300: n300,
            n100: n100,
            n50: n50,
            misses: misses,
        }
    }
}

impl From<ScoreState> for JsScoreState {
    fn from(state: ScoreState) -> Self {
        let obj = js_sys::Object::new();
        let obj_as_ext = obj.unchecked_ref::<util::ObjectExt>();

        let set = |key, value: u32| obj_as_ext.set(util::static_str_to_js(key), value.into());

        set("maxCombo", state.max_combo);
        set("osuLargeTickHits", state.osu_large_tick_hits);
        set("osuSmallTickHits", state.osu_small_tick_hits);
        set("sliderEndHits", state.slider_end_hits);
        set("nGeki", state.n_geki);
        set("nKatu", state.n_katu);
        set("n300", state.n300);
        set("n100", state.n100);
        set("n50", state.n50);
        set("misses", state.misses);

        JsValue::from(obj).into()
    }
}
