use crate::java_class_names::CLASSNAMES;
use crate::java_signatures::{STRING_SIG, VOID_SIG};
use crate::jni_exts::string::AndroidString;
use crate::traits::JavaClass;
use crate::JAVA_PACKAGE;
use ctor::ctor;
use jni::objects::{JClass, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use log::{debug, error};
use mantle_utilities::error::MantleResultError;

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JavaMantleError::full_name(None));
}

pub struct JavaMantleError(pub Box<dyn MantleResultError>);

impl JavaClass<Box<dyn MantleResultError>> for JavaMantleError {
    fn full_name(_instance: Option<&Self>) -> String {
        [JAVA_PACKAGE, "MantleError"].concat()
    }

    fn signature(_instance: Option<&Self>) -> String {
        ["(", STRING_SIG, STRING_SIG, ")", VOID_SIG].concat()
    }

    fn j_object(&self, jni_env: JNIEnv, j_class: JClass) -> jobject {
        let signature = JavaMantleError::signature(None);
        let error_type = AndroidString(self.0.error_type()).to_jstring(jni_env);
        let description = AndroidString(self.0.error_description()).to_jstring(jni_env);

        debug!("DEBUG: description: {}", self.0.to_string());
        // ** Order matters!!! Refer to com/sharkninja/api/mantleutilities/MantleError **
        let args = &[
            JValue::from(error_type.into_inner()),
            JValue::from(description.into_inner()),
        ];

        let mantle_error =
            jni_env
                .new_object(j_class, signature, args)
                .unwrap_or_else(|jni_error| {
                    error!("Error creating MantleError for JNI: {:?}", jni_error);
                    jni_env.exception_describe().unwrap();
                    panic!();
                });
        *mantle_error
    }

    fn new(rust_object: Box<dyn MantleResultError>) -> Self {
        Self(rust_object)
    }
}
