use sh_lib::smart_device::{OnOff, SmartSocket, SmartThermometer};
use sh_lib::smart_home::SmartHome;
use sh_lib::smart_room::{AddDevice, SmartRoom};

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
fn add_thermometer_to_room() {
    let mut room = SmartRoom::new(String::from("Комната"), Vec::new());
    room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
    assert_eq!(room.get_devices().len(), 1);
}

#[test]
fn add_socket_to_room() {
    let mut room = SmartRoom::new(String::from("Комната"), Vec::new());
    room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On));
    assert_eq!(room.get_devices().len(), 1);
}
