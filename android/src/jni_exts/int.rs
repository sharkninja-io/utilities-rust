use crate::java_class_names;
use crate::jni_exts::option_traits::AndroidOption;
use jni::objects::{JObject, JValue};
use jni::sys::jint;
use jni::JNIEnv;
use log::error;

pub fn u32_to_integer_class(jni_env: JNIEnv, value: u32) -> JValue {
    i32_to_integer_class(jni_env, value as i32)
}

pub fn i32_to_integer_class(jni_env: JNIEnv, value: i32) -> JValue {
    let class = java_class_names::get_class_from_name("java/lang/Integer".to_string());
    let object = jni_env
        .new_object(class, "(I)V", &[JValue::Int(value)])
        .unwrap_or_else(|err| {
            error!("Error creating Integer for JNI: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        });
    JValue::from(object)
}

pub fn get_java_integer_class_value(object: JObject, jni_env: JNIEnv) -> jint {
    jni_env
        .call_method(object, "intValue", "()I", &[])
        .unwrap_or_else(|err| {
            error!("Error calling intValue(): {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
        .i()
        .unwrap_or_else(|err| {
            error!("Error converting returned value to jint: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
}

impl AndroidOption for Option<u32> {
    fn to_j_value(self, jni_env: JNIEnv) -> JValue {
        match self {
            None => JValue::from(JObject::null()),
            Some(value) => u32_to_integer_class(jni_env, value),
        }
    }
}

impl AndroidOption for Option<i32> {
    fn to_j_value(self, jni_env: JNIEnv) -> JValue {
        match self {
            None => JValue::from(JObject::null()),
            Some(value) => i32_to_integer_class(jni_env, value),
        }
    }
}
