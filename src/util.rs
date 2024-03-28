use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Formatter, Result as FmtResult},
    hash::{BuildHasherDefault, Hasher},
};

use js_sys::JsString;
use serde::de;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::JsResult;

pub fn from_value<'de, T: serde::Deserialize<'de>>(value: &'de JsValue) -> JsResult<T> {
    T::deserialize(crate::deserializer::JsDeserializer::from_ref(value))
}

#[wasm_bindgen]
extern "C" {
    pub type ObjectExt;

    #[wasm_bindgen(method, indexing_getter)]
    pub fn get_with_ref_key(this: &ObjectExt, key: &JsString) -> JsValue;

    #[wasm_bindgen(method, indexing_setter)]
    fn set(this: &ObjectExt, key: JsString, value: JsValue);
}

/// Store converted strings and return clones instead of converting them
/// every time.
///
/// <https://github.com/RReverser/serde-wasm-bindgen/blob/f073bd40ee5a354e3e9e2ac376300368d28067fe/src/lib.rs#L18-L58>
pub fn static_str_to_js(s: &'static str) -> JsString {
    #[derive(Default)]
    struct PtrHasher {
        addr: usize,
    }

    impl Hasher for PtrHasher {
        fn write(&mut self, _: &[u8]) {
            unreachable!();
        }

        fn write_usize(&mut self, addr_or_len: usize) {
            if self.addr == 0 {
                self.addr = addr_or_len;
            }
        }

        fn finish(&self) -> u64 {
            self.addr as _
        }
    }

    type PtrBuildHasher = BuildHasherDefault<PtrHasher>;

    thread_local! {
        static CACHE: RefCell<HashMap<*const str, JsString, PtrBuildHasher>> = Default::default();
    }

    CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry(s)
            .or_insert_with(|| s.into())
            .clone()
    })
}

pub struct FieldVisitor {
    name: &'static str,
}

impl FieldVisitor {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl<'de> de::Visitor<'de> for FieldVisitor {
    type Value = ();

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.name)
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        if v == self.name {
            return Ok(());
        }

        Err(de::Error::invalid_value(de::Unexpected::Str(v), &self))
    }
}
