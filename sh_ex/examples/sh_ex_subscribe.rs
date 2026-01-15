use sh_lib::{
    smart_device::{SmartSocket, SmartThermometer},
    smart_room::SmartRoom,
    subscriber::Subscribe,
};

struct MySubscriber {}

impl Subscribe for MySubscriber {
    fn on_event(&mut self, name: String) {
        println!("MySubscriber: Device added: {}", name);
    }
}

#[tokio::main]
async fn main() {
    let mut room = SmartRoom::new(String::from("Комната"), &[]);
    room.subscribe(|device_name| println!("Device added: {}", device_name));
    room.subscribe(MySubscriber {});

    room.add_device(SmartThermometer::new(String::from("Термометр"), 24.0));
    room.add_device(SmartSocket::new(String::from("Розетка"), 1000.0, true));
}
