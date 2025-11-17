use std::sync::Arc;

use bincode::{Decode, Encode};
use tokio::sync::RwLock;

use crate::{
    Report,
    smart_device::{contracts::DeviceResponseData, online::ConnectionType},
};

use super::{SmartDevice, SmartDeviceType};

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    name: String,
    pub value: Arc<RwLock<ThermometerData>>,
    pub connection: Option<ConnectionType>,
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct ThermometerData {
    pub temp: f32,
    pub timestamp: i64,
}

impl ThermometerData {
    pub fn new(temp: f32) -> Self {
        Self {
            temp,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

impl SmartThermometer {
    pub fn new(name: String, temp: f32) -> Self {
        Self {
            name,
            value: Arc::new(RwLock::new(ThermometerData::new(temp))),
            connection: None,
        }
    }

    pub fn new_with_connection(name: String, temp: f32, connection: ConnectionType) -> Self {
        Self {
            name,
            value: Arc::new(RwLock::new(ThermometerData::new(temp))),
            connection: Some(connection),
        }
    }

    pub async fn get_temp(&self) -> f32 {
        self.value.read().await.temp
    }

    pub async fn set_temp(&mut self, temp: f32) {
        let mut value = self.value.write().await;
        value.temp = temp;
        value.timestamp = chrono::Utc::now().timestamp_millis();
    }
}

impl SmartDevice for SmartThermometer {
    fn get_name(&self) -> &String {
        &self.name
    }

    async fn get_data(&self) -> DeviceResponseData {
        let value = self.value.read().await;
        super::contracts::DeviceResponseData::Thermometer(ThermometerData {
            temp: value.temp,
            timestamp: value.timestamp,
        })
    }

    async fn update(&mut self, data: DeviceResponseData) {
        let data = match data {
            super::contracts::DeviceResponseData::Thermometer(t) => t,
            _ => panic!("Неверный тип устройства"),
        };

        let mut value = self.value.write().await;
        value.temp = data.temp;
        value.timestamp = data.timestamp;
    }

    fn get_connection(&self) -> Option<&ConnectionType> {
        self.connection.as_ref()
    }
}

impl Report for SmartThermometer {
    /// Получить статус термометра
    async fn get_status_report(&self) -> String {
        format!("{}: {} C°", self.name, self.value.read().await.temp)
    }
}

impl From<SmartThermometer> for SmartDeviceType {
    fn from(value: SmartThermometer) -> Self {
        SmartDeviceType::Thermometer(value)
    }
}

#[cfg(test)]
mod thermometer_tests {
    use super::*;

    #[tokio::test]
    async fn thermometer_get_temp() {
        let thermometer = SmartThermometer::new(String::from("Термометр"), 20.0);
        assert_eq!(thermometer.get_temp().await, 20.0);
    }

    #[tokio::test]
    async fn thermometer_get_status() {
        let thermometer = SmartThermometer::new(String::from("Термометр"), 20.0);
        assert_eq!(thermometer.get_status_report().await, "Термометр: 20 C°");
    }

    #[tokio::test]
    async fn thermometer_get_status_zero() {
        let thermometer = SmartThermometer::new(String::from("Термометр"), 0.0);
        assert_eq!(thermometer.get_status_report().await, "Термометр: 0 C°");
    }

    #[tokio::test]
    async fn thermometer_get_status_negative() {
        let thermometer = SmartThermometer::new(String::from("Термометр"), -10.0);
        assert_eq!(thermometer.get_status_report().await, "Термометр: -10 C°");
    }
}
