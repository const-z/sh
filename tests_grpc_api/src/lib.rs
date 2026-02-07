mod smart_home_contracts {
    tonic::include_proto!("smart_home.v1");
}

use smart_home_contracts::home_service_client::HomeServiceClient;

use smart_home_contracts::{
    AddDeviceRequest, AddHomeRequest, AddRoomRequest, DeleteDeviceRequest, DeleteHomeRequest,
    DeleteRoomRequest, DeviceType, Item, ListDevicesRequest, ListHomesRequest, ListRoomsRequest,
};
use tonic::{Response, Status};
use uuid::Uuid;

use crate::smart_home_contracts::{DeleteDeviceResponse, DeleteHomeResponse, DeleteRoomResponse};

const ADDR_GRPC_API: &str = "http://127.0.0.1:50051";

pub async fn list_homes() -> Vec<Item> {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();
    let mut client = HomeServiceClient::new(channel);

    client
        .list_homes(tonic::Request::new(ListHomesRequest {}))
        .await
        .unwrap()
        .into_inner()
        .items
}

pub async fn add_home() -> String {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();
    let mut client = HomeServiceClient::new(channel);
    let req = tonic::Request::new(AddHomeRequest {
        name: Uuid::new_v4().to_string(),
    });

    client.add_home(req).await.unwrap().into_inner().home_id
}

pub async fn delete_home(home_id: String) -> Result<Response<DeleteHomeResponse>, Status> {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();
    let mut client = HomeServiceClient::new(channel);
    let req = tonic::Request::new(DeleteHomeRequest { home_id });

    client.delete_home(req).await
}

pub async fn list_rooms(home_id: String) -> Vec<Item> {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();
    let mut client = HomeServiceClient::new(channel);
    let req = tonic::Request::new(ListRoomsRequest { home_id });

    client.list_rooms(req).await.unwrap().into_inner().items
}

pub async fn add_room(home_id: String) -> String {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();
    let mut client = HomeServiceClient::new(channel);
    let req = tonic::Request::new(AddRoomRequest {
        home_id,
        name: Uuid::new_v4().to_string(),
    });

    client.add_room(req).await.unwrap().into_inner().room_id
}

pub async fn delete_room(
    home_id: String,
    room_id: String,
) -> Result<Response<DeleteRoomResponse>, Status> {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();
    let mut client = HomeServiceClient::new(channel);
    let req = tonic::Request::new(DeleteRoomRequest { home_id, room_id });

    client.delete_room(req).await
}

pub async fn add_device(home_id: String, room_id: String) -> String {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();
    let mut client = HomeServiceClient::new(channel);
    let req = tonic::Request::new(AddDeviceRequest {
        home_id,
        room_id,
        name: Uuid::new_v4().to_string(),
        device_type: DeviceType::Socket as i32,
        connection: None,
    });

    client.add_device(req).await.unwrap().into_inner().device_id
}

pub async fn list_devices(home_id: String, room_id: String) -> Vec<Item> {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();

    let mut client = HomeServiceClient::new(channel);
    let req = tonic::Request::new(ListDevicesRequest { home_id, room_id });

    client.list_devices(req).await.unwrap().into_inner().items
}

pub async fn delete_device(
    home_id: String,
    room_id: String,
    device_id: String,
) -> Result<Response<DeleteDeviceResponse>, Status> {
    let channel = tonic::transport::Channel::from_static(ADDR_GRPC_API)
        .connect()
        .await
        .unwrap();
    let mut client = HomeServiceClient::new(channel);
    let req = tonic::Request::new(DeleteDeviceRequest {
        home_id,
        room_id,
        device_id,
    });

    client.delete_device(req).await
}
