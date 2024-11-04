use crate::error::MantleResultError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MqttError {
    #[error("{0}")]
    ConnectError(anyhow::Error),
    #[error("{0}")]
    DisconnectError(anyhow::Error),
    #[error("{0}")]
    ReconnectError(anyhow::Error),
    #[error("{0}")]
    SubscribeError(anyhow::Error),
    #[error("{0}")]
    UnsubscribeError(anyhow::Error),
    #[error("{0}")]
    CreateClientError(anyhow::Error),
    #[error("{0}")]
    PublishMessageError(anyhow::Error),
    #[error("{0}")]
    ReceiveMessageError(anyhow::Error),
}

impl MantleResultError for MqttError {
    fn error_type(&self) -> String {
        match self {
            MqttError::ConnectError(_) => "ConnectError",
            MqttError::DisconnectError(_) => "DisconnectError",
            MqttError::ReconnectError(_) => "ReconnectError",
            MqttError::SubscribeError(_) => "SubscribeError",
            MqttError::UnsubscribeError(_) => "UnsubscribeError",
            MqttError::CreateClientError(_) => "CreateClientError",
            MqttError::PublishMessageError(_) => "PublishMessageError",
            MqttError::ReceiveMessageError(_) => "ReceiveMessageError",
        }
        .to_owned()
    }

    fn error_description(&self) -> String {
        self.to_string()
    }
}

// Declaring to be implemented in specific clients
pub struct MqttClient
{
    ///Creates a new MQTT client which can connect to an MQTT broker.
    pub setup:  fn (host: &str, port: u32),
    /// Connects to an MQTT broker
    pub connect: fn () -> Result<(), MqttError>,
    /// Disconnects from the MQTT broker.
    pub disconnect: fn () -> Result<(), MqttError>,
    /// Sets the default timeout used for synchronous operations.
    pub set_timeout: fn (secs: u32),
    /// Attempts to reconnect to the broker. This can only be called after a connection was initially made or attempted. It will retry with the same connect options.
    pub reconnect: fn () -> Result<(), MqttError>,
    /// Subscribes to a single topic.
    ///
    /// `topic` - The topic name.
    ///
    /// `qos` - The quality of service requested for messages.
    pub subscribe: fn(topic: &str, qos: i32) -> Result<(), MqttError>,
    /// Unsubscribes from a single topic.
    ///
    /// `topic` - The topic name.
    pub unsubscribe: fn ( topic: &str) -> Result<(), MqttError>,
    /// Determines if this client is currently connected to an MQTT broker.
    pub is_connected: fn () -> bool,
    /// Publishes a message to an MQTT broker
    ///
    /// `topic` - The topic name.
    ///
    /// `qos` - The quality of service requested for messages.
    ///
    /// `payload` The binary payload of the message (now this only string)
    pub publish_message: fn ( topic: &str, qos: i32, payload: &str) -> Result<(), MqttError>,
    /// Blocks the current thread until a message is received or the channel is empty and disconnected.
    pub receive_message: fn () -> Result<String, MqttError>
}


