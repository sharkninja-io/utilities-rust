use crate::jni_exts::option_traits::AndroidOption;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::JNIEnv;
use log::error;

pub struct AndroidString(pub String);
impl AndroidString {
    pub fn to_jstring(self, jni_env: JNIEnv) -> JString {
        jni_env.new_string(self.0).unwrap_or_else(|err| {
            error!("Error creating jstring: {:?}", err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
    }

    pub fn to_jclass(self, jni_env: JNIEnv) -> JClass {
        jni_env.find_class(self.0.to_owned()).unwrap_or_else(|err| {
            error!("Error finding java class {:?}: {:?}", self.0, err);
            jni_env.exception_describe().unwrap();
            panic!();
        })
    }
}

impl AndroidOption for Option<String> {
    fn to_j_value(self, jni_env: JNIEnv) -> JValue {
        match self {
            None => JValue::from(JObject::null()),
            Some(value) => JValue::from(AndroidString(value).to_jstring(jni_env).into_inner()),
        }
    }
}
