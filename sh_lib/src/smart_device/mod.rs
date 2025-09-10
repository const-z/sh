mod smart_socket;
mod smart_thermometer;

pub use smart_socket::SmartSocket;
pub use smart_thermometer::SmartThermometer;

/// Тип умного устройства
pub enum SmartDeviceType {
    /// Умный термометр
    Thermometer(SmartThermometer),
    /// Умная розетка
    Socket(SmartSocket),
}

/// Состояние устройства
#[derive(Copy, Clone)]
pub enum OnOff {
    /// Включено
    On,
    /// Выключено
    Off,
}

/// Умное устройство
pub trait SmartDevice {
    fn get_status(&self) -> String;
}

impl SmartDevice for SmartDeviceType {
    fn get_status(&self) -> String {
        match self {
            SmartDeviceType::Thermometer(t) => t.get_status(),
            SmartDeviceType::Socket(s) => s.get_status(),
        }
    }
}
