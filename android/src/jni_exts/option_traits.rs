use jni::objects::JValue;
use jni::JNIEnv;

pub trait AndroidOption {
    fn to_j_value(self, jni_env: JNIEnv) -> JValue;
}
