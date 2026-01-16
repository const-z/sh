use sh_lib::{
    builder::HomeBuilder,
    reporter::Report,
    smart_device::{SmartDeviceType, SmartSocket},
    smart_home::SmartHome,
};

#[tokio::main]
async fn main() {
    let smart_home: SmartHome = HomeBuilder::new()
        .add_room("Room 1".to_string())
        .add_device(SmartDeviceType::Socket(SmartSocket::new(
            "Socket 1".to_string(),
            1000.0,
            true,
        )))
        .add_room("Room 2".to_string())
        .add_device(SmartDeviceType::Socket(SmartSocket::new(
            "Socket 2".to_string(),
            1000.0,
            true,
        )))
        .add_room("Room 3".to_string())
        .add_device(SmartDeviceType::Socket(SmartSocket::new(
            "Socket 3".to_string(),
            1000.0,
            true,
        )))
        .add_device(SmartDeviceType::Socket(SmartSocket::new(
            "Socket 4".to_string(),
            1000.0,
            true,
        )))
        .build();

    println!("Отчет:\n{}", smart_home.get_status_report().await);
}
