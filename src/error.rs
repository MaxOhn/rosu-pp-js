use std::{error, fmt};

use serde::de;
use wasm_bindgen::prelude::*;

pub type JsResult<T> = Result<T, JsError>;

#[derive(Debug)]
pub struct JsError(js_sys::Error);

impl JsError {
    pub fn new(msg: &str) -> Self {
        Self(js_sys::Error::new(msg))
    }
}

impl From<JsValue> for JsError {
    fn from(value: JsValue) -> Self {
        Self(value.into())
    }
}

impl From<JsError> for JsValue {
    fn from(JsError(err): JsError) -> Self {
        err.into()
    }
}

impl fmt::Display for JsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.to_string().fmt(f)
    }
}

impl error::Error for JsError {}

impl de::Error for JsError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        JsError::new(&msg.to_string())
    }
}
