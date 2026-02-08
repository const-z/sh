use crate::errors::SmartHomeErrors;
use crate::id::Id;
use crate::reporter::Report;
use crate::{smart_device::SmartDeviceType, smart_room::SmartRoom};
use std::collections::HashMap;
use std::fmt::Write;

/// Умный дом
#[derive(Debug)]
pub struct SmartHome {
    id: Id,
    name: String,
    rooms: HashMap<String, SmartRoom>,
}

impl SmartHome {
    /// Создать дом
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            id: Id::from_string(&name),
            name,
            rooms: HashMap::new(),
        }
    }

    /// Создать дом с комнатами
    pub fn new_with_rooms(name: impl Into<String>, rooms: &[SmartRoom]) -> Self {
        let name = name.into();
        Self {
            id: Id::from_string(&name),
            name,
            rooms: HashMap::from_iter(
                rooms
                    .iter()
                    .map(|room| (room.get_id().to_string(), room.clone())),
            ),
        }
    }

    pub fn get_id(&self) -> &Id {
        &self.id
    }

    /// Получить имя дома
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Получить ссылку на комнату в доме
    pub fn get_room(&self, id: &Id) -> Option<&SmartRoom> {
        self.rooms.get(&id.to_string())
    }

    /// Получить мутабельную ссылку на комнату в доме
    pub fn get_room_mut(&mut self, id: &Id) -> Option<&mut SmartRoom> {
        self.rooms.get_mut(&id.to_string())
    }

    /// Добавить комнату
    pub fn add_room(&mut self, room: SmartRoom) -> Id {
        let room_id = room.get_id().clone();
        self.rooms.insert(room_id.to_string(), room);
        room_id
    }

    /// Получить список комнат
    pub fn get_rooms(&self) -> &HashMap<String, SmartRoom> {
        &self.rooms
    }

    /// Получить список мутабельных ссылок на комнаты
    pub fn get_rooms_mut(&mut self) -> &mut HashMap<String, SmartRoom> {
        &mut self.rooms
    }

    /// Удалить комнату
    pub fn delete_room(&mut self, id: &Id) -> Option<SmartRoom> {
        self.rooms.remove(&id.to_string())
    }

    /// Получить устройство
    pub fn get_device(
        &self,
        room_id: &Id,
        device_id: &Id,
    ) -> Result<&SmartDeviceType, SmartHomeErrors> {
        match self.get_room(room_id) {
            Some(room) => match room.get_device(device_id) {
                Some(device) => Ok(device),
                None => Err(SmartHomeErrors::device_not_found(&device_id.to_string())),
            },
            None => Err(SmartHomeErrors::room_not_found(&room_id.to_string())),
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
        let home = SmartHome::new("Дом");
        assert_eq!(home.get_rooms().len(), 0);
    }

    #[test]
    fn add_room_to_smart_home() {
        let mut home = SmartHome::new("Дом");
        home.add_room(SmartRoom::new("Комната"));
        assert_eq!(home.get_rooms().len(), 1);
    }

    #[test]
    fn get_room_by_non_existent_key() {
        let home = SmartHome::new_with_rooms("Дом", &[SmartRoom::new("Комната")]);

        assert!(home.get_room(&Id::from_string("Другая комната")).is_none());
    }

    #[test]
    fn add_device_to_existed_room() {
        let mut home = SmartHome::new_with_rooms("Дом", &[SmartRoom::new("Комната")]);

        let room = home.get_room_mut(&Id::from_string("Комната")).unwrap();
        room.add_device(SmartThermometer::new("Термометр", 24.0));

        assert_eq!(room.get_devices().len(), 1);
    }

    #[test]
    fn get_device_by_non_existent_room_key() {
        let home = SmartHome::new_with_rooms("Дом", &[SmartRoom::new("Комната")]);

        let get_device_result = home.get_device(
            &Id::from_string("Другая комната"),
            &Id::from_string("Термометр"),
        );

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
        let home = SmartHome::new_with_rooms("Дом", &[SmartRoom::new("Комната")]);

        let get_device_result =
            home.get_device(&Id::from_string("Комната"), &Id::from_string("Термометр"));

        assert!(get_device_result.is_err());

        match get_device_result.unwrap_err() {
            SmartHomeErrors::DeviceNotFound(ErrorInfo { code, .. }) => {
                assert_eq!(code, "1002");
            }
            _ => panic!(),
        }
    }
}
