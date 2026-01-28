pub mod healthcheck {
    tonic::include_proto!("healthcheck.v1");
}

pub mod smart_home_contracts {
    tonic::include_proto!("smart_home.v1");
}

mod repository;
mod store;

use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{env, panic};

use healthcheck::healthcheck_service_server::{HealthcheckService, HealthcheckServiceServer};
use healthcheck::{CheckRequest, CheckResponse};
use repository::Repository;
use sh_lib::rich_console::{TextColor, colored_println};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use tracing::info;

fn init_emulators() {
    let handlers = vec![
        Command::new("cargo")
            .env("SH_THERM_EMULATOR_TARGET_PORT", "4001")
            .arg("run")
            .arg("--bin")
            .arg("sh_therm_emulator")
            .spawn()
            .inspect(|c| {
                colored_println(
                    &format!("Эмулятор термометра 127.0.0.1:4001. pid {}", c.id()),
                    TextColor::Magenta,
                );
            })
            .expect(""),
        Command::new("cargo")
            .env("SH_THERM_EMULATOR_TARGET_PORT", "4002")
            .arg("run")
            .arg("--bin")
            .arg("sh_therm_emulator")
            .spawn()
            .inspect(|c| {
                colored_println(
                    &format!("Эмулятор термометра 127.0.0.1:4002. pid {}", c.id()),
                    TextColor::Magenta,
                );
            })
            .expect("Не удалось запустить эмулятор термометра 127.0.0.1:4002"),
        Command::new("cargo")
            .env("SH_SOCKET_EMULATOR_PORT", "3001")
            .arg("run")
            .arg("--bin")
            .arg("sh_socket_emulator")
            .spawn()
            .inspect(|c| {
                colored_println(
                    &format!("Эмулятор розетки 127.0.0.1:3001. pid {}", c.id()),
                    TextColor::Magenta,
                );
            })
            .expect("Не удалось запустить эмулятор розетки 127.0.0.1:3001"),
    ];

    let emulators = Arc::new(Mutex::new(handlers));
    let ref_em = emulators.clone();
    panic::set_hook(Box::new(move |info| {
        let mut l = ref_em.lock().unwrap();
        l.iter_mut().for_each(|c| match c.kill() {
            Ok(_) => {
                colored_println(
                    &format!("Эмулятор остановлен. pid {}", c.id()),
                    TextColor::Red,
                );
            }
            Err(e) => {
                colored_println(&e.to_string(), TextColor::Red);
            }
        });
        println!("{}", info);
    }));
}

#[tokio::main]
async fn main() {
    init_emulators();

    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let addr = env::var("API_SERVE_ADDR")
        .expect("API_SERVE_ADDR must be set")
        .parse()
        .expect("API_SERVE_ADDR must be a valid address, e.g. 0.0.0.0:50051");

    let health_checker = HealthChecker {};
    let smart_home = store::Store::new();

    info!("Server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(CorsLayer::permissive()) // для разработки
        .layer(GrpcWebLayer::new())
        .add_service(HealthcheckServiceServer::new(health_checker))
        .add_service(smart_home_contracts::home_service_server::HomeServiceServer::new(smart_home))
        .serve(addr)
        .await
        .unwrap();
}

pub struct HealthChecker {}

#[tonic::async_trait]
impl HealthcheckService for HealthChecker {
    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<CheckResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        Ok(CheckResponse {
            status: "OK".to_string(),
            error: "".to_string(),
        }
        .into())
    }
}

#[tonic::async_trait]
impl smart_home_contracts::home_service_server::HomeService for store::Store {
    async fn add_home(
        &self,
        request: Request<smart_home_contracts::AddHomeRequest>,
    ) -> Result<Response<smart_home_contracts::AddHomeResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        let home_id = match Repository::add_home(self, req.name).await {
            Ok(home_id) => home_id,
            Err(err) => return Err(err),
        };

        Ok(smart_home_contracts::AddHomeResponse {
            home_id: home_id.to_string(),
        }
        .into())
    }

    async fn delete_home(
        &self,
        request: Request<smart_home_contracts::DeleteHomeRequest>,
    ) -> Result<Response<smart_home_contracts::DeleteHomeResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        match Repository::delete_home(self, &req.home_id).await {
            Ok(_) => Ok(smart_home_contracts::DeleteHomeResponse {}.into()),
            Err(err) => Err(err),
        }
    }

    async fn list_homes(
        &self,
        request: Request<smart_home_contracts::ListHomesRequest>,
    ) -> Result<Response<smart_home_contracts::ListHomesResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        let homes = match Repository::list_homes(self).await {
            Ok(homes) => homes,
            Err(err) => return Err(err),
        };

        Ok(smart_home_contracts::ListHomesResponse { items: homes }.into())
    }

    async fn add_room(
        &self,
        request: Request<smart_home_contracts::AddRoomRequest>,
    ) -> Result<Response<smart_home_contracts::AddRoomResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        match Repository::add_room(self, &req.home_id, &req.name).await {
            Ok(room_id) => Ok(smart_home_contracts::AddRoomResponse {
                room_id: room_id.to_string(),
            }
            .into()),
            Err(err) => Err(err),
        }
    }

    async fn delete_room(
        &self,
        request: Request<smart_home_contracts::DeleteRoomRequest>,
    ) -> Result<Response<smart_home_contracts::DeleteRoomResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        match Repository::delete_room(self, &req.home_id, &req.room_id).await {
            Ok(_) => Ok(smart_home_contracts::DeleteRoomResponse {}.into()),
            Err(err) => Err(err),
        }
    }

    async fn list_rooms(
        &self,
        request: Request<smart_home_contracts::ListRoomsRequest>,
    ) -> Result<Response<smart_home_contracts::ListRoomsResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        let rooms = match Repository::list_rooms(self, &req.home_id).await {
            Ok(rooms) => rooms,
            Err(err) => return Err(err),
        };

        Ok(smart_home_contracts::ListRoomsResponse { items: rooms }.into())
    }

    async fn add_device(
        &self,
        request: Request<smart_home_contracts::AddDeviceRequest>,
    ) -> Result<Response<smart_home_contracts::AddDeviceResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        match Repository::add_device(
            self,
            &req.home_id,
            &req.room_id,
            req.device_type(),
            req.name,
            req.connection,
        )
        .await
        {
            Ok(device_id) => Ok(smart_home_contracts::AddDeviceResponse {
                device_id: device_id.to_string(),
            }
            .into()),
            Err(err) => Err(err),
        }
    }

    async fn delete_device(
        &self,
        request: Request<smart_home_contracts::DeleteDeviceRequest>,
    ) -> Result<Response<smart_home_contracts::DeleteDeviceResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        match Repository::delete_device(self, &req.home_id, &req.room_id, &req.device_id).await {
            Ok(_) => Ok(smart_home_contracts::DeleteDeviceResponse {}.into()),
            Err(err) => Err(err),
        }
    }

    async fn list_devices(
        &self,
        request: Request<smart_home_contracts::ListDevicesRequest>,
    ) -> Result<Response<smart_home_contracts::ListDevicesResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        let devices = match Repository::list_devices(self, &req.home_id, &req.room_id).await {
            Ok(devices) => devices,
            Err(err) => return Err(err),
        };

        Ok(smart_home_contracts::ListDevicesResponse { items: devices }.into())
    }

    async fn get_report(
        &self,
        request: Request<smart_home_contracts::GetReportRequest>,
    ) -> Result<Response<smart_home_contracts::GetReportResponse>, Status> {
        let req = request.into_inner();
        info!("Got a request: {req:?}");

        todo!()
    }
}
