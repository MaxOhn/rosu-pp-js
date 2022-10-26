use std::cell::RefCell;

use neon::{
    prelude::{Context, FunctionContext, Object},
    result::JsResult,
    types::{
        buffer::TypedArray, Finalize, JsBox, JsNumber, JsObject, JsString, JsTypedArray,
        JsUndefined,
    },
};
use rosu_pp::Beatmap;

use crate::error::ErrorExt;

pub struct Map {
    pub(crate) inner: RefCell<Option<Beatmap>>,
}

macro_rules! set_attr {
    ($fun:ident, $field:ident) => {
        pub fn $fun(mut cx: FunctionContext<'_>) -> JsResult<'_, JsUndefined> {
            let arg_opt = cx
                .argument_opt(0)
                .map(|arg| arg.downcast::<JsNumber, _>(&mut cx).ok());

            let val = if let Some(arg) = arg_opt.flatten() {
                arg.value(&mut cx)
            } else {
                return cx.throw_error("The first argument must be a number");
            };

            let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
            let mut map = this.inner.borrow_mut();

            if let Some(map) = map.as_mut() {
                map.$field = val as f32;

                Ok(JsUndefined::new(&mut cx))
            } else {
                cx.throw_error("Beatmap must be parsed first")
            }
        }
    };
}

impl Map {
    pub fn js_new(mut cx: FunctionContext<'_>) -> JsResult<'_, JsBox<Self>> {
        let arg = match cx
            .argument_opt(0)
            .filter(|arg| !arg.is_a::<JsUndefined, _>(&mut cx))
        {
            Some(arg) => arg,
            None => {
                let inner = RefCell::new(None);

                return Ok(cx.boxed(Self { inner }));
            }
        };

        let obj = match arg.downcast::<JsObject, _>(&mut cx) {
            Ok(obj) => obj,
            Err(_) => return cx.throw_error("The optional first argument must be an object"),
        };

        let path_val = obj.get_value(&mut cx, "path")?;

        if let Ok(path) = path_val.downcast::<JsString, _>(&mut cx) {
            let map = Beatmap::from_path(path.value(&mut cx))
                .or_else(|e| cx.throw_error(e.unwind("Failed to parse beatmap")))?;

            return Self::js_new_with_attrs(cx, &obj, map);
        } else if !path_val.is_a::<JsUndefined, _>(&mut cx) {
            return cx.throw_error("The `path` property must be a string");
        }

        let content_val = obj.get_value(&mut cx, "content")?;

        if let Ok(content) = content_val.downcast::<JsString, _>(&mut cx) {
            let map = Beatmap::from_bytes(content.value(&mut cx).as_bytes())
                .or_else(|e| cx.throw_error(e.unwind("Failed to parse beatmap")))?;

            return Self::js_new_with_attrs(cx, &obj, map);
        } else if let Ok(bytes) = content_val.downcast::<JsTypedArray<u8>, _>(&mut cx) {
            let map = Beatmap::from_bytes(bytes.as_slice(&cx))
                .or_else(|e| cx.throw_error(e.unwind("Failed to parse beatmap")))?;

            return Self::js_new_with_attrs(cx, &obj, map);
        } else if !content_val.is_a::<JsUndefined, _>(&mut cx) {
            return cx.throw_error("The `content` property must be a string or a Uint8Array");
        }

        let bytes_val = obj.get_value(&mut cx, "bytes")?;

        if let Ok(bytes) = bytes_val.downcast::<JsTypedArray<u8>, _>(&mut cx) {
            let map = Beatmap::from_bytes(bytes.as_slice(&cx))
                .or_else(|e| cx.throw_error(e.unwind("Failed to parse beatmap")))?;

            return Self::js_new_with_attrs(cx, &obj, map);
        } else if !bytes_val.is_a::<JsUndefined, _>(&mut cx) {
            return cx.throw_error("The `bytes` property must be a UInt8Array");
        }

        let err = "The optional first argument must be an object with a \
           `path`, `content`, or `bytes` property";

        cx.throw_error(err)
    }

    fn js_new_with_attrs<'c>(
        mut cx: FunctionContext<'c>,
        obj: &JsObject,
        mut map: Beatmap,
    ) -> JsResult<'c, JsBox<Self>> {
        macro_rules! parse_attr {
            ( $( $name:ident ),* ) => {
                $(
                    let val = obj.get_value(&mut cx, stringify!($name))?;

                    if let Ok(num) = val.downcast::<JsNumber, _>(&mut cx) {
                        map.$name = num.value(&mut cx) as f32;
                    } else if !val.is_a::<JsUndefined, _>(&mut cx) {
                        return cx.throw_error(concat!(
                            "The `",
                            stringify!($name),
                            "` property must be a number"
                        ));
                    }
                )*
            };
        }

        parse_attr!(ar, cs, hp, od);

        let inner = RefCell::new(Some(map));

        Ok(cx.boxed(Self { inner }))
    }

    pub fn js_from_path(mut cx: FunctionContext<'_>) -> JsResult<'_, JsUndefined> {
        let arg = match cx.argument_opt(0) {
            Some(arg) => arg,
            None => return cx.throw_error("The first argument must be a path to a .osu file"),
        };

        let path = match arg.downcast::<JsString, _>(&mut cx) {
            Ok(path) => path.value(&mut cx),
            Err(_) => return cx.throw_error("The first argument must be a string"),
        };

        let map = Beatmap::from_path(path)
            .or_else(|e| cx.throw_error(e.unwind("Failed to parse beatmap")))?;

        let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
        let _ = this.inner.borrow_mut().insert(map);

        Ok(JsUndefined::new(&mut cx))
    }

    pub fn js_from_content(mut cx: FunctionContext<'_>) -> JsResult<'_, JsUndefined> {
        let arg = match cx.argument_opt(0) {
            Some(arg) => arg,
            None => {
                return cx.throw_error("The first argument must be the content of an .osu file")
            }
        };

        let map_res = match arg.downcast::<JsString, _>(&mut cx) {
            Ok(arg) => Beatmap::from_bytes(arg.value(&mut cx).as_bytes()),
            Err(_) => match arg.downcast::<JsTypedArray<u8>, _>(&mut cx) {
                Ok(arg) => Beatmap::from_bytes(arg.as_slice(&cx)),
                Err(_) => {
                    return cx.throw_error("The first argument must be a string or Uint8Array")
                }
            },
        };

        let map = map_res.or_else(|e| cx.throw_error(e.unwind("Failed to parse beatmap")))?;
        let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
        let _ = this.inner.borrow_mut().insert(map);

        Ok(JsUndefined::new(&mut cx))
    }

    pub fn js_from_bytes(mut cx: FunctionContext<'_>) -> JsResult<'_, JsUndefined> {
        let arg = match cx.argument_opt(0) {
            Some(arg) => arg,
            None => return cx.throw_error("The first argument must be the bytes of an .osu file"),
        };

        let map_res = match arg.downcast::<JsTypedArray<u8>, _>(&mut cx) {
            Ok(arg) => Beatmap::from_bytes(arg.as_slice(&cx)),
            Err(_) => return cx.throw_error("The first argument must be a Uint8Array"),
        };

        let map = map_res.or_else(|e| cx.throw_error(e.unwind("Failed to parse beatmap")))?;
        let this = cx.this().downcast_or_throw::<JsBox<Self>, _>(&mut cx)?;
        let _ = this.inner.borrow_mut().insert(map);

        Ok(JsUndefined::new(&mut cx))
    }

    set_attr!(js_ar, ar);
    set_attr!(js_cs, cs);
    set_attr!(js_hp, hp);
    set_attr!(js_od, od);
}

impl Finalize for Map {}
