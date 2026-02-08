use sh_lib::{
    reporter::{Report, Reporter},
    smart_device::{SmartSocket, SmartThermometer},
    smart_home::SmartHome,
    smart_room::SmartRoom,
};

#[tokio::main]
async fn main() {
    let socket1 = SmartSocket::new("Socket 1", 400.0, true);
    let socket2 = SmartSocket::new("Socket 2", 1000.0, true);
    let thermo1 = SmartThermometer::new("Thermo 1", 20.0);
    let thermo2 = SmartThermometer::new("Thermo 2", 25.0);
    let room1 =
        SmartRoom::new_with_devices("Room 1", &[socket1.clone().into(), thermo1.clone().into()]);
    let room2 =
        SmartRoom::new_with_devices("Room 2", &[socket2.clone().into(), thermo2.clone().into()]);
    let home = SmartHome::new_with_rooms("Home", &[room1.clone(), room2.clone()]);

    let report = Reporter::new()
        .add_item(&home)
        .add_item(&room1)
        .add_item(&room2)
        .add_item(&socket1)
        .add_item(&socket2)
        .add_item(&thermo1)
        .add_item(&thermo2)
        .get_status_report()
        .await;

    println!("{}", report);
}
