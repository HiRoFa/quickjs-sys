#![allow(improper_ctypes)]
//! FFI Bindings for [quickjs](https://bellard.org/quickjs/),
//! a Javascript engine.
//! See the [quickjs](https://crates.io/crates/quickjs) crate for a high-level
//! wrapper.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// import the functions from static-functions.c

include!("static-functions.rs");

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    // Small sanity test that starts the runtime and evaluates code.
    #[test]
    fn test_eval() {
        unsafe {
            let rt = JS_NewRuntime();
            let ctx = JS_NewContext(rt);

            let code_str = "1 + 1\0";
            let code = CStr::from_bytes_with_nul(code_str.as_bytes()).unwrap();
            let script = CStr::from_bytes_with_nul("script\0".as_bytes()).unwrap();

            let value = JS_Eval(
                ctx,
                code.as_ptr(),
                (code_str.len() - 1) as usize,
                script.as_ptr(),
                JS_EVAL_TYPE_GLOBAL as i32,
            );
            assert_eq!(value.tag, 0);
            assert_eq!(value.u.int32, 2);

            JS_DupValue(ctx, value);
            JS_FreeValue(ctx, value);

            let ival = JS_NewInt32(ctx, 12);
            assert_eq!(ival.tag, 0);
            //let fval = JS_NewFloat64(ctx, f64::MAX);
            //assert_eq!(fval.tag, 7);
            let bval = JS_NewBool(ctx, true);
            assert_eq!(bval.tag, 1);
        }
    }
}
