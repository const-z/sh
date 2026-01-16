pub mod contracts;
pub mod online;
pub mod smart_socket;
pub mod smart_thermometer;

pub use smart_socket::SmartSocket;
pub use smart_thermometer::SmartThermometer;

use crate::{
    reporter::Report,
    smart_device::{contracts::DeviceData, online::ConnectionType},
};

/// Тип умного устройства
#[derive(Clone, Debug)]
pub enum SmartDeviceType {
    /// Умный термометр
    Thermometer(SmartThermometer),
    /// Умная розетка
    Socket(SmartSocket),
}

/// Умное устройство
pub trait SmartDevice {
    fn get_name(&self) -> &String;
    fn get_data(&self) -> impl Future<Output = DeviceData>;
    fn get_connection(&self) -> Option<&ConnectionType>;
}

impl SmartDevice for SmartDeviceType {
    fn get_name(&self) -> &String {
        match self {
            SmartDeviceType::Thermometer(t) => t.get_name(),
            SmartDeviceType::Socket(s) => s.get_name(),
        }
    }

    async fn get_data(&self) -> DeviceData {
        match self {
            SmartDeviceType::Socket(s) => s.get_data().await,
            SmartDeviceType::Thermometer(t) => t.get_data().await,
        }
    }

    fn get_connection(&self) -> Option<&ConnectionType> {
        match self {
            SmartDeviceType::Socket(s) => s.get_connection(),
            SmartDeviceType::Thermometer(t) => t.get_connection(),
        }
    }
}

impl Report for SmartDeviceType {
    async fn get_status_report(&self) -> String {
        match self {
            SmartDeviceType::Thermometer(t) => t.get_status_report().await,
            SmartDeviceType::Socket(s) => s.get_status_report().await,
        }
    }
}
