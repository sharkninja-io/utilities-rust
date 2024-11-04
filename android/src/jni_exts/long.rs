use crate::java_class_names;
use crate::jni_exts::option_traits::AndroidOption;
use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use log::error;

pub fn i64_to_long_class(jni_env: JNIEnv, value: i64) -> JValue {
    let class = java_class_names::get_class_from_name("java/lang/Long".to_string());
    let object = jni_env
        .new_object(class, "(J)V", &[JValue::Long(value)])
        .unwrap_or_else(|err| {
            error!("Error creating Long for JNI: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        });
    JValue::from(object)
}

impl AndroidOption for Option<i64> {
    fn to_j_value(self, jni_env: JNIEnv) -> JValue {
        match self {
            None => JValue::from(JObject::null()),
            Some(value) => i64_to_long_class(jni_env, value),
        }
    }
}
