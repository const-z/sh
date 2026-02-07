use std::{collections::HashMap, sync::Arc};

use sh_lib::{
    id::{self, Id},
    smart_device::{
        SmartDevice, SmartDeviceType, SmartSocket, SmartThermometer,
        online::{ConnectionType, OnlineDevice},
    },
    smart_home::SmartHome,
    smart_room::SmartRoom,
};
use tokio::sync::RwLock;
use tonic::Status;
use tracing::{info, warn};

use crate::smart_home_contracts::{self, Item, ItemType, ThermometrValue};
use crate::{
    repository::Repository,
    smart_home_contracts::{ConnectionSettings, SocketValue, item::Value},
};

pub struct Store {
    _inner: Arc<RwLock<HashMap<String, SmartHome>>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            _inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Repository for Store {
    async fn add_home(&self, name: impl Into<String>) -> Result<String, Status> {
        let mut homes = self._inner.write().await;
        let new_home = SmartHome::new(name);

        if homes.contains_key(&new_home.get_id().to_string()) {
            return Err(Status::already_exists("Home already exists"));
        }

        let home_id = new_home.get_id().clone();
        homes.insert(home_id.to_string(), new_home);

        Ok(home_id.to_string())
    }

    async fn delete_home(&self, home_id: impl Into<String>) -> Result<(), Status> {
        let mut homes = self._inner.write().await;

        match homes.remove(&home_id.into()) {
            Some(_) => Ok(()),
            None => Err(Status::not_found("Home not found")),
        }
    }

    async fn add_room(
        &self,
        home_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<String, Status> {
        let mut homes = self._inner.write().await;

        let home = if let Some(home) = homes.get_mut(&home_id.into()) {
            home
        } else {
            return Err(Status::not_found("Home not found"));
        };

        let room = SmartRoom::new(name);

        if home.get_room(room.get_id()).is_some() {
            return Err(Status::already_exists("Room already exists in home"));
        }

        let room_id = home.add_room(room);

        Ok(room_id.to_string())
    }

    async fn delete_room(
        &self,
        home_id: impl Into<String>,
        room_id: impl Into<String>,
    ) -> Result<(), Status> {
        let mut homes = self._inner.write().await;

        let home = if let Some(home) = homes.get_mut(&home_id.into()) {
            home
        } else {
            return Err(Status::not_found("Home not found"));
        };

        match home.delete_room(&Id::with_inner(room_id)) {
            Some(_) => Ok(()),
            None => Err(Status::not_found("Room not found")),
        }
    }

    async fn add_device(
        &self,
        home_id: impl Into<String>,
        room_id: impl Into<String>,
        device_type: smart_home_contracts::DeviceType,
        device_name: String,
        connection: Option<smart_home_contracts::ConnectionSettings>,
    ) -> Result<String, Status> {
        let mut homes = self._inner.write().await;

        let home = if let Some(home) = homes.get_mut(&home_id.into()) {
            home
        } else {
            return Err(Status::not_found("Home not found"));
        };

        let room = if let Some(room) = home.get_room_mut(&Id::with_inner(room_id)) {
            room
        } else {
            return Err(Status::not_found("Room not found"));
        };

        if room
            .get_device(&id::Id::from_string(device_name.clone()))
            .is_some()
        {
            return Err(Status::already_exists("Device already exists in room"));
        }

        let device = match device_type {
            smart_home_contracts::DeviceType::Socket => match connection {
                Some(c) => SmartDeviceType::Socket(SmartSocket::new_with_connection(
                    device_name,
                    0.0,
                    false,
                    ConnectionType::Tcp {
                        ip: c.ip.parse().unwrap(),
                        port: c.port.parse().unwrap(),
                    },
                )),
                None => SmartDeviceType::Socket(SmartSocket::new(device_name, 0.0, false)),
            },
            smart_home_contracts::DeviceType::Thermo => match connection {
                Some(c) => SmartDeviceType::Thermometer(SmartThermometer::new_with_connection(
                    device_name,
                    0.0,
                    ConnectionType::Udp {
                        bind_ip: c.ip.parse().unwrap(),
                        bind_port: c.port.parse().unwrap(),
                    },
                )),
                None => SmartDeviceType::Thermometer(SmartThermometer::new(device_name, 0.0)),
            },
            _ => {
                return Err(Status::invalid_argument("Invalid device type"));
            }
        };

        let device_id = room.add_device(device);

        if let Some(device) = room.get_device(&device_id) {
            info!("Try connecting device: {device_id}");
            match device.connect().await {
                Ok(_) => (),
                Err(err) => {
                    warn!("Failed to connect device: {device_id}, {err}");
                }
            };
        }

        Ok(device_id.to_string())
    }

    async fn delete_device(
        &self,
        home_id: impl Into<String>,
        room_id: impl Into<String>,
        device_id: impl Into<String>,
    ) -> Result<(), Status> {
        let mut homes = self._inner.write().await;

        let home = if let Some(home) = homes.get_mut(&home_id.into()) {
            home
        } else {
            return Err(Status::not_found("Home not found"));
        };

        let room = if let Some(room) = home.get_room_mut(&Id::with_inner(room_id)) {
            room
        } else {
            return Err(Status::not_found("Room not found"));
        };

        match room.delete_device(&Id::with_inner(device_id.into())) {
            Some(_) => Ok(()),
            None => Err(Status::not_found("Device not found")),
        }
    }

    async fn list_homes(&self) -> Result<Vec<Item>, Status> {
        let homes = self._inner.read().await;

        let mut items: Vec<Item> = homes
            .values()
            .map(|home| Item {
                id: home.get_id().to_string(),
                name: home.get_name().to_string(),
                item_type: ItemType::Home.into(),
                device_connection: None,
                value: None,
            })
            .collect();

        items.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(items)
    }

    async fn list_rooms(&self, home_id: impl Into<String>) -> Result<Vec<Item>, Status> {
        let homes = self._inner.read().await;

        let home = if let Some(home) = homes.get(&home_id.into()) {
            home
        } else {
            return Err(Status::not_found("Home not found"));
        };

        let mut items: Vec<Item> = home
            .get_rooms()
            .values()
            .map(|room| Item {
                id: room.get_id().to_string(),
                name: room.get_name().to_string(),
                item_type: ItemType::Room.into(),
                device_connection: None,
                value: None,
            })
            .collect();

        items.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(items)
    }

    async fn list_devices(
        &self,
        home_id: impl Into<String>,
        room_id: impl Into<String>,
    ) -> Result<Vec<Item>, Status> {
        let homes = self._inner.read().await;

        let home = if let Some(home) = homes.get(&home_id.into()) {
            home
        } else {
            return Err(Status::not_found("Home not found"));
        };

        let room = if let Some(room) = home.get_room(&Id::with_inner(room_id)) {
            room
        } else {
            return Err(Status::not_found("Room not found"));
        };

        let mut items: Vec<Item> = vec![];

        for (device_id, device) in room.get_devices() {
            let data = device.get_data().await;
            let connection: Option<ConnectionSettings> =
                device
                    .get_connection()
                    .map(|connection| ConnectionSettings {
                        ip: connection.get_addr().ip().to_string(),
                        port: format!("{}", connection.get_addr().port()),
                        service: match connection {
                            ConnectionType::Tcp { .. } => "TCP".to_string(),
                            ConnectionType::Udp { .. } => "UDP".to_string(),
                        },
                    });

            println!("data: {:?}, connection: {:?}", data, connection);

            let device_data = device.get_data().await;

            match device {
                SmartDeviceType::Socket(_) => {
                    items.push(Item {
                        id: device_id.to_string(),
                        name: device.get_name().to_string(),
                        item_type: ItemType::Socket.into(),
                        device_connection: connection,
                        value: Some(Value::SocketValue(SocketValue {
                            is_on: device_data.as_socket().is_on,
                            power: device_data.as_socket().power,
                            timestamp: device_data.as_socket().timestamp,
                            is_online: device_data.as_socket().is_online,
                        })),
                    });
                }
                SmartDeviceType::Thermometer(_) => {
                    items.push(Item {
                        id: device_id.to_string(),
                        name: device.get_name().to_string(),
                        item_type: ItemType::Thermo.into(),
                        device_connection: connection,
                        value: Some(Value::ThermoValue(ThermometrValue {
                            is_online: device_data.as_thermometer().is_online,
                            temp: device_data.as_thermometer().temp,
                            timestamp: device_data.as_thermometer().timestamp,
                        })),
                    });
                }
            }
        }

        items.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(items)
    }
}
