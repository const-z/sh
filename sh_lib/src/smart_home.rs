use crate::errors::SmartHomeErrors;
use crate::reporter::Report;
use crate::{smart_device::SmartDeviceType, smart_room::SmartRoom};
use core::str;
use std::collections::HashMap;
use std::fmt::Write;

/// Умный дом
#[derive(Debug)]
pub struct SmartHome {
    name: String,
    rooms: HashMap<String, SmartRoom>,
}

impl SmartHome {
    /// Создать дом
    pub fn new(name: impl Into<String>, rooms: &[SmartRoom]) -> Self {
        Self {
            name: name.into(),
            rooms: HashMap::from_iter(
                rooms
                    .iter()
                    .map(|room| (room.get_name().clone(), room.clone())),
            ),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Получить ссылку на комнату в доме
    pub fn get_room(&self, name: &str) -> Option<&SmartRoom> {
        self.rooms.get(name)
    }

    /// Получить мутабельную ссылку на комнату в доме
    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut SmartRoom> {
        self.rooms.get_mut(name)
    }

    /// Добавить комнату
    pub fn add_room(&mut self, room: SmartRoom) {
        self.rooms.insert(room.get_name().clone(), room);
    }

    pub fn get_rooms(&self) -> Vec<&SmartRoom> {
        Vec::from_iter(self.rooms.values())
    }

    pub fn get_rooms_mut(&mut self) -> Vec<&mut SmartRoom> {
        Vec::from_iter(self.rooms.values_mut())
    }

    pub fn del_room(&mut self, name: &str) {
        self.rooms.remove(name);
    }

    pub fn get_device(
        &self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&SmartDeviceType, SmartHomeErrors> {
        match self.get_room(room_name) {
            Some(room) => match room.get_device(device_name) {
                Some(device) => Ok(device),
                None => Err(SmartHomeErrors::device_not_found(device_name)),
            },
            None => Err(SmartHomeErrors::room_not_found(room_name)),
        }
    }
}

impl Report for SmartHome {
    async fn get_status_report(&self) -> String {
        let name = self.name.clone();
        let rooms = self.rooms.values().clone();
        async move {
            let mut output = format!(r#"Отчет по дому "{}"{}"#, name, "\n");

            for room in rooms {
                write!(output, "{}", room.get_status_report().await).unwrap();
            }

            output
        }
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::ErrorInfo;
    use crate::smart_device::SmartThermometer;
    use crate::smart_room::SmartRoom;

    #[test]
    fn create_smart_home() {
        let home = SmartHome::new(String::from("Дом"), &[]);
        assert_eq!(home.get_rooms().len(), 0);
    }

    #[test]
    fn add_room_to_smart_home() {
        let mut home = SmartHome::new(String::from("Дом"), &[]);
        home.add_room(SmartRoom::new(String::from("Комната"), &[]));
        assert_eq!(home.get_rooms().len(), 1);
    }

    #[test]
    fn get_room_by_non_existent_key() {
        let home = SmartHome::new(
            String::from("Дом"),
            &[SmartRoom::new(String::from("Комната"), &[])],
        );

        assert!(home.get_room(&String::from("Другая комната")).is_none());
    }

    #[test]
    fn add_device_to_existed_room() {
        let mut home = SmartHome::new(
            String::from("Дом"),
            &[SmartRoom::new(String::from("Комната"), &[])],
        );

        let room = home.get_room_mut(&String::from("Комната")).unwrap();
        room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));

        assert_eq!(room.get_devices().len(), 1);
    }

    #[test]
    fn get_device_by_non_existent_room_key() {
        let home = SmartHome::new(
            String::from("Дом"),
            &[SmartRoom::new(String::from("Комната"), &[])],
        );

        let get_device_result = home.get_device("Другая комната", "Термометр");

        assert!(get_device_result.is_err());

        match get_device_result.unwrap_err() {
            SmartHomeErrors::RoomNotFound(ErrorInfo { code, .. }) => {
                assert_eq!(code, "1001");
            }
            _ => panic!(),
        }
    }

    #[test]
    fn get_device_by_non_existent_device_key() {
        let home = SmartHome::new(
            String::from("Дом"),
            &[SmartRoom::new(String::from("Комната"), &[])],
        );

        let get_device_result = home.get_device("Комната", "Термометр");

        assert!(get_device_result.is_err());

        match get_device_result.unwrap_err() {
            SmartHomeErrors::DeviceNotFound(ErrorInfo { code, .. }) => {
                assert_eq!(code, "1002");
            }
            _ => panic!(),
        }
    }
}
