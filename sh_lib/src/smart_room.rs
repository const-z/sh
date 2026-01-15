use std::collections::HashMap;
use std::fmt::Write;

use crate::{
    Report,
    smart_device::{SmartDevice, SmartDeviceType},
};

#[derive(Clone, Debug)]
pub struct SmartRoom {
    name: String,
    devices: HashMap<String, SmartDeviceType>,
}

impl SmartRoom {
    /// Создать комнату
    pub fn new(name: String, devices: &[SmartDeviceType]) -> Self {
        Self {
            name,
            devices: HashMap::from_iter(devices.iter().map(|d| (d.get_name().clone(), d.clone()))),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Получить ссылку на устройство по его имени
    pub fn get_device(&self, name: &str) -> Option<&SmartDeviceType> {
        self.devices.get(name)
    }

    /// Получить мутабельную ссылку на устройство по его имени
    pub fn get_device_mut(&mut self, name: &str) -> Option<&mut SmartDeviceType> {
        self.devices.get_mut(name)
    }

    /// Получить массив ссылок на устройства в комнате
    pub fn get_devices(&self) -> Vec<&SmartDeviceType> {
        Vec::from_iter(self.devices.values())
    }

    /// Получить массив мутабельных ссылок на устройства в комнате
    pub fn get_devices_mut(&mut self) -> Vec<&mut SmartDeviceType> {
        Vec::from_iter(self.devices.values_mut())
    }

    pub fn add_device<T>(&mut self, device: T)
    where
        T: SmartDevice + Into<SmartDeviceType>,
    {
        self.devices
            .insert(String::from(device.get_name()), device.into());
    }

    /// Удалить устройство из комнаты
    pub fn del_device(&mut self, device_name: &str) {
        self.devices.remove(device_name);
    }
}

impl Report for SmartRoom {
    /// Вывести отчет о состоянии комнаты
    async fn get_status_report(&self) -> String {
        let mut output = format!(r#"Отчет по комнате "{}"{}"#, self.name, "\n");

        for (i, device) in self.devices.values().enumerate() {
            writeln!(output, "{}. {}", i + 1, device.get_status_report().await).unwrap();
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smart_device::{OnOff, SmartSocket, SmartThermometer};

    #[tokio::test]
    async fn add_device() {
        let mut room = SmartRoom::new(String::from("Комната"), &[]);
        room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
        room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On));

        assert_eq!(
            room.get_device("Термометр")
                .unwrap()
                .get_status_report()
                .await,
            "Термометр: 24 C°"
        );
        assert_eq!(
            room.get_device("Розетка")
                .unwrap()
                .get_status_report()
                .await,
            "Розетка: Вкл, 1000 Вт"
        );
    }

    #[tokio::test]
    async fn get_mut_device() {
        let mut room = SmartRoom::new(String::from("Комната"), &[]);
        room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
        room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On));

        let device = room.get_device_mut("Термометр").unwrap();

        if let SmartDeviceType::Thermometer(thermometer) = device {
            thermometer.set_temp(25.0).await;
        }

        assert_eq!(
            room.get_device("Термометр")
                .unwrap()
                .get_status_report()
                .await,
            "Термометр: 25 C°"
        );
    }
}
