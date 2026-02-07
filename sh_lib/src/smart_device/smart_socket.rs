use std::sync::Arc;

use bincode::{Decode, Encode};
use tokio::sync::RwLock;

use crate::{id::Id, reporter::Report, smart_device::online::ConnectionType};

use super::{SmartDevice, SmartDeviceType};

#[derive(Clone, Debug, Encode, Decode)]
pub struct SocketData {
    pub power: f32,
    pub is_on: bool,
    pub timestamp: u64,
    pub is_online: bool,
}

impl SocketData {
    pub fn new(power: f32, is_on: bool) -> Self {
        Self {
            power,
            is_on,
            is_online: false,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    pub fn update(&mut self, data: SocketData) {
        *self = data;
    }
}

#[derive(Clone, Debug)]
pub struct SmartSocket {
    pub id: Id,
    pub name: String,
    pub value: Arc<RwLock<SocketData>>,
    pub connection: Option<ConnectionType>,
}

impl SmartSocket {
    pub fn new(name: impl Into<String>, power: f32, is_on: bool) -> Self {
        let name = name.into();
        Self {
            id: Id::from_string(&name),
            name,
            value: Arc::new(RwLock::new(SocketData::new(power, is_on))),
            connection: None,
        }
    }

    pub fn new_with_connection(
        name: impl Into<String>,
        power: f32,
        is_on: bool,
        connection: ConnectionType,
    ) -> Self {
        let name = name.into();
        Self {
            id: Id::from_string(&name),
            name,
            value: Arc::new(RwLock::new(SocketData::new(power, is_on))),
            connection: Some(connection),
        }
    }

    /// Включить розетку
    pub async fn turn_on(&mut self) {
        let mut value = self.value.write().await;
        value.is_on = true;
        value.timestamp = chrono::Utc::now().timestamp_millis() as u64;
    }

    /// Выключить розетку
    pub async fn turn_off(&mut self) {
        let mut value = self.value.write().await;
        value.is_on = false;
        value.timestamp = chrono::Utc::now().timestamp_millis() as u64;
    }

    /// Проверить, включена ли розетка
    pub async fn is_on(&self) -> bool {
        self.value.read().await.is_on
    }

    /// Поучить данные розетки
    pub async fn get_data(&self) -> SocketData {
        let mut data = self.value.read().await.clone();
        data.power = if data.is_on { data.power } else { 0.0 };
        data
    }
}

impl SmartDevice for SmartSocket {
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

    use super::*;

    #[tokio::test]
    async fn socket_power_zero_if_off() {
        let socket = SmartSocket::new(String::from("Розетка"), 0.0, false);
        let socket_data = socket.get_data().await;
        assert_eq!(socket_data.power, 0.0);
    }

    #[tokio::test]
    async fn socket_power() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, true);
        let socket_data = socket.get_data().await;
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
