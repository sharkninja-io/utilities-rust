use crate::java_class_names;
use crate::jni_exts::option_traits::AndroidOption;
use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use log::error;

pub fn bool_to_bool_class(jni_env: JNIEnv, value: bool) -> JValue {
    let class = java_class_names::get_class_from_name("java/lang/Boolean".to_string());
    let object = jni_env
        .new_object(class, "(Z)V", &[JValue::Bool(value.into())])
        .unwrap_or_else(|err| {
            error!("Error creating Boolean for JNI: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        });
    JValue::from(object)
}

pub fn get_java_boolean_class_value(object: JObject, jni_env: JNIEnv) -> bool {
    jni_env
        .call_method(object, "booleanValue", "()Z", &[])
        .unwrap_or_else(|err| {
            error!("Error calling booleanValue(): {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
        .z()
        .unwrap_or_else(|err| {
            error!("Error converting returned value to bool: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
}

impl AndroidOption for Option<bool> {
    fn to_j_value(self, jni_env: JNIEnv) -> JValue {
        match self {
            None => JValue::from(JObject::null()),
            Some(value) => bool_to_bool_class(jni_env, value),
        }
    }
}
