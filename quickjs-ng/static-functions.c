#include "quickjs.h"

// These are static inline functions in quickjs.h so bindgen does not pick 
// them up.
// We use define simple wrapper functions to make them available to bindgen,
// and therefore make them usable from Rust.

int JS_ValueGetTag_real(JSValue v) {
    return JS_VALUE_GET_TAG(v);
}

void JS_FreeValue_real(JSContext *ctx, JSValue v) {
    JS_FreeValue(ctx, v);
}

void JS_FreeValueRT_real(JSRuntime *rt, JSValue v) {
    return JS_FreeValueRT(rt, v);
}

void JS_DupValue_real(JSContext *ctx, JSValue v) {
    JS_DupValue(ctx, v);
}

JSValue JS_DupValueRT_real(JSRuntime *rt, JSValueConst v) {
    return JS_DupValueRT(rt, v);
}

//JSValue JS_NewFloat64_real(JSContext *ctx, double d) {
//    return JS_NewFloat64(ctx, d);
//}

JSValue JS_NewInt32_real(JSContext *ctx, int32_t val) {
    return JS_NewInt32(ctx, val);
}

JSValue JS_NewBool_real(JSContext *ctx, bool val) {
    return JS_NewBool(ctx, val) ;
}

bool JS_VALUE_IS_NAN_real(JSValue v) {
    return JS_VALUE_IS_NAN(v);
}

double JS_VALUE_GET_FLOAT64_real(JSValue v) {
    return JS_VALUE_GET_FLOAT64(v);
}

int JS_VALUE_GET_NORM_TAG_real(JSValue v) {
    return JS_VALUE_GET_NORM_TAG(v);
}

bool JS_IsNumber_real(JSValueConst v) {
    return JS_IsNumber(v);
}

bool JS_IsBigInt_real(JSContext *ctx, JSValueConst v) {
    return JS_IsBigInt(ctx, v);
}

bool JS_IsBool_real(JSValueConst v) {
    return JS_IsBool(v);
}

bool JS_IsNull_real(JSValueConst v) {
    return JS_IsNull(v);
}

bool JS_IsUndefined_real(JSValueConst v) {
    return JS_IsUndefined(v);
}

bool JS_IsException_real(JSValueConst v) {
    return JS_IsException(v);
}

bool JS_IsUninitialized_real(JSValueConst v) {
    return JS_IsUninitialized(v);
}

bool JS_IsString_real(JSValueConst v) {
    return JS_IsString(v);
}

bool JS_IsSymbol_real(JSValueConst v) {
    return JS_IsSymbol(v);
}

bool JS_IsObject_real(JSValueConst v) {
    return JS_IsObject(v);
}

int JS_ToUint32_real(JSContext *ctx, uint32_t *pres, JSValueConst val) {
    return JS_ToUint32(ctx, pres, val);
}

int JS_SetProperty_real(JSContext *ctx, JSValueConst this_obj, JSAtom prop, JSValue val) {
    return JS_SetProperty(ctx, this_obj, prop, val);
}

JSValue JS_NewCFunction_real(JSContext *ctx, JSCFunction *func, const char *name,int length) {
    return JS_NewCFunction(ctx, func, name, length);
}

JSValue JS_NewCFunctionMagic_real(JSContext *ctx, JSCFunctionMagic *func, const char *name, int length, JSCFunctionEnum cproto, int magic) {
    return JS_NewCFunctionMagic(ctx, func, name, length, cproto, magic);
}
