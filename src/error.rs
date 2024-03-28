use std::{error, fmt};

use serde::de;
use wasm_bindgen::prelude::*;

use crate::util;

pub type JsResult<T> = Result<T, JsError>;

#[derive(Debug)]
pub struct JsError(JsValue);

impl JsError {
    pub fn new(err: String) -> Self {
        Self(err.into())
    }
}

impl From<&'static str> for JsError {
    fn from(err: &'static str) -> Self {
        Self(util::static_str_to_js(err).into())
    }
}

impl From<JsValue> for JsError {
    fn from(value: JsValue) -> Self {
        Self(value)
    }
}

impl From<JsError> for JsValue {
    fn from(JsError(value): JsError) -> Self {
        value
    }
}

impl fmt::Display for JsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = String)]
            pub fn to_string(value: &JsValue) -> String;
        }

        to_string(&self.0).fmt(f)
    }
}

impl error::Error for JsError {}

impl de::Error for JsError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        JsError::new(msg.to_string())
    }
}
