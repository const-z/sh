use std::sync::Arc;

use bincode::{Decode, Encode};
use tokio::sync::RwLock;

use crate::{
    Report,
    smart_device::{contracts::DeviceResponseData, online::ConnectionType},
};

use super::{OnOff, SmartDevice, SmartDeviceType};

#[derive(Clone, Debug, Encode, Decode)]
pub struct SocketData {
    pub power: f32,
    pub is_on: bool,
    pub timestamp: i64,
}

impl SocketData {
    pub fn new(power: f32, is_on: bool) -> Self {
        Self {
            power,
            is_on,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SmartSocket {
    name: String,
    pub value: Arc<RwLock<SocketData>>,
    pub connection: Option<ConnectionType>,
}

impl SmartSocket {
    pub fn new(name: String, power: f32, is_on: OnOff) -> Self {
        Self {
            name,
            value: Arc::new(RwLock::new(SocketData::new(power, is_on == OnOff::On))),
            connection: None,
        }
    }

    pub fn new_with_connection(
        name: String,
        power: f32,
        is_on: OnOff,
        connection: ConnectionType,
    ) -> Self {
        Self {
            name,
            value: Arc::new(RwLock::new(SocketData::new(power, is_on == OnOff::On))),
            connection: Some(connection),
        }
    }

    /// Включить розетку
    pub async fn turn_on(&mut self) {
        let mut value = self.value.write().await;
        value.is_on = true;
        value.timestamp = chrono::Utc::now().timestamp_millis();
    }

    /// Выключить розетку
    pub async fn turn_off(&mut self) {
        let mut value = self.value.write().await;
        value.is_on = false;
        value.timestamp = chrono::Utc::now().timestamp_millis();
    }

    /// Проверить, включена ли розетка
    pub async fn is_on(&self) -> bool {
        self.value.read().await.is_on
    }
}

impl SmartDevice for SmartSocket {
    fn get_name(&self) -> &String {
        &self.name
    }

    async fn get_data(&self) -> DeviceResponseData {
        let value = self.value.read().await;
        DeviceResponseData::Socket(SocketData {
            power: value.power,
            is_on: value.is_on,
            timestamp: value.timestamp,
        })
    }

    /// Обновить данные розетки
    async fn update(&mut self, data: DeviceResponseData) {
        let data = match data {
            DeviceResponseData::Socket(s) => s,
            _ => panic!("Неверный тип устройства"),
        };

        let mut value = self.value.write().await;
        value.power = data.power;
        value.is_on = data.is_on;
        value.timestamp = data.timestamp;
    }

    fn get_connection(&self) -> Option<&ConnectionType> {
        self.connection.as_ref()
    }
}

impl Report for SmartSocket {
    /// Получить статус розетки
    async fn get_status_report(&self) -> String {
        let value = self.value.read().await;
        format!(
            "{}: {}",
            self.name,
            match value.is_on {
                true => format!("Вкл, {} Вт", value.power),
                false => "Выкл".to_string(),
            }
        )
    }
}

impl From<SmartSocket> for SmartDeviceType {
    fn from(value: SmartSocket) -> Self {
        SmartDeviceType::Socket(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::smart_device::contracts::DeviceResponseData;

    use super::*;

    #[tokio::test]
    async fn socket_power_zero_if_off() {
        let socket = SmartSocket::new(String::from("Розетка"), 0.0, OnOff::Off);
        let socket_data = match socket.get_data().await {
            DeviceResponseData::Socket(s) => s,
            _ => panic!("Неверный тип устройства"),
        };
        assert_eq!(socket_data.power, 0.0);
    }

    #[tokio::test]
    async fn socket_power() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On);
        let socket_data = match socket.get_data().await {
            DeviceResponseData::Socket(s) => s,
            _ => panic!("Неверный тип устройства"),
        };
        assert_eq!(socket_data.power, 1000.0);
    }

    #[tokio::test]
    async fn socket_status() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On);
        assert_eq!(socket.get_status_report().await, "Розетка: Вкл, 1000 Вт");
    }

    #[tokio::test]
    async fn socket_status_off() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::Off);
        assert_eq!(socket.get_status_report().await, "Розетка: Выкл");
    }

    #[tokio::test]
    async fn socket_turn_on() {
        let mut socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::Off);
        socket.turn_on().await;
        assert_eq!(socket.get_status_report().await, "Розетка: Вкл, 1000 Вт");
    }

    #[tokio::test]
    async fn socket_turn_off() {
        let mut socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On);
        socket.turn_off().await;
        assert_eq!(socket.get_status_report().await, "Розетка: Выкл");
    }

    #[tokio::test]
    async fn socket_is_on_true() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On);
        assert!(socket.is_on().await);
    }

    #[tokio::test]
    async fn socket_is_on_false() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::Off);
        assert!(!socket.is_on().await);
    }
}
