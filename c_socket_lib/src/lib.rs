use std::{
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
};

use lazy_static::lazy_static;
use sh_lib::smart_device::SmartSocket;

// Не хочу подключать tokio
fn block_on<F: Future>(future: F) -> F::Output {
    let mut boxed_future = Box::pin(future);
    struct DummyWaker;
    impl Wake for DummyWaker {
        fn wake(self: Arc<Self>) {}
    }

    let waker = Waker::from(Arc::new(DummyWaker));
    let mut cx = Context::from_waker(&waker);

    loop {
        match boxed_future.as_mut().poll(&mut cx) {
            Poll::Ready(result) => return result,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<SmartSocket> =
        Mutex::new(SmartSocket::new(String::from("Розетка"), 1000.0, false));
}

#[repr(C)]
pub struct SocketState {
    pub power: f32,
    pub is_on: bool,
    pub timestamp: u64,
}

#[unsafe(no_mangle)]
pub extern "C" fn turn_off() -> SocketState {
    let mut state = STATE.lock().unwrap();
    block_on(state.turn_off());
    let r = block_on(state.get_data());
    SocketState {
        power: r.power,
        is_on: r.is_on,
        timestamp: r.timestamp,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn turn_on() -> SocketState {
    let mut state = STATE.lock().unwrap();
    block_on(state.turn_on());
    let r = block_on(state.get_data());
    SocketState {
        power: r.power,
        is_on: r.is_on,
        timestamp: r.timestamp,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_data() -> SocketState {
    let state = STATE.lock().unwrap();
    let r = block_on(state.get_data());
    SocketState {
        power: r.power,
        is_on: r.is_on,
        timestamp: r.timestamp,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn my_add(left: u64, right: u64) -> u64 {
    left + right
}
