use sh_lib::id::Id;
use sh_lib::smart_device::{SmartDeviceType, SmartSocket, SmartThermometer};
use sh_lib::smart_home::SmartHome;
use sh_lib::{create_room, reporter::Report};

/// Функция, которая принимает любой объект, умеющий выводить отчёт
pub async fn print_status_report(smart_object: &impl Report) {
    println!("{}", smart_object.get_status_report().await);
}

fn make_home() -> SmartHome {
    SmartHome::new_with_rooms(
        "Мой дом",
        &[
            create_room!(
                "Кухня",
                SmartThermometer::new("Термометр 1.1", 24.0),
                SmartSocket::new("Розетка 1.1", 1000.0, true),
                SmartSocket::new("Розетка 1.2", 2000.0, false),
                SmartSocket::new("Розетка 1.3", 1100.25, true)
            ),
            create_room!(
                "Кабинет",
                SmartThermometer::new("Термометр 2.1", 20.0),
                SmartSocket::new("Розетка 2.1", 1000.0, true),
                SmartSocket::new("Розетка 2.2", 2000.0, false),
                SmartSocket::new("Розетка 2.3", 1100.25, true)
            ),
        ],
    )
}

async fn turn_off_all_sockets(home: &mut SmartHome) {
    for room in home.get_rooms_mut().values_mut() {
        for device in room.get_devices_mut().values_mut() {
            if let SmartDeviceType::Socket(s) = device {
                s.turn_off().await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut home = make_home();
    print_status_report(&home).await;

    println!("\n\x1b[33mОтключаем все розетки\x1b[0m\n");
    turn_off_all_sockets(&mut home).await;
    print_status_report(&home).await;

    println!("\n\x1b[33mУдаляем из Кухни розетку 1.3\x1b[0m\n");
    home.get_room_mut(&Id::from_string("Кухня"))
        .unwrap()
        .delete_device(&Id::from_string("Розетка 1.3"));
    print_status_report(&home).await;

    println!("\n\x1b[33mУдаляем из дома Кабинет\x1b[0m\n");
    home.delete_room(&Id::from_string("Кабинет"));
    print_status_report(&home).await;

    println!("\n\x1b[33mПытаемся получить устройство в несуществующей комнате - Кабинет\x1b[0m\n");
    let room_not_found =
        home.get_device(&Id::from_string("Кабинет"), &Id::from_string("Розетка 2.1"));

    match room_not_found {
        Ok(d) => println!("Успешно получили устройство: Кабинет - {:?}", d),
        Err(e) => eprintln!("Ошибка при получении устройства: {e}"),
    }

    println!("\n\x1b[33mПытаемся получить несуществующее устройство в комнате - Кухня\x1b[0m\n");
    let device_not_found =
        home.get_device(&Id::from_string("Кухня"), &Id::from_string("Розетка 1.3"));

    match device_not_found {
        Ok(d) => println!("Успешно получили устройство: Кухня - {:?}", d),
        Err(e) => eprintln!("Ошибка при получении устройства: {e}"),
    }
}
