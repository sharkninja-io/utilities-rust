use android_utilities::jni_exts;
use jni::objects::{JObject, JString};
use jni::sys::{jboolean, jdouble, jint, jlong, JNI_FALSE};
use jni::JNIEnv;
use serde::de::DeserializeOwned;

pub fn jlong_to_i64(value: jlong) -> i64 {
    value
}

pub fn jint_to_i32(value: jint) -> i32 {
    value
}

pub fn jint_to_u32(value: jint) -> u32 {
    value as u32
}

pub fn jint_to_u8(value: jint) -> u8 {
    value as u8
}

pub fn jdouble_to_f64(value: jdouble) -> f64 {
    value
}

pub fn jboolean_to_bool(value: jboolean) -> bool {
    value != JNI_FALSE
}

pub fn jstring_to_string(env: JNIEnv, value: JString) -> String {
    env.get_string(value)
        .expect("couldn't get java string")
        .into()
}

pub fn jobject_to_bytes(env: JNIEnv, bytes: JObject) -> Vec<u8> {
    env.convert_byte_array(*bytes).unwrap()
}

pub fn jobject_to_string_vec(env: JNIEnv, jlist: JObject) -> Vec<String> {
    let jlen = env
        .get_array_length(*jlist)
        .expect("couldn't get java array length");
    let capacity = usize::try_from(jlen).expect("got invalid java array length");
    let mut list = Vec::with_capacity(capacity);
    for idx in 0..jlen {
        let jobject = env.get_object_array_element(*jlist, idx).unwrap();
        let string = env
            .get_string(jobject.into())
            .expect("couldn't get java string");
        list.push(string.into())
    }
    list
}

pub fn jobject_to_serialized<T: DeserializeOwned>(
    env: JNIEnv,
    deserialized: JObject,
) -> serde_json::Result<T> {
    let bytes = jobject_to_bytes(env, deserialized);
    serde_json::from_slice(&bytes)
}

pub fn jobject_to_optional_i32(env: JNIEnv, value: JObject) -> Option<i32> {
    if value.is_null() {
        return None;
    }

    Some(jni_exts::int::get_java_integer_class_value(value, env))
}

pub fn jobject_to_optional_u32(env: JNIEnv, value: JObject) -> Option<u32> {
    jobject_to_optional_i32(env, value).map(|v| v as u32)
}

pub fn jobject_to_optional_bool(env: JNIEnv, value: JObject) -> Option<bool> {
    if value.is_null() {
        return None;
    }

    Some(jni_exts::bool::get_java_boolean_class_value(value, env))
}

pub fn jstring_to_optional_string(env: JNIEnv, jstring: JString) -> Option<String> {
    if jstring.is_null() {
        return None;
    }

    Some(jstring_to_string(env, jstring))
}

pub fn jobject_to_optional_serialized<T: DeserializeOwned>(
    env: JNIEnv,
    deserialized: JObject,
) -> serde_json::Result<Option<T>> {
    if deserialized.is_null() {
        return Ok(None);
    }

    jobject_to_serialized(env, deserialized).map(|v| Some(v))
}
