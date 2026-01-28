use std::sync::Arc;

use bincode::{Decode, Encode};
use tokio::sync::RwLock;

use crate::{
    reporter::Report,
    smart_device::{contracts::DeviceData, online::ConnectionType},
};

use super::{SmartDevice, SmartDeviceType};

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
    pub name: String,
    pub value: Arc<RwLock<DeviceData>>,
    pub connection: Option<ConnectionType>,
}

impl SmartSocket {
    pub fn new(name: impl Into<String>, power: f32, is_on: bool) -> Self {
        Self {
            name: name.into(),
            value: Arc::new(RwLock::new(DeviceData::Socket(SocketData::new(
                power, is_on,
            )))),
            connection: None,
        }
    }

    pub fn new_with_connection(
        name: String,
        power: f32,
        is_on: bool,
        connection: ConnectionType,
    ) -> Self {
        Self {
            name,
            value: Arc::new(RwLock::new(DeviceData::Socket(SocketData::new(
                power, is_on,
            )))),
            connection: Some(connection),
        }
    }

    /// Включить розетку
    pub async fn turn_on(&mut self) {
        let mut value = self.value.write().await;
        let current_data = value.as_socket();
        value.update(DeviceData::Socket(SocketData::new(
            current_data.power,
            true,
        )));
    }

    /// Выключить розетку
    pub async fn turn_off(&mut self) {
        let mut value = self.value.write().await;
        let current_data = value.as_socket();
        value.update(DeviceData::Socket(SocketData::new(
            current_data.power,
            false,
        )));
    }

    /// Проверить, включена ли розетка
    pub async fn is_on(&self) -> bool {
        self.value.read().await.as_socket().is_on
    }
}

impl SmartDevice for SmartSocket {
    fn get_name(&self) -> &String {
        &self.name
    }

    async fn get_data(&self) -> DeviceData {
        let mut data = self.value.read().await.as_socket();
        data.power = if data.is_on { data.power } else { 0.0 };
        DeviceData::Socket(data)
    }

    fn get_connection(&self) -> Option<&ConnectionType> {
        self.connection.as_ref()
    }
}

impl Report for SmartSocket {
    /// Получить статус розетки
    async fn get_status_report(&self) -> String {
        let value = self.value.read().await.as_socket();
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
    use crate::smart_device::contracts::DeviceData;

    use super::*;

    #[tokio::test]
    async fn socket_power_zero_if_off() {
        let socket = SmartSocket::new(String::from("Розетка"), 0.0, false);
        let socket_data = match socket.get_data().await {
            DeviceData::Socket(s) => s,
            _ => panic!("Неверный тип устройства"),
        };
        assert_eq!(socket_data.power, 0.0);
    }

    #[tokio::test]
    async fn socket_power() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, true);
        let socket_data = match socket.get_data().await {
            DeviceData::Socket(s) => s,
            _ => panic!("Неверный тип устройства"),
        };
        assert_eq!(socket_data.power, 1000.0);
    }

    #[tokio::test]
    async fn socket_status() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, true);
        assert_eq!(socket.get_status_report().await, "Розетка: Вкл, 1000 Вт");
    }

    #[tokio::test]
    async fn socket_status_off() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, false);
        assert_eq!(socket.get_status_report().await, "Розетка: Выкл");
    }

    #[tokio::test]
    async fn socket_turn_on() {
        let mut socket = SmartSocket::new(String::from("Розетка"), 1000.0, false);
        socket.turn_on().await;
        assert_eq!(socket.get_status_report().await, "Розетка: Вкл, 1000 Вт");
    }

    #[tokio::test]
    async fn socket_turn_off() {
        let mut socket = SmartSocket::new(String::from("Розетка"), 1000.0, true);
        socket.turn_off().await;
        assert_eq!(socket.get_status_report().await, "Розетка: Выкл");
    }

    #[tokio::test]
    async fn socket_is_on_true() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, true);
        assert!(socket.is_on().await);
    }

    #[tokio::test]
    async fn socket_is_on_false() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, false);
        assert!(!socket.is_on().await);
    }
}
