use crate::jni_exts::jobject::MantleJObject;
use jni::objects::JClass;
use jni::sys::jobject;
use jni::JNIEnv;

pub trait JavaClass<Rust> {
    // Even though most implementers do not need a reference to self,
    // enums do. So in order to make life a (little) easier
    // full_name and signature are static functions that take in
    // an option of self.
    fn full_name(instance: Option<&Self>) -> String;
    fn signature(instance: Option<&Self>) -> String;
    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject;
    fn new(rust_object: Rust) -> Self;
}

pub trait JObjectRustBridge<Rust> {
    fn rust_object(j_object: MantleJObject, jni_env: JNIEnv) -> Option<Rust>;
}
