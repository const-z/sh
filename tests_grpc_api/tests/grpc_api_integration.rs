use tests_grpc_api::{
    add_device, add_home, add_room, delete_device, delete_home, delete_room, list_devices,
    list_homes, list_rooms,
};

mod smart_home_contracts {
    tonic::include_proto!("smart_home.v1");
}
#[tokio::test]
async fn test_list_homes() {
    let homes = list_homes().await;
    assert!(!homes.is_empty());
}

#[tokio::test]
async fn test_add_home() {
    let home_id = add_home().await;
    let homes = list_homes().await;
    assert!(homes.iter().any(|h| h.id == home_id));
}

#[tokio::test]
async fn test_delete_home() {
    let home_id = add_home().await;
    assert!(delete_home(home_id).await.is_ok());
}

#[tokio::test]
async fn test_delete_missing_home() {
    match delete_home("missing-id".to_string()).await {
        Ok(_) => panic!("Expected error"),
        Err(err) => assert!(err.code() == tonic::Code::NotFound),
    };
}

#[tokio::test]
async fn test_list_rooms() {
    let home_id = add_home().await;
    let room_id = add_room(home_id.clone()).await;
    let rooms = list_rooms(home_id).await;
    assert!(rooms.iter().any(|r| r.id == room_id));
}

#[tokio::test]
async fn test_add_room() {
    let home_id = add_home().await;
    let room_id = add_room(home_id).await;
    assert!(!room_id.is_empty());
}

#[tokio::test]
async fn test_delete_room() {
    let home_id = add_home().await;
    let room_id = add_room(home_id.clone()).await;
    assert!(delete_room(home_id, room_id).await.is_ok());
}

#[tokio::test]
async fn test_delete_missing_room() {
    match delete_room("missing-id".to_string(), "missing-id".to_string()).await {
        Ok(_) => panic!("Expected error"),
        Err(err) => assert!(err.code() == tonic::Code::NotFound),
    };
}

#[tokio::test]
async fn test_add_device() {
    let home_id = add_home().await;
    let room_id = add_room(home_id.clone()).await;
    let device_id = add_device(home_id, room_id).await;
    assert!(!device_id.is_empty());
}

#[tokio::test]
async fn test_list_devices() {
    let home_id = add_home().await;
    let room_id = add_room(home_id.clone()).await;
    let device_id = add_device(home_id.clone(), room_id.clone()).await;
    let devices = list_devices(home_id, room_id).await;
    assert!(devices.iter().any(|d| d.id == device_id));
}

#[tokio::test]
async fn test_delete_device() {
    let home_id = add_home().await;
    let room_id = add_room(home_id.clone()).await;
    let device_id = add_device(home_id.clone(), room_id.clone()).await;
    assert!(delete_device(home_id, room_id, device_id).await.is_ok());
}

#[tokio::test]
async fn test_delete_missing_device() {
    match delete_device(
        "missing-id".to_string(),
        "missing-id".to_string(),
        "missing-id".to_string(),
    )
    .await
    {
        Ok(_) => panic!("Expected error"),
        Err(err) => assert!(err.code() == tonic::Code::NotFound),
    };
}
