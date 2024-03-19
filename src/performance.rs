use rosu_pp::Performance;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    args::performance::{JsMapOrAttributes, JsPerformanceArgs, MapOrAttrs, PerformanceArgs},
    attributes::performance::{JsPerformanceAttributes, PerformanceAttributes},
    beatmap::JsBeatmap,
    util, JsError, JsResult,
};

/// Builder for a performance calculation.
#[wasm_bindgen(js_name = Performance)]
pub struct JsPerformance {
    args: PerformanceArgs,
}

#[wasm_bindgen(js_class = Performance)]
impl JsPerformance {
    /// Create a new performance calculator.
    #[wasm_bindgen(constructor)]
    pub fn new(args: Option<JsPerformanceArgs>) -> JsResult<JsPerformance> {
        #[cfg(feature = "panic_hook")]
        console_error_panic_hook::set_once();

        let args = args
            .as_ref()
            .map(PerformanceArgs::from_value)
            .transpose()?
            .unwrap_or_default();

        Ok(Self { args })
    }

    /// Calculate performance attributes.
    ///
    /// If a beatmap is passed as argument, difficulty attributes will have to
    /// be calculated internally which is a comparably expensive task. Hence,
    /// passing previously calculated attributes should be prefered whenever
    /// available.
    ///
    /// However, be careful that the passed attributes have been calculated
    /// for the same difficulty settings like mods, clock rate, beatmap,
    /// custom ar, ... otherwise the final attributes will be incorrect.
    pub fn calculate(&mut self, args: &JsMapOrAttributes) -> JsResult<JsPerformanceAttributes> {
        let map_or_attrs = MapOrAttrs::from_value(args);
        let map;

        let mut perf = match map_or_attrs {
            MapOrAttrs::Map(value) => {
                map = JsBeatmap::try_from_value(value).map_err(|_| {
                    JsError::new(
                        "argument must be either previously calculated difficulty attributes or \
                        a beatmap",
                    )
                })?;

                Performance::from_map(&map.inner)
            }
            MapOrAttrs::Attrs(attrs) => Performance::from_attributes(attrs),
        };

        perf = self.args.apply(perf);
        let state = perf.generate_state();
        let attrs = PerformanceAttributes::new(perf.calculate(), state);

        util::to_value(&attrs).map(From::from)
    }
}
