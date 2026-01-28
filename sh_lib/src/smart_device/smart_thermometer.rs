use std::sync::Arc;

use bincode::{Decode, Encode};
use tokio::sync::RwLock;

use crate::{
    reporter::Report,
    smart_device::{contracts::DeviceData, online::ConnectionType},
};

use super::{SmartDevice, SmartDeviceType};

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

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    pub name: String,
    pub value: Arc<RwLock<DeviceData>>,
    pub connection: Option<ConnectionType>,
}

impl SmartThermometer {
    pub fn new(name: impl Into<String>, temp: f32) -> Self {
        Self {
            name: name.into(),
            value: Arc::new(RwLock::new(DeviceData::Thermometer(ThermometerData::new(
                temp,
            )))),
            connection: None,
        }
    }

    pub fn new_with_connection(name: String, temp: f32, connection: ConnectionType) -> Self {
        Self {
            name,
            value: Arc::new(RwLock::new(DeviceData::Thermometer(ThermometerData::new(
                temp,
            )))),
            connection: Some(connection),
        }
    }

    pub async fn get_temp(&self) -> f32 {
        self.value.read().await.clone().as_thermometer().temp
    }

    pub async fn set_temp(&mut self, temp: f32) {
        let mut value = self.value.write().await;
        value.update(DeviceData::Thermometer(ThermometerData::new(temp)));
    }
}

impl SmartDevice for SmartThermometer {
    fn get_name(&self) -> &String {
        &self.name
    }

    async fn get_data(&self) -> DeviceData {
        self.value.read().await.clone()
    }

    fn get_connection(&self) -> Option<&ConnectionType> {
        self.connection.as_ref()
    }
}

impl Report for SmartThermometer {
    /// Получить статус термометра
    async fn get_status_report(&self) -> String {
        let value = self.value.read().await.as_thermometer();
        format!("{}: {} C°", self.name, value.temp)
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
