use crate::{smart_device::SmartDeviceType, smart_home::SmartHome, smart_room::SmartRoom};

#[derive(Debug, Default)]
pub struct HomeBuilder {
    rooms: Vec<SmartRoom>,
}

impl HomeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_room(self, room_name: String) -> RoomBuilder {
        RoomBuilder::new(self, room_name)
    }
}

#[derive(Debug)]
pub struct RoomBuilder {
    name: String,
    devices: Vec<SmartDeviceType>,
    home_builder: HomeBuilder,
}

impl RoomBuilder {
    pub fn new(home_builder: HomeBuilder, name: String) -> Self {
        Self {
            name,
            devices: Vec::new(),
            home_builder,
        }
    }

    pub fn add_room(mut self, room_name: String) -> Self {
        self.home_builder
            .rooms
            .push(SmartRoom::new_with_devices(self.name, &self.devices));

        self.home_builder.add_room(room_name)
    }

    pub fn add_device(mut self, device: SmartDeviceType) -> Self {
        self.devices.push(device);

        self
    }

    pub fn build(mut self) -> SmartHome {
        self.home_builder
            .rooms
            .push(SmartRoom::new_with_devices(self.name, &self.devices));

        SmartHome::new_with_rooms("Дом", &self.home_builder.rooms)
    }
}
