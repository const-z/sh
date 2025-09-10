// use crate::smart_device::{Device, SmartDevice};
use crate::smart_device::{SmartDevice, SmartDeviceType, SmartSocket, SmartThermometer};

pub struct SmartRoom {
    name: String,
    devices: Vec<SmartDeviceType>,
}

impl SmartRoom {
    /// Создать комнату
    pub fn new(name: String, devices: Vec<SmartDeviceType>) -> Self {
        Self { name, devices }
    }

    /// Получить ссылку на устройство по его индексу
    pub fn get_device(&self, index: usize) -> &SmartDeviceType {
        &self.devices[index]
    }

    /// Получить мутабельную ссылку на устройство по его индексу
    pub fn get_device_mut(&mut self, index: usize) -> &mut SmartDeviceType {
        &mut self.devices[index]
    }

    /// Получить ссылку на хранилище устройств в комнате
    pub fn get_devices(&self) -> &Vec<SmartDeviceType> {
        &self.devices
    }

    /// Получить мутабельную ссылку на хранилище устройств в комнате
    pub fn get_devices_mut(&mut self) -> &mut Vec<SmartDeviceType> {
        &mut self.devices
    }

    /// Вывести отчет о состоянии комнаты
    pub fn print_status_report(&self, indent_size: u8) {
        let indent = String::from_utf8(vec![b' '; indent_size as usize]).unwrap();
        println!(r#"{}== Отчет по комнате "{}" =="#, indent, self.name);

        for (i, device) in self.devices.iter().enumerate() {
            println!("{}{}{}: {}", indent, indent, i + 1, device.get_status());
        }
    }
}

pub trait AddDevice<T> {
    /// Добавить устройство в комнату
    fn add_device(&mut self, device: T);
}

impl AddDevice<SmartSocket> for SmartRoom {
    fn add_device(&mut self, device: SmartSocket) {
        self.devices.push(SmartDeviceType::Socket(device));
    }
}

impl AddDevice<SmartThermometer> for SmartRoom {
    fn add_device(&mut self, device: SmartThermometer) {
        self.devices.push(SmartDeviceType::Thermometer(device));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smart_device::OnOff;

    #[test]
    fn add_device() {
        let mut room = SmartRoom::new(String::from("Комната"), vec![]);
        room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
        room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On));

        assert_eq!(room.get_device(0).get_status(), "Термометр: 24 C°");
        assert_eq!(room.get_device(1).get_status(), "Розетка: Вкл, 1000 Вт");
    }

    #[test]
    fn print_status_report() {
        let mut room = SmartRoom::new(String::from("Комната"), vec![]);
        room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
        room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On));
        room.print_status_report(4);
    }

    #[test]
    fn get_mut_device() {
        let mut room = SmartRoom::new(String::from("Комната"), vec![]);
        room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
        room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On));

        let device = room.get_device_mut(0);

        match device {
            SmartDeviceType::Thermometer(thermometer) => {
                thermometer.set_temp(25.0);
            }
            _ => {}
        }

        assert_eq!(room.get_device(0).get_status(), "Термометр: 25 C°");
    }
}
