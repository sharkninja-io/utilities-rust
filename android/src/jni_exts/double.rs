use crate::java_class_names;
use crate::jni_exts::option_traits::AndroidOption;
use jni::objects::{JObject, JValue};
use jni::sys::jdouble;
use jni::JNIEnv;
use log::error;

pub fn f64_to_double_class(jni_env: JNIEnv, value: f64) -> JValue {
    let class = java_class_names::get_class_from_name("java/lang/Double".to_string());
    let object = jni_env
        .new_object(class, "(D)V", &[JValue::Double(value)])
        .unwrap_or_else(|err| {
            error!("Error creating Double for JNI: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        });
    JValue::from(object)
}

pub fn get_java_double_class_value(object: JObject, jni_env: JNIEnv) -> jdouble {
    jni_env
        .call_method(object, "doubleValue", "()D", &[])
        .unwrap_or_else(|err| {
            error!("Error calling doubleValue(): {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
        .d()
        .unwrap_or_else(|err| {
            error!("Error converting returned value to jdouble: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
}

impl AndroidOption for Option<f64> {
    fn to_j_value(self, jni_env: JNIEnv) -> JValue {
        match self {
            None => JValue::from(JObject::null()),
            Some(value) => f64_to_double_class(jni_env, value),
        }
    }
}
