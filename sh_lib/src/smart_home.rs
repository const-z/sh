use crate::smart_room::SmartRoom;

/// Умный дом
pub struct SmartHome {
    name: String,
    rooms: Vec<SmartRoom>,
}

impl SmartHome {
    /// Создать дом
    pub fn new(name: String, rooms: Vec<SmartRoom>) -> Self {
        Self { name, rooms }
    }

    /// Получить ссылку на комнату в доме
    pub fn get_room(&self, index: usize) -> &SmartRoom {
        &self.rooms[index]
    }

    /// Получить мутабельную ссылку на комнату в доме
    pub fn get_room_mut(&mut self, index: usize) -> &mut SmartRoom {
        &mut self.rooms[index]
    }

    /// Добавить комнату
    pub fn add_room(&mut self, room: SmartRoom) {
        self.rooms.push(room);
    }

    pub fn get_rooms(&self) -> &Vec<SmartRoom> {
        &self.rooms
    }

    pub fn get_rooms_mut(&mut self) -> &mut Vec<SmartRoom> {
        &mut self.rooms
    }

    /// Вывести отчет о состоянии дома
    pub fn print_status_report(&self) {
        println!(r#"== Отчет по дому "{}" =="#, self.name);

        for room in self.rooms.iter() {
            room.print_status_report(4);
        }

        println!(r#"== Конец отчета по дому "{}" =="#, self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smart_device::SmartThermometer;
    use crate::smart_room::{AddDevice, SmartRoom};

    #[test]
    fn create_smart_home() {
        let home = SmartHome::new(String::from("Дом"), Vec::new());
        assert_eq!(home.get_rooms().len(), 0);
    }

    #[test]
    fn add_room_to_smart_home() {
        let mut home = SmartHome::new(String::from("Дом"), Vec::new());
        home.add_room(SmartRoom::new(String::from("Комната"), Vec::new()));
        assert_eq!(home.get_rooms().len(), 1);
    }

    #[test]
    #[should_panic]
    fn get_room_by_non_existent_index() {
        let home = SmartHome::new(
            String::from("Дом"),
            vec![SmartRoom::new(String::from("Комната"), Vec::new())],
        );

        home.get_room(10);
    }

    #[test]
    fn add_device_to_existed_room() {
        let mut home = SmartHome::new(
            String::from("Дом"),
            vec![SmartRoom::new(String::from("Комната"), Vec::new())],
        );

        let room = home.get_room_mut(0);
        room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));

        assert_eq!(room.get_devices().len(), 1);
    }
}
