pub mod difficulty;
pub mod performance;

fn maybe_convert_serialize<J, O, A>(attrs: Option<A>) -> crate::JsResult<Option<O>>
where
    J: From<A> + serde::Serialize,
    O: From<wasm_bindgen::JsValue>,
{
    match attrs {
        Some(attrs) => crate::util::to_value(&J::from(attrs))
            .map(From::from)
            .map(Some),
        None => Ok(None),
    }
}
