//! If the tests compiles they pass.

use android_binding_generator::impl_android_binding;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jdouble, jint, jlong, jobject, jstring};
use jni::JNIEnv;
use mantle_utilities::error::MantleResultError;
use serde::{Deserialize, Serialize};

type TestResult<T> = Result<T, Box<dyn MantleResultError>>;

#[derive(Serialize, Deserialize)]
struct SerializableType(String);

#[test]
fn compiles_with_explicit_arg_conversions() {
    fn test(
        _: i32,
        _: u32,
        _: u8,
        _: i64,
        _: f64,
        _: bool,
        _: String,
        _: Vec<u8>,
        _: Vec<String>,
        _: SerializableType,
        _: Option<i32>,
        _: Option<u32>,
        _: Option<bool>,
        _: Option<String>,
        _: Option<SerializableType>,
    ) -> TestResult<()> {
        unimplemented!()
    }

    impl_android_binding!(
        test_binding_1(
            JNIEnv,
            jint = I32,
            jint = U32,
            jint = U8,
            jlong = I64,
            jdouble = Double,
            jboolean = Boolean,
            JString = String,
            JObject = Bytes,
            JObject = StringList,
            JObject = Serialized(SerializableType),
            JObject = OptionalI32,
            JObject = OptionalU32,
            JObject = OptionalBool,
            JString = OptionalString,
            JObject = OptionalSerialized(SerializableType),
        ) -> jobject = FallibleVoid,
        test
    );
}

#[test]
fn compiles_with_explicit_return_conversions() {
    fn test_i32() -> i32 {
        unimplemented!()
    }

    fn test_i64() -> i64 {
        unimplemented!()
    }

    fn test_boolean() -> bool {
        unimplemented!()
    }

    fn test_string() -> String {
        unimplemented!()
    }

    fn test_bytes() -> Vec<u8> {
        unimplemented!()
    }

    fn test_void() {
        unimplemented!()
    }

    fn test_fallible_i32() -> TestResult<i32> {
        unimplemented!()
    }

    fn test_fallible_u32() -> TestResult<u32> {
        unimplemented!()
    }

    fn test_fallible_i64() -> TestResult<i64> {
        unimplemented!()
    }

    fn test_fallible_double() -> TestResult<f64> {
        unimplemented!()
    }

    fn test_fallible_bool() -> TestResult<bool> {
        unimplemented!()
    }

    fn test_fallible_string() -> TestResult<String> {
        unimplemented!()
    }

    fn test_fallible_u32_list() -> TestResult<Vec<u32>> {
        unimplemented!()
    }

    fn test_fallible_serialized() -> TestResult<SerializableType> {
        unimplemented!()
    }

    fn test_fallible_void() -> TestResult<()> {
        unimplemented!()
    }

    impl_android_binding!(test_binding_2(JNIEnv) -> jint = I32, test_i32);
    impl_android_binding!(test_binding_returns_i64(JNIEnv) -> jlong = I64, test_i64);
    impl_android_binding!(test_binding_3(JNIEnv) -> jboolean = Boolean, test_boolean);
    impl_android_binding!(test_binding_4(JNIEnv) -> jstring = String, test_string);
    impl_android_binding!(test_binding_explicit_bytes_output(JNIEnv) -> jobject = Bytes, test_bytes);
    impl_android_binding!(test_binding_5(JNIEnv) -> () = Void, test_void);
    impl_android_binding!(test_binding_6(JNIEnv) -> jobject = FallibleI32, test_fallible_i32);
    impl_android_binding!(test_binding_fallible_u32(JNIEnv) -> jobject = FallibleU32, test_fallible_u32);
    impl_android_binding!(test_binding_fallible_i64(JNIEnv) -> jobject = FallibleI64, test_fallible_i64);
    impl_android_binding!(test_binding_fallible_double(JNIEnv) -> jobject = FallibleDouble, test_fallible_double);
    impl_android_binding!(test_binding_7(JNIEnv) -> jobject = FallibleBoolean, test_fallible_bool);
    impl_android_binding!(test_binding_8(JNIEnv) -> jobject = FallibleString, test_fallible_string);
    impl_android_binding!(test_binding_fallible_u32_list(JNIEnv) -> jobject = FallibleU32List, test_fallible_u32_list);
    impl_android_binding!(test_binding_9(JNIEnv) -> jobject = FallibleSerialized(SerializableType), test_fallible_serialized);
    impl_android_binding!(test_binding_10(JNIEnv) -> jobject = FallibleVoid, test_fallible_void);
}

#[test]
fn does_not_pass_jclass_to_rust() {
    fn test_jclass() {}

    fn test_jclass_with_arg(_: i32) {}

    impl_android_binding!(test_binding_11(JNIEnv, JClass), test_jclass);
    impl_android_binding!(test_binding_12(JNIEnv, JClass, jint), test_jclass_with_arg);
}

#[test]
fn can_infer_args() {
    fn test(_: i32, _: i64, _: f64, _: bool, _: String) {}

    impl_android_binding!(
        test_binding_13(JNIEnv, jint, jlong, jdouble, jboolean, JString),
        test
    );
}

#[test]
fn can_infer_return_types() {
    fn test_void() {}

    fn test_i32() -> i32 {
        unimplemented!()
    }

    fn test_i64() -> i64 {
        unimplemented!()
    }

    fn test_boolean() -> bool {
        unimplemented!()
    }

    fn test_string() -> String {
        unimplemented!()
    }

    impl_android_binding!(test_binding_14(JNIEnv), test_void);
    impl_android_binding!(test_binding_15(JNIEnv) -> jint, test_i32);
    impl_android_binding!(test_binding_infer_i64(JNIEnv) -> jlong, test_i64);
    impl_android_binding!(test_binding_16(JNIEnv) -> jboolean, test_boolean);
    impl_android_binding!(test_binding_17(JNIEnv) -> jstring, test_string);
}
