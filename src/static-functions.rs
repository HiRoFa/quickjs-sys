unsafe extern "C" {
    fn JS_ValueGetTag_real(v: JSValue) -> i32;
    #[cfg(feature = "bellard")]
    fn JS_DupValue_real(ctx: *mut JSContext, v: JSValue);
    #[cfg(feature = "bellard")]
    fn JS_DupValueRT_real(rt: *mut JSRuntime, v: JSValue);
    #[cfg(feature = "bellard")]
    fn JS_FreeValue_real(ctx: *mut JSContext, v: JSValue);
    #[cfg(feature = "bellard")]
    fn JS_FreeValueRT_real(rt: *mut JSRuntime, v: JSValue);
    fn JS_NewBool_real(ctx: *mut JSContext, v: bool) -> JSValue;
    fn JS_NewInt32_real(ctx: *mut JSContext, v: i32) -> JSValue;

    #[cfg(feature = "bellard")]
    fn JS_NewFloat64_real(ctx: *mut JSContext, v: f64) -> JSValue;

    fn JS_VALUE_IS_NAN_real(v: JSValue) -> bool;
    fn JS_VALUE_GET_FLOAT64_real(v: JSValue) -> f64;
    fn JS_VALUE_GET_NORM_TAG_real(v: JSValue) -> ::std::os::raw::c_int;
    fn JS_IsNumber_real(v: JSValue) -> bool;
    fn JS_IsBigInt_real(ctx: *mut JSContext, v: JSValue) -> bool;
    fn JS_IsBool_real(v: JSValue) -> bool;
    fn JS_IsNull_real(v: JSValue) -> bool;
    fn JS_IsUndefined_real(v: JSValue) -> bool;
    fn JS_IsException_real(v: JSValue) -> bool;
    fn JS_IsUninitialized_real(v: JSValue) -> bool;
    fn JS_IsString_real(v: JSValue) -> bool;
    fn JS_IsSymbol_real(v: JSValue) -> bool;
    fn JS_IsObject_real(v: JSValue) -> bool;
    fn JS_ToUint32_real(ctx: *mut JSContext, pres: u32, val: JSValue) -> u32;
    #[cfg(feature = "bellard")]
    fn JS_SetProperty_real(ctx: *mut JSContext, this_obj: JSValue, prop: JSAtom, val: JSValue) -> ::std::os::raw::c_int;
    fn JS_NewCFunction_real(ctx: *mut JSContext, func: *mut JSCFunction, name: *const ::std::os::raw::c_char, length: ::std::os::raw::c_int) -> JSValue;
    fn JS_NewCFunctionMagic_real(ctx: *mut JSContext, func: *mut JSCFunctionMagic, name: *const ::std::os::raw::c_char, length: ::std::os::raw::c_int, cproto: JSCFunctionEnum, magic: ::std::os::raw::c_int) -> JSValue;
}

pub unsafe fn JS_ValueGetTag(v: JSValue) -> i32 {
    unsafe {
        JS_ValueGetTag_real(v)
    }
}

/// Increment the refcount of this value
#[cfg(feature = "bellard")]
pub unsafe fn JS_DupValue(ctx: *mut JSContext, v: JSValue) {
    unsafe {
        JS_DupValue_real(ctx, v);
    }
}

/// Increment the refcount of this value
#[cfg(feature = "bellard")]
pub unsafe fn JS_DupValueRT(rt: *mut JSRuntime, v: JSValue) {
    unsafe {
        JS_DupValueRT_real(rt, v);
    }
}

/// Decrement the refcount of this value
#[cfg(feature = "bellard")]
pub unsafe fn JS_FreeValue(ctx: *mut JSContext, v: JSValue) {
    unsafe {
        JS_FreeValue_real(ctx, v);
    }
}

/// Decrement the refcount of this value
#[cfg(feature = "bellard")]
pub unsafe fn JS_FreeValueRT(rt: *mut JSRuntime, v: JSValue) {
    unsafe {
        JS_FreeValueRT_real(rt, v);
    }
}

/// create a new boolean value
pub unsafe fn JS_NewBool(ctx: *mut JSContext, v: bool) -> JSValue {
    unsafe {
        JS_NewBool_real(ctx, v)
    }
}

/// create a new int32 value
pub unsafe fn JS_NewInt32(ctx: *mut JSContext, v: i32) -> JSValue {
    unsafe {
        JS_NewInt32_real(ctx, v)
    }
}

/// create a new f64 value, please note that if the passed f64 fits in a i32 this will return a value with flag 0 (i32)
#[cfg(feature = "bellard")]
pub unsafe fn JS_NewFloat64(ctx: *mut JSContext, v: f64) -> JSValue {
    unsafe {
        JS_NewFloat64_real(ctx, v)
    }
}

/// check if a JSValue is a NaN value
pub unsafe fn JS_VALUE_IS_NAN(v: JSValue) -> bool {
    unsafe {
        JS_VALUE_IS_NAN_real(v)
    }
}

/// get a f64 value from a JSValue
pub unsafe fn JS_VALUE_GET_FLOAT64(v: JSValue) -> f64 {
    unsafe {
        JS_VALUE_GET_FLOAT64_real(v)
    }
}

/// same as JS_VALUE_GET_TAG, but return JS_TAG_FLOAT64 with NaN boxing
pub unsafe fn JS_VALUE_GET_NORM_TAG(v: JSValue) -> ::std::os::raw::c_int {
    unsafe {
        JS_VALUE_GET_NORM_TAG_real(v)
    }
}

/// check if a JSValue is a Number
pub unsafe fn JS_IsNumber(v: JSValue) -> bool {
    unsafe {
        JS_IsNumber_real(v)
    }
}

/// check if a JSValue is a BigInt
pub unsafe fn JS_IsBigInt(ctx: *mut JSContext, v: JSValue) -> bool {
    unsafe {
        JS_IsBigInt_real(ctx, v)
    }
}

/// check if a JSValue is a Boolean
pub unsafe fn JS_IsBool(v: JSValue) -> bool {
    unsafe {
        JS_IsBool_real(v)
    }
}

/// check if a JSValue is null
pub unsafe fn JS_IsNull(v: JSValue) -> bool {
    unsafe {
        JS_IsNull_real(v)
    }
}

/// check if a JSValue is Undefined
pub unsafe fn JS_IsUndefined(v: JSValue) -> bool {
    unsafe {
        JS_IsUndefined_real(v)
    }
}

/// check if a JSValue is an Exception
pub unsafe fn JS_IsException(v: JSValue) -> bool {
    unsafe {
        JS_IsException_real(v)
    }
}

/// check if a JSValue is initialized
pub unsafe fn JS_IsUninitialized(v: JSValue) -> bool {
    unsafe {
        JS_IsUninitialized_real(v)
    }
}

/// check if a JSValue is a String
pub unsafe fn JS_IsString(v: JSValue) -> bool {
    unsafe {
        JS_IsString_real(v)
    }
}

/// check if a JSValue is a Symbol
pub unsafe fn JS_IsSymbol(v: JSValue) -> bool {
    unsafe {
        JS_IsSymbol_real(v)
    }
}

/// check if a JSValue is an Object
pub unsafe fn JS_IsObject(v: JSValue) -> bool {
    unsafe {
        JS_IsObject_real(v)
    }
}

/// get a u32 value from a JSValue
pub unsafe fn JS_ToUint32(ctx: *mut JSContext, pres: u32, val: JSValue) -> u32 {
    unsafe {
        JS_ToUint32_real(ctx, pres, val)
    }
}

/// set a property of an object identified by a JSAtom
#[cfg(feature = "bellard")]
pub unsafe fn JS_SetProperty(ctx: *mut JSContext, this_obj: JSValue, prop: JSAtom, val: JSValue) -> ::std::os::raw::c_int {
    unsafe {
        JS_SetProperty_real(ctx, this_obj, prop, val)
    }
}

/// create a new Function based on a JSCFunction
pub unsafe fn JS_NewCFunction(ctx: *mut JSContext, func: *mut JSCFunction, name: *const ::std::os::raw::c_char, length: ::std::os::raw::c_int) -> JSValue {
    unsafe {
        JS_NewCFunction_real(ctx, func, name, length)
    }
}

/// create a new Function based on a JSCFunction
pub unsafe fn JS_NewCFunctionMagic(ctx: *mut JSContext, func: *mut JSCFunctionMagic, name: *const ::std::os::raw::c_char, length: ::std::os::raw::c_int, cproto: JSCFunctionEnum, magic: ::std::os::raw::c_int) -> JSValue {
    unsafe {
        JS_NewCFunctionMagic_real(ctx, func, name, length, cproto, magic)
    }
}
