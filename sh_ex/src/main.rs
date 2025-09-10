use sh_lib::smart_device::{OnOff, SmartDeviceType, SmartSocket, SmartThermometer};
use sh_lib::smart_home::SmartHome;
use sh_lib::smart_room::{AddDevice, SmartRoom};

fn make_home() -> SmartHome {
    let mut home = SmartHome::new("Мой дом".to_string(), vec![]);

    home.add_room(SmartRoom::new("Кухня".to_string(), vec![]));
    home.add_room(SmartRoom::new("Кабинет".to_string(), vec![]));

    home.get_room_mut(0)
        .add_device(SmartThermometer::new(String::from("Термометр 1.1"), 24.0));
    home.get_room_mut(0).add_device(SmartSocket::new(
        String::from("Розетка 1.1"),
        1000.0,
        OnOff::On,
    ));
    home.get_room_mut(0).add_device(SmartSocket::new(
        String::from("Розетка 1.2"),
        2000.0,
        OnOff::Off,
    ));
    home.get_room_mut(0).add_device(SmartSocket::new(
        String::from("Розетка 1.3"),
        1100.25,
        OnOff::On,
    ));

    home.get_room_mut(1)
        .add_device(SmartThermometer::new(String::from("Термометр 2.1"), 20.0));
    home.get_room_mut(1)
        .add_device(SmartThermometer::new(String::from("Термометр 2.2"), 22.0));
    home.get_room_mut(1).add_device(SmartSocket::new(
        String::from("Розетка 2.1"),
        600.0,
        OnOff::Off,
    ));
    home.get_room_mut(1).add_device(SmartSocket::new(
        String::from("Розетка 2.2"),
        400.5,
        OnOff::On,
    ));

    home
}

fn turn_off_all_sockets(home: &mut SmartHome) {
    for r in home.get_rooms_mut() {
        for d in r.get_devices_mut() {
            if let SmartDeviceType::Socket(s) = d {
                s.turn_off();
            }
        }
    }
}

fn main() {
    let mut home = make_home();
    home.print_status_report();
    println!("\n\x1b[33mОтключаем все розетки\x1b[0m\n");
    turn_off_all_sockets(&mut home);
    home.print_status_report();
}
