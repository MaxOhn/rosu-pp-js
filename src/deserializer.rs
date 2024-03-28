use std::borrow::Cow;

use js_sys::{Number, Reflect, Uint8Array};
use serde::de;
use wasm_bindgen::prelude::*;

use crate::{util, JsError, JsResult};

/// Largely references `serde_wasm_bindgen`'s deserializer.
pub struct JsDeserializer<'js> {
    value: Cow<'js, JsValue>,
}

impl<'js> JsDeserializer<'js> {
    pub const fn from_ref(value: &'js JsValue) -> Self {
        Self {
            value: Cow::Borrowed(value),
        }
    }

    pub const fn from_owned(value: JsValue) -> Self {
        Self {
            value: Cow::Owned(value),
        }
    }

    fn is_nullish(&self) -> bool {
        self.value.loose_eq(&JsValue::NULL)
    }

    fn as_bytes(&self) -> Option<Vec<u8>> {
        self.value.dyn_ref().map(Uint8Array::to_vec)
    }

    #[cold]
    fn invalid_type_(&self, visitor: &dyn de::Expected) -> JsError {
        let string;
        let bytes;

        let unexpected = if self.is_nullish() {
            de::Unexpected::Unit
        } else if let Some(v) = self.value.as_bool() {
            de::Unexpected::Bool(v)
        } else if let Some(v) = self.value.as_f64() {
            de::Unexpected::Float(v)
        } else if let Some(v) = self.value.as_string() {
            string = v;
            de::Unexpected::Str(&string)
        } else if let Some(v) = self.as_bytes() {
            bytes = v;
            de::Unexpected::Bytes(&bytes)
        } else {
            string = format!("{:?}", self.value);
            de::Unexpected::Other(&string)
        };

        de::Error::invalid_type(unexpected, visitor)
    }

    fn invalid_type<'de, V: de::Visitor<'de>>(&self, visitor: V) -> JsResult<V::Value> {
        Err(self.invalid_type_(&visitor))
    }

    fn as_safe_integer(&self) -> Option<i64> {
        if Number::is_safe_integer(&self.value) {
            return Some(self.value.unchecked_into_f64() as i64);
        }

        None
    }

    fn deserialize_from_js_number_unsigned<'de, V: de::Visitor<'de>>(
        &self,
        visitor: V,
    ) -> JsResult<V::Value> {
        match self.as_safe_integer() {
            Some(v) if v >= 0 => visitor.visit_u64(v as _),
            _ => self.invalid_type(visitor),
        }
    }
}

impl<'de, 'js> de::Deserializer<'de> for JsDeserializer<'js> {
    type Error = JsError;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let Some(v) = self.value.as_string() {
            visitor.visit_string(v)
        } else if let Some(v) = self.as_bytes() {
            visitor.visit_byte_buf(v)
        } else if let Some(v) = self.value.as_bool() {
            visitor.visit_bool(v)
        } else if let Some(v) = self.value.as_f64() {
            if Number::is_safe_integer(&self.value) {
                visitor.visit_i64(v as i64)
            } else {
                visitor.visit_f64(v)
            }
        } else {
            self.invalid_type(visitor)
        }
    }

    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let Some(v) = self.value.as_bool() {
            visitor.visit_bool(v)
        } else {
            self.invalid_type(visitor)
        }
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_from_js_number_unsigned(visitor)
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_from_js_number_unsigned(visitor)
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let Some(v) = self.value.as_f64() {
            visitor.visit_f64(v)
        } else {
            self.invalid_type(visitor)
        }
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let Some(v) = self.value.as_string() {
            visitor.visit_string(v)
        } else {
            self.invalid_type(visitor)
        }
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if let Some(bytes) = self.as_bytes() {
            visitor.visit_byte_buf(bytes)
        } else {
            self.invalid_type(visitor)
        }
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if !self.is_nullish() {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(
        self,
        _len: usize,
        _: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        _: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        struct_name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        let obj = if self.value.is_object() {
            self.value.unchecked_ref::<util::ObjectExt>()
        } else {
            return self.invalid_type(visitor);
        };

        let constructor = Reflect::get(&self.value, &util::static_str_to_js("constructor").into())?;

        let correct_classname = Reflect::get(&constructor, &util::static_str_to_js("name").into())?
            .as_string()
            .is_some_and(|name| name == struct_name);

        if !correct_classname {
            return Err(JsError::new(format!("Expected {struct_name}")));
        }

        visitor.visit_map(ObjectAccess::new(obj, fields))
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _: V,
    ) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }

    fn deserialize_identifier<V: de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, _: V) -> Result<V::Value, Self::Error> {
        unimplemented!()
    }
}

struct ObjectAccess<'js> {
    obj: &'js util::ObjectExt,
    fields: std::slice::Iter<'static, &'static str>,
    next_value: Option<JsDeserializer<'js>>,
}

impl<'js> ObjectAccess<'js> {
    fn new(obj: &'js util::ObjectExt, fields: &'static [&'static str]) -> Self {
        Self {
            obj,
            fields: fields.iter(),
            next_value: None,
        }
    }
}

fn str_deserializer(s: &str) -> de::value::StrDeserializer<JsError> {
    de::IntoDeserializer::into_deserializer(s)
}

impl<'de> de::MapAccess<'de> for ObjectAccess<'_> {
    type Error = JsError;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> JsResult<Option<K::Value>> {
        debug_assert!(self.next_value.is_none());

        for field in self.fields.by_ref() {
            let js_field = util::static_str_to_js(field);
            let next_value = self.obj.get_with_ref_key(&js_field);
            let is_missing_field = next_value.is_undefined() && !js_field.js_in(self.obj);

            if !is_missing_field {
                self.next_value = Some(JsDeserializer::from_owned(next_value));

                return Ok(Some(seed.deserialize(str_deserializer(field))?));
            }
        }

        Ok(None)
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> JsResult<V::Value> {
        seed.deserialize(self.next_value.take().unwrap_throw())
    }
}
