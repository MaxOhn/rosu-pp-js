use rosu_pp::Performance;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::performance::{JsPerformanceArgs, MapOrAttrs, PerformanceArgs},
    attributes::performance::{JsPerformanceAttributes, PerformanceAttributes},
    beatmap::JsBeatmap,
    util, JsError, JsResult,
};

/// Builder for a performance calculation.
#[wasm_bindgen(js_name = Performance)]
pub struct JsPerformance {
    inner: Performance<'static>,
}

#[wasm_bindgen(js_class = Performance)]
impl JsPerformance {
    /// Create a new performance calculator.
    #[wasm_bindgen(constructor)]
    pub fn new(args: &JsPerformanceArgs) -> JsResult<JsPerformance> {
        #[cfg(feature = "panic_hook")]
        console_error_panic_hook::set_once();

        let inner = PerformanceArgs::from_value(args).and_then(Performance::try_from)?;

        Ok(Self { inner })
    }

    /// Calculate performance attributes.
    #[wasm_bindgen(js_name = calculate)]
    pub fn calculate(&mut self) -> JsResult<JsPerformanceAttributes> {
        let state = self.inner.generate_state();
        let perf = self.inner.clone().calculate();
        let attrs = PerformanceAttributes::new(perf, state);

        util::to_value(&attrs).map(From::from)
    }
}

impl TryFrom<PerformanceArgs> for Performance<'static> {
    type Error = JsError;

    fn try_from(args: PerformanceArgs) -> Result<Self, Self::Error> {
        let difficulty = args.difficulty.as_difficulty();

        let attrs = match args.map_or_attrs {
            MapOrAttrs::Map(value) => {
                let map = JsBeatmap::try_from_value(&value)?;

                difficulty.calculate(&map.inner)
            }
            MapOrAttrs::Attrs(attrs) => attrs,
            MapOrAttrs::Neither => {
                return Err(JsError::new("`map` or `attributes` must be specified"))
            }
        };

        let mut performance = Performance::from_attributes(attrs).difficulty(difficulty);

        if let Some(accuracy) = args.accuracy {
            performance = performance.accuracy(accuracy);
        }

        if let Some(combo) = args.combo {
            performance = performance.combo(combo);
        }

        if let Some(n_geki) = args.n_geki {
            performance = performance.n_geki(n_geki);
        }

        if let Some(n_katu) = args.n_katu {
            performance = performance.n_katu(n_katu);
        }

        if let Some(n300) = args.n300 {
            performance = performance.n300(n300);
        }

        if let Some(n100) = args.n100 {
            performance = performance.n100(n100);
        }

        if let Some(n50) = args.n50 {
            performance = performance.n50(n50);
        }

        if let Some(misses) = args.misses {
            performance = performance.misses(misses);
        }

        Ok(performance.hitresult_priority(args.hitresult_priority))
    }
}
