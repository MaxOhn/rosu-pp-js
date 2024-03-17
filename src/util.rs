use std::{
    cell::RefCell,
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
    slice::Iter,
};

use js_sys::{JsString, Number};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::{
    prelude::{wasm_bindgen, JsValue},
    JsCast,
};

use crate::{JsError, JsResult};

pub trait JsValueExt {
    fn as_safe_integer(&self) -> Option<i64>;
}

impl JsValueExt for JsValue {
    fn as_safe_integer(&self) -> Option<i64> {
        if Number::is_safe_integer(self) {
            return Some(self.unchecked_into_f64() as i64);
        }

        None
    }
}

pub trait FromJsValue: Default {
    const FIELDS: &'static [&'static str];

    fn field(&mut self, name: &str, value: JsValue) -> JsResult<()>;
}

pub fn from_value<T: FromJsValue>(value: &JsValue) -> JsResult<T> {
    struct FieldIter<'a> {
        obj: &'a ObjectExt,
        inner: Iter<'static, &'static str>,
    }

    impl Iterator for FieldIter<'_> {
        type Item = (&'static str, JsValue);

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let field = self.inner.next()?;
                let js_field = static_str_to_js(field);
                let js_value = self.obj.get_with_ref_key(&js_field);

                if !js_value.is_undefined() {
                    return Some((field, js_value));
                }
            }
        }
    }

    if !value.is_object() {
        return Err(JsError::new("argument must be an object"));
    }

    let mut output = T::default();

    let iter = FieldIter {
        obj: value.unchecked_ref::<ObjectExt>(),
        inner: T::FIELDS.iter(),
    };

    for (field, js_value) in iter {
        output.field(field, js_value)?;
    }

    Ok(output)
}

pub fn to_value<T: Serialize>(value: &T) -> JsResult<JsValue> {
    value.serialize(&Serializer::new())
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
