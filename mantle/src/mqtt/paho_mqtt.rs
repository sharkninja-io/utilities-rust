use std::sync::{Arc, Mutex};
use std::time::Duration;

use paho_mqtt as mqtt;

use crate::mqtt::mqtt_client::MqttError;
use crate::mqtt::mqtt_ffi_wraper::SHARED_MQTT;
use once_cell::sync::OnceCell;
use super::mqtt_client::MqttClient;

static SHARED_PAHO_CLIENT: OnceCell<Arc<Mutex<PahoMqttClient>>> = OnceCell::new();
// Implementation of a specific MQTT client using the "paho_mqtt" script
pub struct PahoMqttClient {
    client: mqtt::Client,
}

// Implementation for PahoMqttClient
impl PahoMqttClient {

    pub fn set_shared_mqtt()
    {
        let new_shared_client = MqttClient {
            setup: |host, port| {

                SHARED_PAHO_CLIENT
                    .get_or_init(move || Arc::new(Mutex::new(PahoMqttClient::new(host, port))));
            },
            connect: || SHARED_PAHO_CLIENT.get().unwrap().lock().unwrap().connect(),
            disconnect: || {
                SHARED_PAHO_CLIENT
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .disconnect()
            },
            set_timeout: |secs| {
                SHARED_PAHO_CLIENT
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .set_timeout(secs)
            },
            reconnect: || {
                SHARED_PAHO_CLIENT
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .reconnect()
            },
            subscribe: |topic, qos| {
                SHARED_PAHO_CLIENT
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .subscribe(topic, qos)
            },
            unsubscribe: |topic| {
                SHARED_PAHO_CLIENT
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .unsubscribe(topic)
            },
            is_connected: || {
                SHARED_PAHO_CLIENT
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .is_connected()
            },
            publish_message: |topic, qos, payload| {
                SHARED_PAHO_CLIENT
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .publish_message(topic, qos, payload)
            },
            receive_message: || {
                SHARED_PAHO_CLIENT
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .receive_message()
            },
        };
        
        let mut locked_client = SHARED_MQTT.get().unwrap().lock().unwrap();
        *locked_client = new_shared_client;
    }
    pub fn new(host: &str, port: u32) -> Self {
        let create_opts = mqtt::CreateOptionsBuilder::new()
            .server_uri(format!("tcp://{}:{}", host, port))
            .finalize();

        Self {
            client: mqtt::Client::new(create_opts).unwrap(),
        }
    }

    pub fn connect(&self) -> Result<(), MqttError> {
        self.client
            .connect(None)
            .map_err(|err| MqttError::ConnectError(err.into()))?;
        Ok(())
    }

    pub fn disconnect(&self) -> Result<(), MqttError> {
        self.client
            .disconnect(None)
            .map_err(|err| MqttError::DisconnectError(err.into()))
    }

    pub fn set_timeout(&mut self, secs: u32) {
        let timeout = Duration::from_secs(secs as u64);
        self.client.set_timeout(timeout);
    }

    pub fn reconnect(&self) -> Result<(), MqttError> {
        self.client
            .reconnect()
            .map_err(|err| MqttError::ReconnectError(err.into()))?;
        Ok(())
    }

    pub fn subscribe(&self, topic: &str, qos: i32) -> Result<(), MqttError> {
        self.client
            .subscribe(topic, qos)
            .map_err(|err| MqttError::SubscribeError(err.into()))?;
        Ok(())
    }

    pub fn unsubscribe(&self, topic: &str) -> Result<(), MqttError> {
        self.client
            .unsubscribe(topic)
            .map_err(|err| MqttError::UnsubscribeError(err.into()))
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected()
    }

    pub fn publish_message(&self, topic: &str, qos: i32, payload: &str) -> Result<(), MqttError> {
        let msg = mqtt::MessageBuilder::new()
            .topic(topic)
            .payload(payload)
            .qos(qos)
            .finalize();

        self.client
            .publish(msg)
            .map_err(|err| MqttError::PublishMessageError(err.into()))?;
        Ok(())
    }
    pub fn receive_message(&self) -> Result<String, MqttError> {
        let rx = self.client.start_consuming();
        let received_message = rx
            .recv()
            .map_err(|err| MqttError::ReceiveMessageError(err.into()))?;
        Ok(received_message.unwrap().payload_str().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_timeout() {
        let mut client = PahoMqttClient::new("localhost", 1883);
        client.set_timeout(5);
        assert_eq!(client.client.timeout(), Duration::from_secs(5));
    }
    #[cfg(feature = "with_integrated_tests")]
    #[test]
    fn test_connection() {
        let topic = "test_topic";
        let host = "broker.emqx.io";
        let port = 1883;
        let qos = 1;
        let payload = "Hello, MQTT!";

        let client = PahoMqttClient::new(host, port);
        client.connect().unwrap();

        assert!(client.client.is_connected());

        client.subscribe(topic, qos).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));

        client.publish_message(topic, qos, payload).unwrap();
        let received_message = client.receive_message().unwrap();

        assert_eq!(received_message, payload);

        client.unsubscribe(topic).unwrap();

        client.disconnect().unwrap();
    }
}
