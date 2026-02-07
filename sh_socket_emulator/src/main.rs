use dotenv::dotenv;
use sh_lib::smart_device::SmartSocket;
use sh_lib::smart_device::contracts::{Commands, DecodeEncode, DeviceData, DeviceResponse};
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let pid = std::process::id();

    let port = env::var("SH_SOCKET_EMULATOR_PORT").unwrap_or("3001".to_string());

    let socket_arc = Arc::new(RwLock::new(SmartSocket::new(
        format!("Розетка SN: {}", pid),
        0.0,
        false,
    )));

    socket_arc.write().await.value.write().await.is_online = true;

    let listen_addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&listen_addr).await?;

    println!(
        "Розетка SN: {} слушает подключение на {}",
        pid, &listen_addr
    );

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("Розетка SN: {} приняла подключение от {}", pid, addr);
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
    let pid = std::process::id();
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    loop {
        let mut buf = [0u8; 4];

        if let Err(e) = reader.read_exact(&mut buf).await {
            println!(
                "Розетка SN: {} потеряла соединение с {}. Err: {}",
                pid, addr, e
            );
            break;
        }

        let result = match Commands::from(i32::from_be_bytes(buf)) {
            Commands::TurnOn => {
                println!("Розетка SN: {} получила команду TurnOn", pid);
                let mut socket = socket.write().await;
                turn_on(&mut socket).await
            }
            Commands::TurnOff => {
                println!("Розетка SN: {} получила команду TurnOff", pid);
                let mut socket = socket.write().await;
                turn_off(&mut socket).await
            }
            Commands::GetStatus => {
                println!("Розетка SN: {} получила команду GetStatus", pid);

                // Для демонстрации смены состояния
                {
                    let socket = socket.write().await;
                    let mut socket_value = socket.value.write().await;
                    socket_value.is_on = !socket_value.is_on;
                    socket_value.power = (rand::random_range(70000..=200000) as f32) / 100.0;
                    socket_value.timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                }

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
        println!("Розетка SN: {} отправила ответ: {:?}", pid, result);

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
        data: Some(DeviceData::Socket(socket.get_data().await)),
    }
}
