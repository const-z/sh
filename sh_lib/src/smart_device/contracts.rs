use bincode::{Decode, Encode, config::Configuration};

use crate::{
    errors::SmartHomeErrors,
    smart_device::{smart_socket::SocketData, smart_thermometer::ThermometerData},
};

const ENCODING_CONFIG: Configuration = bincode::config::standard();

#[derive(Clone, Debug, Encode, Decode)]
pub enum DeviceResponseData {
    Socket(SocketData),
    Thermometer(ThermometerData),
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct DeviceResponse {
    pub data: Option<DeviceResponseData>,
    pub success: bool,
    pub error: Option<String>,
}

pub trait DecodeEncode {
    fn decode(value: &[u8]) -> Result<DeviceResponse, SmartHomeErrors>;
    fn encode(&self) -> Result<Vec<u8>, SmartHomeErrors>;
}

impl DecodeEncode for DeviceResponse {
    fn decode(value: &[u8]) -> Result<DeviceResponse, SmartHomeErrors> {
        let r = bincode::decode_from_slice(value, ENCODING_CONFIG);

        if let Err(e) = r {
            return Err(SmartHomeErrors::decode_message_error(e.to_string()));
        }

        Ok(r.unwrap().0)
    }

    fn encode(&self) -> Result<Vec<u8>, SmartHomeErrors> {
        let r = bincode::encode_to_vec(self, ENCODING_CONFIG);

        if let Err(e) = r {
            return Err(SmartHomeErrors::decode_message_error(e.to_string()));
        }

        Ok(r.unwrap())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Commands {
    Unknown = 0,
    TurnOn = 1,
    TurnOff = 2,
    GetStatus = 3,
}

impl From<i32> for Commands {
    fn from(value: i32) -> Self {
        match value {
            1 => Commands::TurnOn,
            2 => Commands::TurnOff,
            3 => Commands::GetStatus,
            _ => Commands::Unknown,
        }
    }
}
