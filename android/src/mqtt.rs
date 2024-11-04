use jni::objects::JString;
use jni::sys::jint;
use jni::JNIEnv;
use mantle_utilities::error::MantleResultError;

use mantle_utilities::mqtt::mqtt_ffi_wraper;

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_init() {
    mqtt_ffi_wraper::init();
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_setup(env: JNIEnv, host: JString, port: jint) {
    let host = env
        .get_string(host)
        .expect("couldn't get java string")
        .into();
    mqtt_ffi_wraper::setup(host, port as u32);
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_connect() -> Result<(), Box<dyn MantleResultError>> {
    mqtt_ffi_wraper::connect()
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_disconnect() -> Result<(), Box<dyn MantleResultError>>
{
    mqtt_ffi_wraper::disconnect()
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_set_timeout(secs: jint) {
    mqtt_ffi_wraper::set_timeout(secs as u32)
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_reconnect() -> Result<(), Box<dyn MantleResultError>> {
    mqtt_ffi_wraper::reconnect()
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_is_connected() -> bool {
    mqtt_ffi_wraper::is_connected()
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_subscribe(
    env: JNIEnv,
    topic: JString,
    qos: jint,
) -> Result<(), Box<dyn MantleResultError>> {
    let topic = env
        .get_string(topic)
        .expect("couldn't get java string")
        .into();
    mqtt_ffi_wraper::subscribe(topic, qos)
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_unsubscribe(
    env: JNIEnv,
    topic: JString,
) -> Result<(), Box<dyn MantleResultError>> {
    let topic = env
        .get_string(topic)
        .expect("couldn't get java string")
        .into();
    mqtt_ffi_wraper::unsubscribe(topic)
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_publish_message(
    env: JNIEnv,
    topic: JString,
    qos: jint,
    payload: JString,
) -> Result<(), Box<dyn MantleResultError>> {
    let topic = env
        .get_string(topic)
        .expect("couldn't get java string")
        .into();
    let payload = env
        .get_string(payload)
        .expect("couldn't get java string")
        .into();
    mqtt_ffi_wraper::publish_message(topic, qos, payload)
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn android_mqtt_client_receive_message(
) -> Result<String, Box<dyn MantleResultError>> {
    mqtt_ffi_wraper::receive_message()
}
