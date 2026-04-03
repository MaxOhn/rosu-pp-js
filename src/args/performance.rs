use rosu_mods::GameMods;
use rosu_pp::{
    Performance,
    any::{
        DifficultyAttributes, HitResultPriority,
        hitresult_generator::{Closest, Composable, Fast},
    },
};
use serde::de;
use wasm_bindgen::{__rt::RcRef, JsValue, prelude::wasm_bindgen};

use crate::{
    JsError, JsResult,
    attributes::{difficulty::JsDifficultyAttributes, performance::JsPerformanceAttributes},
    beatmap::JsBeatmap,
    deserializer::JsDeserializer,
    util,
};

use super::difficulty::DifficultyArgs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = PerformanceArgs)]
    pub type JsPerformanceArgs;

    #[wasm_bindgen(typescript_type = "MapOrAttributes")]
    pub type JsMapOrAttributes;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `Performance` constructor.
*/
export interface PerformanceArgs extends DifficultyArgs {
    /** Set the accuracy between `0.0` and `100.0`. */
    accuracy?: number | null;
    /**
    * Specify the max combo of the play.
    *
    * Irrelevant for osu!mania.
    */
    combo?: number | null;
    /**
    * The amount of "large tick" hits.
    *
    * Only relevant for osu!.
    *
    * The meaning depends on the kind of score:
    * - if set on osu!stable, this value is irrelevant and can be `0`
    * - if set on osu!lazer *without* `CL`, this value is the amount of hit
    *   slider ticks and repeats
    * - if set on osu!lazer *with* `CL`, this value is the amount of hit
    *   slider heads, ticks, and repeats
    */
    largeTickHits?: number | null;
    /**
    * The amount of "small tick" hits.
    *
    * These are essentially the slider end hits for lazer scores without
    * slider accuracy.
    *
    * Only relevant for osu!.
    */
    smallTickHits?: number | null;
    /**
    * The amount of slider end hits.
    *
    * Only relevant for osu! in lazer.
    */
    sliderEndHits?: number | null;
    /**
    * Specify the amount of gekis of a play.
    *
    * Only relevant for osu!mania for which it repesents the amount of n320.
    */
    nGeki?: number | null;
    /**
    * Specify the amount of katus of a play.
    *
    * Only relevant for osu!catch for which it represents the amount of tiny
    * droplet misses and osu!mania for which it repesents the amount of n200.
    */
    nKatu?: number | null;
    /** Specify the amount of 300s of a play. */
    n300?: number | null;
    /** Specify the amount of 100s of a play. */
    n100?: number | null;
    /**
    * Specify the amount of 50s of a play.
    *
    * Irrelevant for osu!taiko.
    */
    n50?: number | null;
    /** Specify the amount of misses of a play. */
    misses?: number | null;
    /**
    * Specify the legacy total score.
    *
    * Only relevant for osu!.
    */
    legacyTotalScore?: number | null;
    /**
    * Specify how hitresults should be generated.
    *
    * Defaults to `HitResultPriority.BestCase`.
    */
    hitresultPriority?: HitResultPriority;
    /** Four optional generators; one for each mode. */
    hitresultGenerators?: Array<(HitResultGenerator | null)> | null;
}"#;

#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase", rename = "Object")]
pub struct PerformanceArgs {
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
    pub passed_objects: Option<u32>,
    pub hardrock_offsets: Option<bool>,
    pub lazer: Option<bool>,
    pub accuracy: Option<f64>,
    pub combo: Option<u32>,
    pub large_tick_hits: Option<u32>,
    pub small_tick_hits: Option<u32>,
    pub slider_end_hits: Option<u32>,
    pub n_geki: Option<u32>,
    pub n_katu: Option<u32>,
    pub n300: Option<u32>,
    pub n100: Option<u32>,
    pub n50: Option<u32>,
    pub misses: Option<u32>,
    pub legacy_total_score: Option<u32>,
    #[serde(default, deserialize_with = "JsHitResultPriority::deserialize")]
    pub hitresult_priority: HitResultPriority,
    #[serde(default, deserialize_with = "JsHitResultGenerator::deserialize")]
    pub hitresult_generators: [Option<JsHitResultGenerator>; 4],
}

/// While generating remaining hitresults, decide how they should be distributed.
#[wasm_bindgen(js_name = HitResultPriority)]
#[derive(Copy, Clone)]
pub enum JsHitResultPriority {
    /// Prioritize good hitresults over bad ones
    BestCase,
    /// Prioritize bad hitresults over good ones
    WorstCase,
}

impl From<JsHitResultPriority> for HitResultPriority {
    fn from(priority: JsHitResultPriority) -> Self {
        match priority {
            JsHitResultPriority::BestCase => Self::BestCase,
            JsHitResultPriority::WorstCase => Self::WorstCase,
        }
    }
}

impl JsHitResultPriority {
    fn deserialize<'de, D: de::Deserializer<'de>>(d: D) -> Result<HitResultPriority, D::Error> {
        let priority = match <u8 as de::Deserialize>::deserialize(d) {
            Ok(0) => HitResultPriority::BestCase,
            Ok(1) => HitResultPriority::WorstCase,
            _ => return Err(de::Error::custom("invalid HitResultPriority")),
        };

        Ok(priority)
    }
}

/// A specific implementation of hitresult generation.
#[wasm_bindgen(js_name = HitResultGenerator)]
#[derive(Copy, Clone)]
pub enum JsHitResultGenerator {
    /// Prioritize generating hitresults quickly.
    Fast,
    /// Find the hitresults that match the given accuracy the closest.
    Closest,
}

impl JsHitResultGenerator {
    fn deserialize<'de, D: de::Deserializer<'de>>(d: D) -> Result<[Option<Self>; 4], D::Error> {
        <Option<[Option<u8>; 4]> as de::Deserialize>::deserialize(d)
            .ok()
            .unwrap_or_default()
            .and_then(|generators| {
                let mut all_valid = true;

                let generators = generators.map(|opt| match opt {
                    Some(0) => Some(Self::Fast),
                    Some(1) => Some(Self::Closest),
                    _ => {
                        all_valid = false;

                        None
                    }
                });

                all_valid.then_some(generators)
            })
            .ok_or_else(|| {
                de::Error::custom(
                    "invalid hitresult generators, expected list of four optional generators",
                )
            })
    }
}

impl PerformanceArgs {
    pub fn apply<'a>(&self, mut perf: Performance<'a>) -> Performance<'a> {
        let Self {
            mods,
            clock_rate,
            ar,
            fixed_ar,
            cs,
            fixed_cs,
            hp,
            fixed_hp,
            od,
            fixed_od,
            passed_objects,
            hardrock_offsets,
            lazer,
            accuracy,
            combo,
            large_tick_hits,
            small_tick_hits,
            slider_end_hits,
            n_geki,
            n_katu,
            n300,
            n100,
            n50,
            misses,
            legacy_total_score,
            hitresult_priority,
            hitresult_generators,
        } = self;

        if let Some(accuracy) = accuracy {
            perf = perf.accuracy(*accuracy);
        }

        if let Some(combo) = combo {
            perf = perf.combo(*combo);
        }

        if let Some(large_tick_hits) = large_tick_hits {
            perf = perf.large_tick_hits(*large_tick_hits);
        }

        if let Some(small_tick_hits) = small_tick_hits {
            perf = perf.small_tick_hits(*small_tick_hits);
        }

        if let Some(slider_end_hits) = slider_end_hits {
            perf = perf.slider_end_hits(*slider_end_hits);
        }

        if let Some(n_geki) = n_geki {
            perf = perf.n_geki(*n_geki);
        }

        if let Some(n_katu) = n_katu {
            perf = perf.n_katu(*n_katu);
        }

        if let Some(n300) = n300 {
            perf = perf.n300(*n300);
        }

        if let Some(n100) = n100 {
            perf = perf.n100(*n100);
        }

        if let Some(n50) = n50 {
            perf = perf.n50(*n50);
        }

        if let Some(misses) = misses {
            perf = perf.misses(*misses);
        }

        if let Some(legacy_total_score) = legacy_total_score {
            perf = perf.legacy_total_score(*legacy_total_score);
        }

        let difficulty = DifficultyArgs {
            mods: mods.to_owned(),
            clock_rate: *clock_rate,
            ar: *ar,
            fixed_ar: *fixed_ar,
            cs: *cs,
            fixed_cs: *fixed_cs,
            hp: *hp,
            fixed_hp: *fixed_hp,
            od: *od,
            fixed_od: *fixed_od,
            passed_objects: *passed_objects,
            hardrock_offsets: *hardrock_offsets,
            lazer: *lazer,
        };

        // Bridging runtime values to compile-time types
        macro_rules! apply_hitresult_generator {
            // Entry: pass all 4 indices as a "remaining" list
            () => {
                apply_hitresult_generator!(@step [0, 1, 2, 3] [])
            };

            // Still have indices to process
            ( @step [ $i:tt $(, $rest:tt )* ] [ $( $acc:ty ),* ] ) => {
                match hitresult_generators[$i] {
                    None | Some(JsHitResultGenerator::Fast) => {
                        apply_hitresult_generator!(
                            @step [$($rest),*] [$($acc,)* Fast]
                        )
                    }
                    Some(JsHitResultGenerator::Closest) => {
                        apply_hitresult_generator!(
                            @step [$($rest),*] [$($acc,)* Closest]
                        )
                    }
                }
            };

            // No indices left: emit the call
            ( @step [] [$osu:ty, $taiko:ty, $catch:ty, $mania:ty] ) => {
                perf.hitresult_generator::<Composable<$osu, $taiko, $catch, $mania>>()
            };
        }

        perf = apply_hitresult_generator!();

        perf.hitresult_priority(*hitresult_priority)
            .difficulty(difficulty.to_difficulty())
    }
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Either previously calculated attributes or a beatmap.
*/
export type MapOrAttributes = DifficultyAttributes | PerformanceAttributes | Beatmap;"#;

pub enum MapOrAttrs {
    Map(RcRef<JsBeatmap>),
    Attrs(DifficultyAttributes),
}

impl MapOrAttrs {
    pub fn from_value(value: &JsValue) -> JsResult<Self> {
        if let Ok(js_attrs) =
            JsPerformanceAttributes::deserialize_difficulty(JsDeserializer::from_ref(value))
        {
            return js_attrs.try_into().map(Self::Attrs);
        }

        if let Ok(js_attrs) = util::from_value::<JsDifficultyAttributes>(value) {
            return js_attrs.try_into().map(Self::Attrs);
        }

        if let Ok(map) = JsBeatmap::deserialize(JsDeserializer::from_ref(value)) {
            return Ok(Self::Map(map));
        }

        Err(JsError::new(
            "Expected either previously calculated attributes or a beatmap",
        ))
    }
}
