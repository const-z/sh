use std::env;

use dotenv::dotenv;
use tokio::net::UdpSocket;

use sh_lib::smart_device::contracts::{DecodeEncode, DeviceData, DeviceResponse};

#[tokio::main]
async fn main() {
    let pid = std::process::id();

    dotenv().ok();

    let target_ip = env::var("SH_THERM_EMULATOR_TARGET_IP").unwrap_or("127.0.0.1".to_string());
    let target_port = env::var("SH_THERM_EMULATOR_TARGET_PORT").unwrap_or("4001".to_string());

    let interval: u64 = env::var("SH_THERM_EMULATOR_INTERVAL_MS")
        .unwrap_or("2000".to_string())
        .parse()
        .unwrap();

    let thermometer =
        sh_lib::smart_device::smart_thermometer::SmartThermometer::new(pid.to_string(), 0.0);
    thermometer.value.write().await.is_online = true;

    let udp_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let target_addr = format!("{}:{}", target_ip, target_port);

    println!(
        "Термометр SN: {} будет писать статус в {}",
        pid, &target_addr
    );

    loop {
        // Для демонстрации смены состояния
        {
            thermometer.value.write().await.temp = (rand::random_range(1800..=2500) as f32) / 100.0;
            thermometer.value.write().await.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
        }

        let therm_data = DeviceResponse {
            data: Some(DeviceData::Thermometer(thermometer.get_data().await)),
            success: true,
            error: None,
        };

        tokio::time::sleep(tokio::time::Duration::from_millis(interval)).await;

        match therm_data.encode() {
            Err(e) => {
                eprintln!("❌ Failed to encode device response: {}", e);
                continue;
            }
            Ok(encoded) => {
                let size_bytes = encoded.len().to_be_bytes().to_vec();
                let d = [size_bytes, encoded].concat();

                if let Err(e) = udp_socket.send_to(&d, &target_addr).await {
                    eprintln!("❌ Failed to send device response: {}", e);
                    continue;
                }

                println!("Термометр SN: {} отправил данные: {:?}", pid, therm_data);
            }
        }
    }
}
