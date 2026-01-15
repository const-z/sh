use std::{
    net::{IpAddr, SocketAddr, UdpSocket},
    sync::Arc,
};

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::Duration;

use crate::{
    errors::SmartHomeErrors,
    smart_device::{
        SmartDevice, SmartDeviceType,
        contracts::{Commands, DecodeEncode, DeviceResponse, DeviceResponseData},
    },
};

#[derive(Debug, Clone)]
pub enum ConnectionType {
    Tcp { ip: IpAddr, port: u16 },
    Udp { bind_ip: IpAddr, bind_port: u16 },
}

impl ConnectionType {
    pub fn tcp(ip: IpAddr, port: u16) -> Self {
        ConnectionType::Tcp { ip, port }
    }

    pub fn udp(bind_ip: IpAddr, bind_port: u16) -> Self {
        ConnectionType::Udp { bind_ip, bind_port }
    }

    pub fn get_addr(&self) -> SocketAddr {
        match self {
            ConnectionType::Tcp { ip, port, .. } => SocketAddr::new(*ip, *port),
            ConnectionType::Udp {
                bind_ip, bind_port, ..
            } => SocketAddr::new(*bind_ip, *bind_port),
        }
    }
}

fn decode_result(message: Vec<u8>) -> Result<Option<DeviceResponseData>, SmartHomeErrors> {
    let decode_result = DeviceResponse::decode(&message);

    if let Err(e) = decode_result {
        return Err(e);
    }

    let device_response = decode_result.unwrap();

    if !device_response.success
        && let Some(e) = device_response.error
    {
        return Err(SmartHomeErrors::emulator_error(e));
    }

    Ok(device_response.data)
}

async fn send_command(
    stream: &Arc<Mutex<TcpStream>>,
    cmd: Commands,
) -> Result<Option<DeviceResponseData>, anyhow::Error> {
    let mut stream = stream.lock().await;
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    let bytes = (cmd as i32).to_be_bytes();

    if let Err(e) = writer.write_all(&bytes).await {
        return Err(anyhow::anyhow!(e));
    }

    if let Err(e) = writer.flush().await {
        return Err(anyhow::anyhow!(e));
    }

    let mut message_length = [0u8; size_of::<usize>()];
    if let Err(e) = reader.read_exact(&mut message_length).await {
        return Err(anyhow::anyhow!(e));
    }

    let message_length = usize::from_be_bytes(message_length);

    let mut message = vec![0u8; message_length];
    if let Err(e) = reader.read_exact(&mut message).await {
        return Err(anyhow::anyhow!(e));
    }

    match decode_result(message) {
        Ok(device_response) => Ok(device_response),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

async fn start_tcp_monitoring<Fut, F>(stream: Arc<tokio::sync::Mutex<TcpStream>>, mut callback: F)
where
    F: FnMut(DeviceResponseData) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        loop {
            let device_response = send_command(&stream, Commands::GetStatus).await;

            if let Err(e) = device_response {
                eprintln!(
                    "{}",
                    SmartHomeErrors::getting_status_error(format!("TCP: {}", e))
                );
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            let device_response = device_response.unwrap();

            if device_response.is_none() {
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            callback(device_response.unwrap()).await;

            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
}

async fn start_udp_monitoring<Fut, F>(socket: Arc<Mutex<UdpSocket>>, mut callback: F)
where
    F: FnMut(DeviceResponseData) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        loop {
            let mut message_length = [0u8; size_of::<usize>()];

            let locked_socket = socket.lock().await;

            if let Err(e) = locked_socket.recv_from(&mut message_length) {
                eprintln!(
                    "{}",
                    SmartHomeErrors::getting_status_error(format!("UDP: {}", e))
                );
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            let message_length = usize::from_be_bytes(message_length);
            let mut message = vec![0u8; message_length + size_of::<usize>()];

            if let Err(e) = locked_socket.recv_from(&mut message) {
                eprintln!(
                    "{}",
                    SmartHomeErrors::getting_status_error(format!("UDP: {}", e))
                );
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            let device_response = decode_result(message[size_of::<usize>()..].to_vec());

            if let Err(e) = device_response {
                eprintln!("❌ UDP: Ошибка при декодировании сообщения: {}", e);
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            let device_response = device_response.unwrap();

            if device_response.is_none() {
                tokio::time::sleep(Duration::from_secs(2)).await;
                continue;
            }

            callback(device_response.unwrap()).await;
        }
    });
}

pub trait OnlineDevice {
    fn connect(&self) -> impl std::future::Future<Output = Result<(), String>> + Send;
}

impl OnlineDevice for SmartDeviceType {
    async fn connect(&self) -> Result<(), String> {
        let device_name = self.get_name().to_string();

        if self.get_connection().is_none() {
            return Err("Connection options is empty".to_string());
        }

        match &mut self.get_connection().unwrap() {
            ConnectionType::Tcp { ip, port, .. } => {
                let addr = SocketAddr::new(*ip, *port);
                match TcpStream::connect(&addr).await {
                    Ok(s) => {
                        match self {
                            SmartDeviceType::Socket(socket) => {
                                let value = Arc::clone(&socket.value);

                                start_tcp_monitoring(Arc::new(Mutex::new(s)), move |data| {
                                    let value = value.clone();
                                    async move {
                                        value.write().await.update(data);
                                    }
                                })
                                .await
                            }
                            _ => unimplemented!("Только для SmartSocket"),
                        }

                        Ok(())
                    }
                    Err(e) => Err(format!(
                        "{}: Ошибка подключения к {}: {}",
                        device_name, addr, e
                    )),
                }
            }
            ConnectionType::Udp {
                bind_ip, bind_port, ..
            } => {
                let addr = SocketAddr::new(*bind_ip, *bind_port);
                match UdpSocket::bind(addr) {
                    Ok(s) => {
                        match self {
                            SmartDeviceType::Thermometer(therm) => {
                                let value = Arc::clone(&therm.value);
                                start_udp_monitoring(Arc::new(Mutex::new(s)), move |data| {
                                    let value = value.clone();
                                    async move {
                                        value.write().await.update(data);
                                    }
                                })
                                .await
                            }
                            _ => unimplemented!("Только для SmartThermometer"),
                        }

                        Ok(())
                    }
                    Err(e) => Err(format!(
                        "{}: Failed to bind UDP socket {}: {}",
                        device_name, addr, e
                    )),
                }
            }
        }
    }
}
