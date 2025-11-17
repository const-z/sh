use std::process::Stdio;

use sh_lib::{
    Report, create_room,
    smart_device::{
        OnOff, SmartSocket, SmartThermometer,
        online::{ConnectionType, OnlineDevice},
    },
    smart_home::SmartHome,
};
use tokio::process::Command;

/// Функция, которая принимает любой объект, умеющий выводить отчёт
pub async fn print_status_report(smart_object: &impl Report) {
    println!("{}", smart_object.get_status_report().await);
}

fn make_home() -> SmartHome {
    SmartHome::new(
        "Мой дом".to_string(),
        &[
            create_room!(
                "Кухня",
                SmartThermometer::new_with_connection(
                    String::from("Термометр 1.1"),
                    24.0,
                    ConnectionType::udp("127.0.0.1".parse().unwrap(), 4001)
                ),
                SmartSocket::new_with_connection(
                    String::from("Розетка 1.1"),
                    1000.0,
                    OnOff::On,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
                SmartSocket::new_with_connection(
                    String::from("Розетка 1.2"),
                    2000.0,
                    OnOff::Off,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
                SmartSocket::new_with_connection(
                    String::from("Розетка 1.3"),
                    1100.25,
                    OnOff::On,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                )
            ),
            create_room!(
                "Кабинет",
                SmartThermometer::new_with_connection(
                    String::from("Термометр 2.1"),
                    20.0,
                    ConnectionType::udp("127.0.0.1".parse().unwrap(), 4002)
                ),
                SmartSocket::new_with_connection(
                    String::from("Розетка 2.1"),
                    1000.0,
                    OnOff::On,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
                SmartSocket::new_with_connection(
                    String::from("Розетка 2.2"),
                    2000.0,
                    OnOff::Off,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
                SmartSocket::new_with_connection(
                    String::from("Розетка 2.3"),
                    1100.25,
                    OnOff::On,
                    ConnectionType::Tcp {
                        ip: "127.0.0.1".parse().unwrap(),
                        port: 3001,
                    },
                ),
            ),
        ],
    )
}

#[allow(dead_code)]
enum TextColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Reset,
}

impl TextColor {
    fn as_code(&self) -> &str {
        match self {
            TextColor::Red => "31m",
            TextColor::Green => "32m",
            TextColor::Yellow => "33m",
            TextColor::Blue => "34m",
            TextColor::Magenta => "35m",
            TextColor::Cyan => "36m",
            TextColor::Reset => "0m",
        }
    }
}

fn colored_println(text: &str, color: TextColor) {
    println!(
        "\x1b[{}{}\x1b[{}",
        color.as_code(),
        text,
        TextColor::Reset.as_code()
    );
}

#[tokio::main]
async fn main() {
    colored_println("Запускаем эмулятор термометра 1.1", TextColor::Magenta);
    Command::new("cargo")
        .env("SH_THERM_EMULATOR_TARGET_PORT", "4001")
        .arg("run")
        .arg("--bin")
        .arg("sh_therm_emulator")
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    colored_println("Запускаем эмулятор термометра 2.1", TextColor::Magenta);
    Command::new("cargo")
        .env("SH_THERM_EMULATOR_TARGET_PORT", "4002")
        .arg("run")
        .arg("--bin")
        .arg("sh_therm_emulator")
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    colored_println("Запускаем эмулятор розеток", TextColor::Magenta);
    Command::new("cargo")
        .env("SH_SOCKET_EMULATOR_PORT", "3001")
        .arg("run")
        .arg("--bin")
        .arg("sh_socket_emulator")
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    colored_println("Ждём запуска всех эмуляторов", TextColor::Magenta);
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;

    let mut home = make_home();

    colored_println("Исходный отчет", TextColor::Green);
    print_status_report(&home).await;

    for r in home.get_rooms_mut() {
        for d in r.get_devices_mut() {
            if let Err(e) = d.connect().await {
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
