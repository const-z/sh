use std::{error::Error, thread::sleep, time::Duration};

#[repr(C)]
#[derive(Debug)]
struct SocketState {
    power: f32,
    is_on: bool,
    timestamp: u64,
}

type FnSocketInteraction = unsafe extern "C" fn() -> SocketState;

fn main() -> Result<(), Box<dyn Error>> {
    let library_result = unsafe { libloading::Library::new("target/debug/libc_socket_lib.so") };
    let library = library_result.inspect_err(|e| eprintln!("Failed to load library: {e}"))?;

    let get_data = unsafe { library.get::<FnSocketInteraction>("get_data") }
        .inspect_err(|e| eprintln!("Failed to get symbol: {e}"))?;
    let turn_on = unsafe { library.get::<FnSocketInteraction>("turn_on") }
        .inspect_err(|e| eprintln!("Failed to get symbol: {e}"))?;
    let turn_off = unsafe { library.get::<FnSocketInteraction>("turn_off") }
        .inspect_err(|e| eprintln!("Failed to get symbol: {e}"))?;

    println!("Socket state: {:#?}", unsafe { get_data() });
    sleep(Duration::from_millis(500));
    println!("Turn on. Socket state: {:#?}", unsafe { turn_on() });
    sleep(Duration::from_millis(500));
    println!("Turn off. Socket state: {:#?}", unsafe { turn_off() });

    Ok(())
}
