mod smart_socket;
mod smart_thermometer;

pub use smart_socket::SmartSocket;
pub use smart_thermometer::SmartThermometer;

use crate::Report;

/// Тип умного устройства
#[derive(Clone, Debug)]
pub enum SmartDeviceType {
    /// Умный термометр
    Thermometer(SmartThermometer),
    /// Умная розетка
    Socket(SmartSocket),
}

impl SmartDeviceType {
    pub fn get_name(&self) -> &String {
        match self {
            SmartDeviceType::Thermometer(t) => t.get_name(),
            SmartDeviceType::Socket(s) => s.get_name(),
        }
    }
}

/// Состояние устройства
#[derive(Copy, Clone, Debug)]
pub enum OnOff {
    /// Включено
    On,
    /// Выключено
    Off,
}

/// Умное устройство
pub trait SmartDevice {
    fn get_name(&self) -> &String;
}

impl SmartDevice for SmartDeviceType {
    fn get_name(&self) -> &String {
        match self {
            SmartDeviceType::Thermometer(t) => t.get_name(),
            SmartDeviceType::Socket(s) => s.get_name(),
        }
    }
}

impl Report for SmartDeviceType {
    fn get_status_report(&self) -> String {
        match self {
            SmartDeviceType::Thermometer(t) => t.get_status_report(),
            SmartDeviceType::Socket(s) => s.get_status_report(),
        }
    }
}
