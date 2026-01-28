use std::{thread::sleep, time::Duration};

#[repr(C)]
#[derive(Debug)]
struct SocketState {
    power: f32,
    is_on: bool,
    timestamp: u64,
}

unsafe extern "C" {
    fn turn_on() -> SocketState;
    fn turn_off() -> SocketState;
    fn get_data() -> SocketState;
}

fn main() {
    println!("Socket state: {:#?}", unsafe { get_data() });
    sleep(Duration::from_millis(500));
    println!("Turn on. Socket state: {:#?}", unsafe { turn_on() });
    sleep(Duration::from_millis(500));
    println!("Turn off. Socket state: {:#?}", unsafe { turn_off() });
}
