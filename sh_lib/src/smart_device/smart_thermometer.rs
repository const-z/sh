use std::sync::Arc;

use bincode::{Decode, Encode};
use tokio::sync::RwLock;

use crate::{id::Id, reporter::Report, smart_device::online::ConnectionType};

use super::{SmartDevice, SmartDeviceType};

#[derive(Clone, Debug, Encode, Decode)]
pub struct ThermometerData {
    pub temp: f32,
    pub timestamp: u64,
    pub is_online: bool,
}

impl ThermometerData {
    pub fn new(temp: f32) -> Self {
        Self {
            temp,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            is_online: false,
        }
    }

    pub fn update(&mut self, data: ThermometerData) {
        *self = data;
    }
}

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    pub id: Id,
    pub name: String,
    pub value: Arc<RwLock<ThermometerData>>,
    pub connection: Option<ConnectionType>,
}

impl SmartThermometer {
    pub fn new(name: impl Into<String>, temp: f32) -> Self {
        let name = name.into();
        Self {
            id: Id::from_string(&name),
            name,
            value: Arc::new(RwLock::new(ThermometerData::new(temp))),
            connection: None,
        }
    }

    pub fn new_with_connection(
        name: impl Into<String>,
        temp: f32,
        connection: ConnectionType,
    ) -> Self {
        let name = name.into();
        Self {
            id: Id::from_string(&name),
            name,
            value: Arc::new(RwLock::new(ThermometerData::new(temp))),
            connection: Some(connection),
        }
    }

    /// Получить данные термометра
    pub async fn get_data(&self) -> ThermometerData {
        self.value.read().await.clone()
    }
}

impl SmartDevice for SmartThermometer {
    fn get_id(&self) -> &Id {
        &self.id
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_connection(&self) -> Option<&ConnectionType> {
        self.connection.as_ref()
    }
}

impl Report for SmartThermometer {
    /// Получить статус термометра
    async fn get_status_report(&self) -> String {
        let value = self.value.read().await;
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
        assert_eq!(thermometer.value.read().await.temp, 20.0);
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
