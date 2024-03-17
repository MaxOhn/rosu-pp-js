macro_rules! beatmap_getters {
    ( $( $field:ident as $getter:ident: $ty:ty, )+ ) => {
        #[wasm_bindgen::prelude::wasm_bindgen(js_class = Beatmap)]
        impl JsBeatmap {
            $(
                #[wasm_bindgen::prelude::wasm_bindgen(js_name = $getter, getter)]
                pub fn $field(&self) -> $ty {
                    self.inner.$field
                }
            )*
        }
    };
}

macro_rules! from_jsvalue {
    (
        $ty:ident {
            $( $field:ident as $js_field:ident: $field_ty:tt $bang_or_question:tt $(,)? )*
        }
    ) => {
        impl crate::util::FromJsValue for $ty {
            const FIELDS: &'static [&'static str] = &[
                $( stringify!($js_field), )*
            ];

            fn field(&mut self, name: &str, value: wasm_bindgen::JsValue) -> crate::JsResult<()> {
                match name {
                    $(
                        stringify!($js_field) => {
                            from_jsvalue!( @BRANCH $field as $js_field: $field_ty $bang_or_question [value, self] )
                        },
                    )*
                    _ => unreachable!(),
                }

                Ok(())
            }
        }
    };

    ( @BRANCH $field:ident as $js_field:ident: $ty:tt ! [ $value:ident, $self:ident ] ) => {
        from_jsvalue!( @INNER $field as $js_field: $ty [ $value, $self ] )
    };

    ( @BRANCH $field:ident as $js_field:ident: $ty:tt ? [ $value:ident, $self:ident ] ) => {
        from_jsvalue!( @INNER $field as $js_field: Option<$ty> [ $value, $self ] )
    };

    ( @BRANCH $field:ident as $js_field:ident: $ty:tt $other:tt [ $value:ident, $self:ident ] ) => {
        compile_error!("put either ! or ? after the type")
    };

    ( @INNER $field:ident as $js_field:ident: f64 [ $value:ident, $self:ident] ) => {
        match $value.as_f64() {
            Some($field) => $self.$field = $field,
            None => from_jsvalue!( @ERROR $js_field ),
        }
    };

    ( @INNER $field:ident as $js_field:ident: Option<f64> [ $value:ident, $self:ident] ) => {
        match $value.as_f64() {
            Some($field) => $self.$field = Some($field),
            None => from_jsvalue!( @ERROR $js_field ),
        }
    };

    ( @INNER $field:ident as $js_field:ident: f32 [ $value:ident, $self:ident] ) => {
        match $value.as_f64() {
            Some($field) => $self.$field = $field as f32,
            None => from_jsvalue!( @ERROR $js_field ),
        }
    };

    ( @INNER $field:ident as $js_field:ident: Option<f32> [ $value:ident, $self:ident] ) => {
        match $value.as_f64() {
            Some($field) => $self.$field = Some($field as f32),
            None => from_jsvalue!( @ERROR $js_field ),
        }
    };

    ( @INNER $field:ident as $js_field:ident: bool [ $value:ident, $self:ident] ) => {
        match $value.as_bool() {
            Some($field) => $self.$field = $field,
            None => from_jsvalue!( @ERROR $js_field ),
        }
    };

    ( @INNER $field:ident as $js_field:ident: Option<bool> [ $value:ident, $self:ident] ) => {
        match $value.as_bool() {
            Some($field) => $self.$field = Some($field),
            None => from_jsvalue!( @ERROR $js_field ),
        }
    };

    ( @INNER $field:ident as $js_field:ident: $other:tt [ $value:ident, $self:ident] ) => {
        match crate::util::JsValueExt::as_safe_integer(&$value).map(TryFrom::try_from) {
            Some(Ok($field)) => $self.$field = $field,
            _ => from_jsvalue!( @ERROR $js_field ),
        }
    };

    ( @INNER $field:ident as $js_field:ident: Option<$other:tt> [ $value:ident, $self:ident] ) => {
        match crate::util::JsValueExt::as_safe_integer(&$value).map(TryFrom::try_from) {
            Some(Ok($field)) => $self.$field = Some($field),
            _ => from_jsvalue!( @ERROR $js_field ),
        }
    };

    ( @ERROR $js_field:ident ) => {
        return Err(crate::JsError::new(concat!("invalid ", stringify!($js_field))))
    }
}
