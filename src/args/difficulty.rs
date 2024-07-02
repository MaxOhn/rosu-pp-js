use rosu_mods::GameMods;
use rosu_pp::Difficulty;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::util;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = DifficultyArgs)]
    pub type JsDifficultyArgs;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `Difficulty` constructor.
*/
export interface DifficultyArgs extends CommonArgs {
    /**
    * Amount of passed objects for partial plays, e.g. a fail.
    *
    * If you want to calculate the difficulty after every few objects,
    * instead of using `Difficulty` multiple times with different
    * `passedObjects`, you should use `GradualDifficulty`.
    */
    passedObjects?: number;
    /**
    * Adjust patterns as if the HR mod is enabled.
    *
    * Only relevant for osu!catch.
    */
    hardrockOffsets?: boolean;
}"#;

#[derive(Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase", rename = "Object")]
pub struct DifficultyArgs {
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
    pub passed_objects: Option<u32>,
    pub hardrock_offsets: Option<bool>,
}

impl DifficultyArgs {
    pub fn to_difficulty(&self) -> Difficulty {
        let mut difficulty = Difficulty::new();

        difficulty = match self.mods.checked_bits() {
            Some(bits) => difficulty.mods(bits),
            None => difficulty.mods(self.mods.clone()),
        };

        if let Some(passed_objects) = self.passed_objects {
            difficulty = difficulty.passed_objects(passed_objects);
        }

        if let Some(clock_rate) = self.clock_rate {
            difficulty = difficulty.clock_rate(clock_rate);
        }

        if let Some(ar) = self.ar {
            difficulty = difficulty.ar(ar, self.ar_with_mods);
        }

        if let Some(cs) = self.cs {
            difficulty = difficulty.cs(cs, self.cs_with_mods);
        }

        if let Some(hp) = self.hp {
            difficulty = difficulty.hp(hp, self.hp_with_mods);
        }

        if let Some(od) = self.od {
            difficulty = difficulty.od(od, self.od_with_mods);
        }

        if let Some(hardrock_offsets) = self.hardrock_offsets {
            difficulty = difficulty.hardrock_offsets(hardrock_offsets);
        }

        difficulty
    }
}
