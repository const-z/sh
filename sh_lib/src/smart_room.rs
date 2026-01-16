use core::fmt;
use std::fmt::Write;
use std::{collections::HashMap, vec};

use crate::{
    reporter::Report,
    smart_device::{SmartDevice, SmartDeviceType},
    subscriber::Subscribe,
};

pub struct SmartRoom {
    name: String,
    devices: HashMap<String, SmartDeviceType>,
    subscribers: Vec<Box<dyn Subscribe>>,
}

impl Subscribe for SmartRoom {
    fn on_event(&mut self, name: String) {
        for subscriber in self.subscribers.iter_mut() {
            subscriber.on_event(name.clone());
        }
    }
}

impl Clone for SmartRoom {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            devices: self.devices.clone(),
            subscribers: vec![],
        }
    }
}

impl fmt::Debug for SmartRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmartRoom")
            .field("name", &self.name)
            .field("devices", &self.devices)
            .finish()
    }
}

impl SmartRoom {
    /// Создать комнату
    pub fn new(name: impl Into<String>, devices: &[SmartDeviceType]) -> Self {
        Self {
            name: name.into(),
            devices: HashMap::from_iter(devices.iter().map(|d| (d.get_name().clone(), d.clone()))),
            subscribers: vec![],
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
        let device_name = device.get_name().clone();

        self.devices.insert(device_name.clone(), device.into());

        for subscriber in &mut self.subscribers {
            subscriber.on_event(device_name.clone());
        }
    }

    /// Удалить устройство из комнаты
    pub fn del_device(&mut self, device_name: &str) {
        self.devices.remove(device_name);
    }

    pub fn subscribe<S>(&mut self, subscriber: S)
    where
        S: Subscribe + 'static,
    {
        self.subscribers.push(Box::new(subscriber));
    }
}

impl Report for SmartRoom {
    /// Вывести отчет о состоянии комнаты
    async fn get_status_report(&self) -> String {
        let name = self.name.clone();
        let devices = self.devices.clone();
        async move {
            let mut output = format!(r#"Отчет по комнате "{}"{}"#, name, "\n");

            for (i, device) in devices.values().enumerate() {
                writeln!(output, "{}. {}", i + 1, device.get_status_report().await).unwrap();
            }

            output
        }
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smart_device::{SmartSocket, SmartThermometer};

    #[tokio::test]
    async fn add_device() {
        let mut room = SmartRoom::new(String::from("Комната"), &[]);
        room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
        room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, true));

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
        room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, true));

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
