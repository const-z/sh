use crate::{smart_device::SmartDeviceType, smart_room::SmartRoom};

trait HomeBuilder {
    fn add_room(&mut self, item: SmartRoom) -> Self;
}

trait RoomBuilder {
    fn add_device(&mut self, item: SmartDeviceType) -> Self;
}
