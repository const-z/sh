use dotenv::dotenv;
use sh_lib::smart_device::contracts::{Commands, DecodeEncode, DeviceResponse};
use sh_lib::smart_device::{SmartDevice, SmartSocket};
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

const DEVICE_ID: &str = "c29ja2V0IGVtdWxhdG9yIDE";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let port = env::var("SH_SOCKET_EMULATOR_PORT").unwrap_or("3001".to_string());

    let socket_arc = Arc::new(RwLock::new(SmartSocket::new(
        format!("Розетка SN: {}", DEVICE_ID),
        0.0,
        false,
    )));

    let listen_addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&listen_addr).await?;

    println!(
        "Розетка SN: {} слушает подключение на {}",
        DEVICE_ID, &listen_addr
    );

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("Розетка SN: {} приняла подключение от {}", DEVICE_ID, addr);
        let socket_arc = socket_arc.clone();

        tokio::spawn(async move {
            handle_connection(&mut stream, &socket_arc, addr).await;
        });
    }
}

async fn handle_connection(
    stream: &mut TcpStream,
    socket: &Arc<RwLock<SmartSocket>>,
    addr: std::net::SocketAddr,
) {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    loop {
        let mut buf = [0u8; 4];

        if let Err(e) = reader.read_exact(&mut buf).await {
            println!(
                "Розетка SN: {} потеряла соединение с {}. Err: {}",
                DEVICE_ID, addr, e
            );
            break;
        }

        let result = match Commands::from(i32::from_be_bytes(buf)) {
            Commands::TurnOn => {
                println!("Розетка SN: {} получила команду TurnOn", DEVICE_ID);
                let mut socket = socket.write().await;
                turn_on(&mut socket).await
            }
            Commands::TurnOff => {
                println!("Розетка SN: {} получила команду TurnOff", DEVICE_ID);
                let mut socket = socket.write().await;
                turn_off(&mut socket).await
            }
            Commands::GetStatus => {
                println!("Розетка SN: {} получила команду GetStatus", DEVICE_ID);
                let socket = socket.read().await;
                get_socket_data(&socket).await
            }
            Commands::Unknown => DeviceResponse {
                success: false,
                error: Some(String::from("Unknown command")),
                data: None,
            },
        };

        let encoded: Vec<u8> = result.encode().unwrap();
        println!("Розетка SN: {} отправила ответ: {:#?}", DEVICE_ID, result);

        let size_bytes = encoded.len().to_be_bytes().to_vec();
        let d = [size_bytes, encoded].concat();

        writer.write_all(&d).await.unwrap();
        writer.flush().await.unwrap();
    }
}

async fn turn_on(socket: &mut SmartSocket) -> DeviceResponse {
    socket.turn_on().await;

    DeviceResponse {
        success: true,
        error: None,
        data: None,
    }
}

async fn turn_off(socket: &mut SmartSocket) -> DeviceResponse {
    socket.turn_off().await;

    DeviceResponse {
        success: true,
        error: None,
        data: None,
    }
}

async fn get_socket_data(socket: &SmartSocket) -> DeviceResponse {
    DeviceResponse {
        success: true,
        error: None,
        data: Some(socket.get_data().await),
    }
}
