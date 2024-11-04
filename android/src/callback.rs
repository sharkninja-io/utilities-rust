use jni::objects::{GlobalRef, JObject, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use log::error;

pub mod holder;

const RESULT_INVOKE_SIG: &str = "(Lcom/sharkninja/api/mantleutilities/Result;)V";

pub fn invoke_callback(env: JNIEnv, callback: &GlobalRef, sig: String, args: &[JValue]) {
    env.call_method(callback, "invoke", &sig, args)
        .unwrap_or_else(|err| {
            error!("Error invoking callback for JNI: {:?}", err);
            env.exception_describe().unwrap();
            panic!();
        });
}

pub fn invoke_result_callback(env: JNIEnv, callback: &GlobalRef, result: JObject) {
    invoke_callback(
        env,
        callback,
        RESULT_INVOKE_SIG.to_string(),
        &[JValue::Object(result)],
    );
}

pub fn invoke_callback_object(
    env: JNIEnv,
    callback: &GlobalRef,
    sig: String,
    args: &[JValue],
) -> jobject {
    let response = env
        .call_method(callback, "invoke", &sig, args)
        .unwrap_or_else(|err| {
            error!("Error invoking callback for JNI: {:?}", err);
            env.exception_describe().unwrap();
            panic!();
        })
        .l()
        .unwrap_or_else(|err| {
            error!(
                "Error converting JValue to JObject in invoke_callback_object: {:?}",
                err
            );
            env.exception_describe().unwrap();
            panic!();
        });
    *response
}
