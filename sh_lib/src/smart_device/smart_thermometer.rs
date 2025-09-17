use crate::Report;

use super::{SmartDevice, SmartDeviceType};

#[derive(Clone, Debug)]
pub struct SmartThermometer {
    name: String,
    temp: f32,
}

impl SmartThermometer {
    pub fn new(name: String, temp: f32) -> Self {
        Self { name, temp }
    }

    pub fn get_temp(&self) -> f32 {
        self.temp
    }

    pub fn set_temp(&mut self, temp: f32) {
        self.temp = temp;
    }
}

impl SmartDevice for SmartThermometer {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Report for SmartThermometer {
    /// Получить статус термометра
    fn get_status_report(&self) -> String {
        format!("{}: {} C°", self.name, self.temp)
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

    #[test]
    fn thermometer_get_temp() {
        let thermometer = SmartThermometer::new(String::from("Термометр"), 20.0);
        assert_eq!(thermometer.get_temp(), 20.0);
    }

    #[test]
    fn thermometer_get_status() {
        let thermometer = SmartThermometer::new(String::from("Термометр"), 20.0);
        assert_eq!(thermometer.get_status_report(), "Термометр: 20 C°");
    }

    #[test]
    fn thermometer_get_status_zero() {
        let thermometer = SmartThermometer::new(String::from("Термометр"), 0.0);
        assert_eq!(thermometer.get_status_report(), "Термометр: 0 C°");
    }

    #[test]
    fn thermometer_get_status_negative() {
        let thermometer = SmartThermometer::new(String::from("Термометр"), -10.0);
        assert_eq!(thermometer.get_status_report(), "Термометр: -10 C°");
    }
}
