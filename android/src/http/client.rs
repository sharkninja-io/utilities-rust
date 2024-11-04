use jni::{
    objects::{JClass, JObject, JValue},
    JNIEnv,
};
use log::error;
use mantle_utilities::http::response::Response;
use mantle_utilities::http::{client::SHARED, request::Request};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

use crate::java_class_names::get_class_from_name;
use crate::traits::{JObjectRustBridge, JavaClass};
use crate::{invoke_callback_object, jni_exts::jobject::MantleJObject, CallbackStruct};

use super::request::{JavaRequest, JavaResponse, HTTP_REQUEST_SIG, HTTP_RESPONSE_SIG};

static REQUESTS_CB_STRUCT: Lazy<Arc<Mutex<CallbackStruct>>> =
    Lazy::new(|| Arc::new(Mutex::new(CallbackStruct::new())));

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn Java_com_sharkninja_api_mantleutilities_httpclient_HttpClient_00024Companion_extSetRequestsCallback(
    env: JNIEnv,
    _class: JClass,
    requests_callback: JObject,
) {
    if let Ok(mut guard) = REQUESTS_CB_STRUCT.lock() {
        guard.update(env, requests_callback);
        drop(guard);
    }
    if let Ok(mut shared) = SHARED.clone().lock() {
        shared.set_callback(handle_request);
        drop(shared);
    };
}

fn handle_request(request: Request) -> Response {
    let cbs = REQUESTS_CB_STRUCT.clone();
    if let Ok(cb_struct) = cbs.lock() {
        if let Some(jvm) = &cb_struct.jvm {
            if let Some(callback) = &cb_struct.callback.clone() {
                let env = jvm
                    //.get_env()
                    .attach_current_thread_permanently()
                    .unwrap_or_else(|err| {
                        error!("Error getting jvm env for http request: {:?}", err);
                        panic!();
                    });

                let java_request = JavaRequest(request);
                let request_class = get_class_from_name(JavaRequest::full_name(None));
                let request_object = java_request.j_object(env, request_class);

                let sig = ["(", HTTP_REQUEST_SIG, ")", HTTP_RESPONSE_SIG].concat();
                //drop(cbs);
                let response = invoke_callback_object(
                    env,
                    callback,
                    sig,
                    &[JValue::from(JObject::from(request_object))],
                );
                let response = MantleJObject(JObject::from(response));
                let response = JavaResponse::rust_object(response, env);
                return response.unwrap();
            }
        }
    }
    Response::default()
}
