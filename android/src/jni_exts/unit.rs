use crate::java_class_names::CLASSNAMES;
use crate::java_signatures::VOID_SIG;
use crate::traits::JavaClass;
use ctor::ctor;
use jni::objects::JClass;
use jni::sys::jobject;
use jni::JNIEnv;
use log::error;

const KOTLIN_UNIT: &str = "kotlin/Unit";
pub const KOTLIN_UNIT_SIG: &str = "Lkotlin/Unit;";

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(AndroidUnit::full_name(None));
}

pub type AndroidUnit = ();
impl JavaClass<()> for AndroidUnit {
    fn full_name(_instance: Option<&Self>) -> String {
        KOTLIN_UNIT.to_owned()
    }

    fn signature(_instance: Option<&Self>) -> String {
        ["(", ")", VOID_SIG].concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let object = jni_env
            .new_object(j_class, AndroidUnit::signature(None), &[])
            .unwrap_or_else(|err| {
                error!("Error creating Unit object for JNI: {:?}", err);
                jni_env.exception_describe().unwrap();
                panic!();
            });
        *object
    }

    fn new(_rust_object: AndroidUnit) -> Self {}
}
