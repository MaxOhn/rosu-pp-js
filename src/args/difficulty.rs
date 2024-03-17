use rosu_pp::Difficulty;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{util, JsResult};

use super::common::CommonArgs;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = DifficultyArgs)]
    pub type JsDifficultyArgs;
}

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"/**
* Arguments to provide the `Difficulty` constructor.
*/
interface DifficultyArgs extends CommonArgs {
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

#[derive(Debug, Default)]
pub struct DifficultyArgs {
    pub common: CommonArgs,
    pub passed_objects: Option<u32>,
    pub hardrock_offsets: Option<bool>,
}

impl DifficultyArgs {
    pub fn from_value(value: &JsDifficultyArgs) -> JsResult<Self> {
        let mut this = util::from_value::<Self>(value)?;
        this.common = util::from_value::<CommonArgs>(value)?;

        Ok(this)
    }

    pub fn as_difficulty(&self) -> Difficulty {
        let mut difficulty = Difficulty::new().mods(self.common.mods);

        if let Some(passed_objects) = self.passed_objects {
            difficulty = difficulty.passed_objects(passed_objects);
        }

        if let Some(clock_rate) = self.common.clock_rate {
            difficulty = difficulty.clock_rate(clock_rate);
        }

        if let Some(ar) = self.common.ar {
            difficulty = difficulty.ar(ar, self.common.ar_with_mods);
        }

        if let Some(cs) = self.common.cs {
            difficulty = difficulty.cs(cs, self.common.cs_with_mods);
        }

        if let Some(hp) = self.common.hp {
            difficulty = difficulty.hp(hp, self.common.hp_with_mods);
        }

        if let Some(od) = self.common.od {
            difficulty = difficulty.od(od, self.common.od_with_mods);
        }

        if let Some(hardrock_offsets) = self.hardrock_offsets {
            difficulty = difficulty.hardrock_offsets(hardrock_offsets);
        }

        difficulty
    }
}

from_jsvalue! {
    DifficultyArgs {
        passed_objects as passedObjects: u32?,
        hardrock_offsets as hardrockOffsets: bool?,
    }
}
