use sh_lib::{
    reporter::{Report, Reporter},
    smart_device::{SmartSocket, SmartThermometer},
    smart_room::SmartRoom,
};

#[tokio::main]
async fn main() {
    let socket1 = SmartSocket::new("Socket 1", 400.0, true);
    let socket2 = SmartSocket::new("Socket 2", 1000.0, true);
    let thermo1 = SmartThermometer::new("Thermo 1", 20.0);
    let thermo2 = SmartThermometer::new("Thermo 2", 25.0);
    let room = SmartRoom::new(
        "Room 1",
        &[
            socket1.clone().into(),
            socket2.clone().into(),
            thermo1.clone().into(),
            thermo2.clone().into(),
        ],
    );

    let report = Reporter::new()
        .add(&room)
        .add(&socket1)
        .add(&socket2)
        .add(&thermo1)
        .add(&thermo2)
        .get_status_report()
        .await;

    println!("{}", report);
}
