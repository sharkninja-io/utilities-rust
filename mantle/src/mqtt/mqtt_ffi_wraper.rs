use std::sync::{Arc, Mutex};

use crate::error::MantleResultError;
use crate::mqtt::mqtt_client::MqttClient;
use once_cell::sync::OnceCell;

pub static SHARED_MQTT: OnceCell<Arc<Mutex<MqttClient>>> = OnceCell::new();

pub fn init() {
    SHARED_MQTT.get_or_init(|| {
        Arc::new(Mutex::new(MqttClient {
            setup: |host, port| {},
            connect: || { Ok(()) },
            disconnect: || { Ok(()) },
            set_timeout: |secs| {},
            reconnect: || { Ok(()) },
            subscribe: |topic, qos| { Ok(()) },
            unsubscribe: |topic| { Ok(()) },
            is_connected: || { false },
            publish_message: |topic, qos, payload| { Ok(()) },
            receive_message: || { Ok(String::new()) },
        }))
    });
}

pub fn setup(host: String, port: u32) {
    (SHARED_MQTT.get().unwrap().lock().unwrap().setup)(&host, port);
}

pub fn connect() -> Result<(), Box<dyn MantleResultError>> {
    (SHARED_MQTT.get().unwrap().lock().unwrap().connect)()
        .map_err(|err| Box::new(err) as Box<dyn MantleResultError>)
}

pub fn disconnect() -> Result<(), Box<dyn MantleResultError>> {
    (SHARED_MQTT.get().unwrap().lock().unwrap().disconnect)()
        .map_err(|err| Box::new(err) as Box<dyn MantleResultError>)
}

pub fn set_timeout(secs: u32) {
    (SHARED_MQTT.get().unwrap().lock().unwrap().set_timeout)(secs);
}

pub fn reconnect() -> Result<(), Box<dyn MantleResultError>> {
    (SHARED_MQTT.get().unwrap().lock().unwrap().reconnect)()
        .map_err(|err| Box::new(err) as Box<dyn MantleResultError>)
}

pub fn subscribe(topic: String, qos: i32) -> Result<(), Box<dyn MantleResultError>> {
    (SHARED_MQTT.get().unwrap().lock().unwrap().subscribe)(&topic, qos)
        .map_err(|err| Box::new(err) as Box<dyn MantleResultError>)
}

pub fn unsubscribe(topic: String) -> Result<(), Box<dyn MantleResultError>> {
    (SHARED_MQTT.get().unwrap().lock().unwrap().unsubscribe)(&topic)
        .map_err(|err| Box::new(err) as Box<dyn MantleResultError>)
}

pub fn is_connected() -> bool {
    (SHARED_MQTT.get().unwrap().lock().unwrap().is_connected)()
}

pub fn publish_message(
    topic: String,
    qos: i32,
    payload: String,
) -> Result<(), Box<dyn MantleResultError>> {
    (SHARED_MQTT.get().unwrap().lock().unwrap().publish_message)(&topic, qos, &payload)
        .map_err(|err| Box::new(err) as Box<dyn MantleResultError>)
}

pub fn receive_message() -> Result<String, Box<dyn MantleResultError>> {
    (SHARED_MQTT.get().unwrap().lock().unwrap().receive_message)()
        .map_err(|err| Box::new(err) as Box<dyn MantleResultError>)
}
