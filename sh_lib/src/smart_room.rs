use core::fmt;
use std::fmt::Write;
use std::{collections::HashMap, vec};

use crate::id::Id;
use crate::{
    reporter::Report,
    smart_device::{SmartDevice, SmartDeviceType},
    subscriber::Subscribe,
};

pub struct SmartRoom {
    id: Id,
    name: String,
    devices: HashMap<String, SmartDeviceType>,
    subscribers: Vec<Box<dyn Subscribe + Send + Sync>>,
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
            id: self.id.clone(),
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
    /// Создать пустую комнату
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            id: Id::from_string(&name),
            name,
            devices: HashMap::new(),
            subscribers: Vec::new(),
        }
    }

    /// Создать комнату с устройствами
    pub fn new_with_devices(name: impl Into<String>, devices: &[SmartDeviceType]) -> Self {
        let name = name.into();
        Self {
            id: Id::from_string(&name),
            name,
            devices: HashMap::from_iter(
                devices.iter().map(|d| (d.get_id().to_string(), d.clone())),
            ),
            subscribers: Vec::new(),
        }
    }

    /// Получить id комнаты
    pub fn get_id(&self) -> &Id {
        &self.id
    }

    /// Получить имя комнаты
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Получить ссылку на устройство по его имени
    pub fn get_device(&self, id: &Id) -> Option<&SmartDeviceType> {
        self.devices.get(&id.to_string())
    }

    /// Получить мутабельную ссылку на устройство по его имени
    pub fn get_device_mut(&mut self, id: &Id) -> Option<&mut SmartDeviceType> {
        self.devices.get_mut(&id.to_string())
    }

    /// Получить массив ссылок на устройства в комнате
    pub fn get_devices(&self) -> &HashMap<String, SmartDeviceType> {
        &self.devices
    }

    /// Получить массив мутабельных ссылок на устройства в комнате
    pub fn get_devices_mut(&mut self) -> &mut HashMap<String, SmartDeviceType> {
        &mut self.devices
    }

    pub fn add_device<T>(&mut self, device: T) -> Id
    where
        T: SmartDevice + Into<SmartDeviceType>,
    {
        let device_name = device.get_name().clone();
        let id = device.get_id().clone();

        self.devices.insert(id.to_string(), device.into());

        for subscriber in &mut self.subscribers {
            subscriber.on_event(device_name.clone());
        }

        id
    }

    /// Удалить устройство из комнаты
    pub fn delete_device(&mut self, id: &Id) -> Option<SmartDeviceType> {
        self.devices.remove(&id.to_string())
    }

    /// Подписаться на уведомления
    pub fn subscribe<S>(&mut self, subscriber: S)
    where
        S: Subscribe + Send + Sync + 'static,
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
        let mut room = SmartRoom::new("Комната");
        let id_therm = room.add_device(SmartThermometer::new("Термометр", 24.0));
        let id_socket = room.add_device(SmartSocket::new("Розетка", 1000.0, true));

        assert_eq!(
            room.get_device(&id_therm)
                .unwrap()
                .get_status_report()
                .await,
            "Термометр: 24 C°"
        );
        assert_eq!(
            room.get_device(&id_socket)
                .unwrap()
                .get_status_report()
                .await,
            "Розетка: Вкл, 1000 Вт"
        );
    }

    #[tokio::test]
    async fn get_mut_device() {
        let mut room = SmartRoom::new("Комната");
        let id_therm = room.add_device(SmartThermometer::new("Термометр", 24.0));
        room.add_device(SmartSocket::new("Розетка", 1000.0, true));

        let device = room.get_device_mut(&id_therm).unwrap();

        if let SmartDeviceType::Thermometer(thermometer) = device {
            let mut td = thermometer.value.write().await;
            td.temp = 25.0;
        }

        assert_eq!(
            room.get_device(&id_therm)
                .unwrap()
                .get_status_report()
                .await,
            "Термометр: 25 C°"
        );
    }
}
