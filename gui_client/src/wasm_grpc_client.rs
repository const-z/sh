pub mod smart_home_contracts {
    tonic::include_proto!("smart_home.v1");
}

use smart_home_contracts::{
    AddDeviceRequest, AddHomeRequest, AddRoomRequest, ConnectionSettings, DeleteHomeRequest,
    DeleteRoomRequest, ListDevicesRequest, ListHomesRequest, ListRoomsRequest,
    home_service_client::HomeServiceClient,
};
use tonic::Status;
use tonic_web_wasm_client::{Client, options::FetchOptions};

use crate::wasm_grpc_client::smart_home_contracts::{DeleteDeviceRequest, Item};

const SH_GRPS_SERVER: &str = "http://127.0.0.1:50051";

pub async fn get_homes() -> Result<Vec<(String, String)>, Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(ListHomesRequest {});
    let response = match home_service.list_homes(request).await {
        Ok(response) => response,
        Err(e) => {
            return Err(e);
        }
    };

    let r: Vec<(String, String)> = response
        .into_inner()
        .items
        .iter()
        .map(|item| (item.id.clone(), item.name.clone()))
        .collect();

    Ok(r)
}

pub async fn add_home(home_name: String) -> Result<(), Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(AddHomeRequest { name: home_name });

    match home_service.add_home(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn delete_home(home_id: String) -> Result<(), Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(DeleteHomeRequest { home_id });

    match home_service.delete_home(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn get_rooms(home_id: String) -> Result<Vec<(String, String)>, Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(ListRoomsRequest { home_id });
    let response = match home_service.list_rooms(request).await {
        Ok(response) => response,
        Err(e) => {
            return Err(e);
        }
    };

    let r: Vec<(String, String)> = response
        .into_inner()
        .items
        .iter()
        .map(|item| (item.id.clone(), item.name.clone()))
        .collect();

    Ok(r)
}

pub async fn add_room(home_id: String, home_name: String) -> Result<(), Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(AddRoomRequest {
        home_id,
        name: home_name,
    });

    match home_service.add_room(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn delete_room(home_id: String, room_id: String) -> Result<(), Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(DeleteRoomRequest { home_id, room_id });

    match home_service.delete_room(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn get_devices(home_id: String, room_id: String) -> Result<Vec<Item>, Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(ListDevicesRequest { home_id, room_id });
    let response = match home_service.list_devices(request).await {
        Ok(response) => response,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(response.into_inner().items)
}

pub async fn add_device(
    home_id: String,
    room_id: String,
    device_type: i32,
    name: String,
    ip_addr: String,
    port: String,
) -> Result<(), Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(AddDeviceRequest {
        home_id,
        room_id,
        device_type,
        name,
        connection: if ip_addr.is_empty() || port.is_empty() {
            None
        } else {
            Some(ConnectionSettings {
                ip: ip_addr,
                port,
                // UDP или TCP будет выбрано на сервере, зависит от типа устройства
                service: "".to_string(),
            })
        },
    });

    match home_service.add_device(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn delete_device(
    home_id: String,
    room_id: String,
    device_id: String,
) -> Result<(), Status> {
    let addr = SH_GRPS_SERVER;
    let client = Client::new_with_options(
        addr.to_string(),
        FetchOptions {
            timeout: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let mut home_service = HomeServiceClient::new(client);

    let request = tonic::Request::new(DeleteDeviceRequest {
        home_id,
        room_id,
        device_id,
    });

    match home_service.delete_device(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
