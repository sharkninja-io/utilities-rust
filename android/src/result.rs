use crate::java_class_names;
use crate::java_class_names::{CLASSNAMES, CLASSREFSMAP};
use crate::mantle_error::JavaMantleError;
use crate::traits::JavaClass;
use ctor::ctor;
use jni::objects::{JClass, JObject, JValue};
use jni::JNIEnv;
use log::error;
use mantle_utilities::error::MantleResultError;

const JAVA_SUCCESS_RESULT: &str = "com/sharkninja/api/mantleutilities/Result$Success";
const JAVA_FAIL_RESULT: &str = "com/sharkninja/api/mantleutilities/Result$Fail";

const ERROR_RESULT_SIG: &str = "(Lcom/sharkninja/api/mantleutilities/MantleError;)V";
const SUCCESS_RESULT_SIG: &str = "(Ljava/lang/Object;)V";

#[ctor]
fn add_class_names() {
    let mut names = CLASSNAMES.lock().unwrap();
    names.push(JAVA_SUCCESS_RESULT.to_owned()); // Success Result
    names.push(JAVA_FAIL_RESULT.to_owned()); // Failed Result
}

pub struct AndroidResult<T>(pub Result<T, Box<dyn MantleResultError>>);

impl<T> AndroidResult<T> {
    pub fn to_jobject_result<Rust>(self, jni_env: JNIEnv) -> JObject
    where
        T: JavaClass<Rust>,
    {
        match self.0 {
            Ok(success) => {
                let j_class = java_class_names::get_class(Some(&success));
                let success_object = success.j_object(jni_env, j_class);
                success_result(jni_env, JValue::from(success_object))
            }
            Err(err) => error_result(jni_env, err),
        }
    }
}

pub fn success_result<'a>(jni_env: JNIEnv<'a>, object: JValue<'a>) -> JObject<'a> {
    let args = &[object];
    let map = CLASSREFSMAP.lock().unwrap();
    jni_env
        .new_object(
            JClass::from(map.get(JAVA_SUCCESS_RESULT).unwrap().as_obj()),
            SUCCESS_RESULT_SIG,
            args,
        )
        .unwrap_or_else(|err| {
            error!(
                "Error creating success result for object for JNI: {:?}",
                err
            );
            jni_env.exception_describe().unwrap();
            panic!();
        })
}

pub fn error_result(jni_env: JNIEnv, err: Box<dyn MantleResultError>) -> JObject {
    let jme = JavaMantleError(err);
    let j_mantle_error_class = java_class_names::get_class(Some(&jme));
    let args = [JValue::from(jme.j_object(jni_env, j_mantle_error_class))];
    let map = CLASSREFSMAP.lock().unwrap();
    jni_env
        .new_object(
            JClass::from(map.get(JAVA_FAIL_RESULT).unwrap().as_obj()),
            ERROR_RESULT_SIG,
            &args,
        )
        .unwrap_or_else(|jni_error| {
            error!("Error creating Fail Result for JNI: {:?}", jni_error);
            jni_env.exception_describe().unwrap();
            panic!();
        })
}
