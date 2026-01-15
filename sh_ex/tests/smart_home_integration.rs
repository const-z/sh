use sh_lib::create_room;
use sh_lib::smart_device::{SmartSocket, SmartThermometer};
use sh_lib::smart_home::SmartHome;
use sh_lib::smart_room::SmartRoom;

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
fn create_room_with_macros() {
    let room = create_room!(
        "Комната",
        SmartThermometer::new(String::from("Термометр"), 24.0),
        SmartSocket::new(String::from("Розетка"), 1000.0, true)
    );
    let home = SmartHome::new(String::from("Дом"), &[room]);
    let room_ref = home.get_room("Комната");
    assert!(room_ref.is_some());
    assert_eq!(room_ref.unwrap().get_devices().len(), 2);
}

#[test]
fn add_thermometer_to_room() {
    let mut room = SmartRoom::new(String::from("Комната"), &[]);
    room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
    assert_eq!(room.get_devices().len(), 1);
}

#[test]
fn add_socket_to_room() {
    let mut room = SmartRoom::new(String::from("Комната"), &[]);
    room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, true));
    assert_eq!(room.get_devices().len(), 1);
}
