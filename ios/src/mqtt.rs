use mantle_utilities::{error::MantleResultError, string::MantleStringPointer};
use std::os::raw::{c_char, c_int, c_uint};

use mantle_utilities::mqtt::mqtt_ffi_wraper;

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_init() {
    let host = MantleStringPointer(host).to_string();
    mqtt_ffi_wraper::init();
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_setup(host: *const c_char, port: c_uint) {
    let host = MantleStringPointer(host).to_string();
    mqtt_ffi_wraper::setup(host, port);
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_connect() -> Result<(), Box<dyn MantleResultError>> {
    mqtt_ffi_wraper::connect()
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_disconnect() -> Result<(), Box<dyn MantleResultError>> {
    mqtt_ffi_wraper::disconnect()
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_set_timeout(secs: c_uint) {
    mqtt_ffi_wraper::set_timeout(secs)
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_reconnect() -> Result<(), Box<dyn MantleResultError>> {
    mqtt_ffi_wraper::reconnect()
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_is_connected() -> bool {
    mqtt_ffi_wraper::is_connected()
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_subscribe(
    topic: *const c_char,
    qos: c_int,
) -> Result<(), Box<dyn MantleResultError>> {
    let topic = MantleStringPointer(topic).to_string();
    mqtt_ffi_wraper::subscribe(topic, qos)
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_unsubscribe(
    topic: *const c_char,
) -> Result<(), Box<dyn MantleResultError>> {
    let topic = MantleStringPointer(topic).to_string();
    mqtt_ffi_wraper::unsubscribe(topic)
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_publish_message(
    topic: *const c_char,
    qos: c_int,
    payload: *const c_char,
) -> Result<(), Box<dyn MantleResultError>> {
    let topic = MantleStringPointer(topic).to_string();
    let payload = MantleStringPointer(payload).to_string();
    mqtt_ffi_wraper::publish_message(topic, qos, payload)
}

#[no_mangle]
#[allow(improper_ctypes_definitions, clippy::missing_safety_doc)]
pub unsafe extern "C" fn ios_mqtt_client_receive_message(
) -> Result<String, Box<dyn MantleResultError>> {
    mqtt_ffi_wraper::receive_message()
}
