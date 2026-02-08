use std::panic;
use std::process::Command;
use std::sync::{Arc, Mutex};

use sh_lib::{
    create_room,
    reporter::Report,
    rich_console::{TextColor, colored_println},
    smart_device::{
        SmartSocket, SmartThermometer,
        online::{ConnectionType, OnlineDevice},
    },
    smart_home::SmartHome,
};

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
                SmartThermometer::new_with_connection(
                    "Термометр 1.1",
                    24.0,
                    ConnectionType::udp("127.0.0.1".parse().unwrap(), 4001)
                ),
                SmartSocket::new_with_connection(
                    "Розетка 1.1",
                    1000.0,
                    true,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
                SmartSocket::new_with_connection(
                    "Розетка 1.2",
                    2000.0,
                    false,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
                SmartSocket::new_with_connection(
                    "Розетка 1.3",
                    1100.25,
                    true,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                )
            ),
            create_room!(
                "Кабинет",
                SmartThermometer::new_with_connection(
                    "Термометр 2.1",
                    20.0,
                    ConnectionType::udp("127.0.0.1".parse().unwrap(), 4002)
                ),
                SmartSocket::new_with_connection(
                    "Розетка 2.1",
                    1000.0,
                    true,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
                SmartSocket::new_with_connection(
                    "Розетка 2.2",
                    2000.0,
                    false,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
                SmartSocket::new_with_connection(
                    "Розетка 2.3",
                    1100.25,
                    true,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
            ),
        ],
    )
}

fn init_emulators() {
    let handlers = vec![
        Command::new("cargo")
            .env("SH_THERM_EMULATOR_TARGET_PORT", "4001")
            .arg("run")
            .arg("--bin")
            .arg("sh_therm_emulator")
            .spawn()
            .inspect(|c| {
                colored_println(
                    &format!("Эмулятор термометра 127.0.0.1:4001. pid {}", c.id()),
                    TextColor::Magenta,
                );
            })
            .expect(""),
        Command::new("cargo")
            .env("SH_THERM_EMULATOR_TARGET_PORT", "4002")
            .arg("run")
            .arg("--bin")
            .arg("sh_therm_emulator")
            .spawn()
            .inspect(|c| {
                colored_println(
                    &format!("Эмулятор термометра 127.0.0.1:4002. pid {}", c.id()),
                    TextColor::Magenta,
                );
            })
            .expect("Не удалось запустить эмулятор термометра 127.0.0.1:4002"),
        Command::new("cargo")
            .env("SH_SOCKET_EMULATOR_PORT", "3001")
            .arg("run")
            .arg("--bin")
            .arg("sh_socket_emulator")
            .spawn()
            .inspect(|c| {
                colored_println(
                    &format!("Эмулятор розетки 127.0.0.1:3001. pid {}", c.id()),
                    TextColor::Magenta,
                );
            })
            .expect("Не удалось запустить эмулятор розетки 127.0.0.1:3001"),
    ];

    let emulators = Arc::new(Mutex::new(handlers));
    let ref_em = emulators.clone();
    panic::set_hook(Box::new(move |info| {
        let mut l = ref_em.lock().unwrap();
        l.iter_mut().for_each(|c| match c.kill() {
            Ok(_) => {
                colored_println(
                    &format!("Эмулятор остановлен. pid {}", c.id()),
                    TextColor::Red,
                );
            }
            Err(e) => {
                colored_println(&e.to_string(), TextColor::Red);
            }
        });
        println!("{}", info);
    }));
}

#[tokio::main]
async fn main() {
    init_emulators();

    colored_println("Ждём запуска всех эмуляторов", TextColor::Magenta);
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

    let mut home = make_home();

    colored_println("Исходный отчет", TextColor::Green);
    print_status_report(&home).await;

    for room in home.get_rooms_mut().values_mut() {
        for device in room.get_devices_mut().values_mut() {
            if let Err(e) = device.connect().await {
                eprintln!("❌ Failed to connect: {}", e);
            }
        }
    }

    loop {
        colored_println("\nПолучаем отчет об умном доме", TextColor::Green);
        print_status_report(&home).await;

        tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    }
}
