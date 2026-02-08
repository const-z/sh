use tonic::Status;

use super::smart_home_contracts;

// #[derive(Debug)]
// pub struct Connection {
//     pub ip: String,
//     pub port: String,
//     pub service: String,
// }

// pub enum DeviceData {
//     Socket(SocketData),
//     Thermometer(ThermometerData),
// }

// pub enum ItemType {
//     Home,
//     Room,
//     Socket,
//     Thermo,
// }

// pub struct ReportItem {
//     pub id: String,
//     pub name: String,
//     pub item_type: ItemType,
//     pub device_connection: Option<Connection>,
//     pub device_data: Option<DeviceData>,
// }

// pub struct Report {
//     pub items: Vec<ReportItem>,
// }

pub trait Repository {
    async fn add_home(&self, name: impl Into<String>) -> Result<String, Status>;
    async fn delete_home(&self, home_id: impl Into<String>) -> Result<(), Status>;
    async fn list_homes(&self) -> Result<Vec<smart_home_contracts::Item>, Status>;

    async fn add_room(
        &self,
        home_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<String, Status>;
    async fn delete_room(
        &self,
        home_id: impl Into<String>,
        room_id: impl Into<String>,
    ) -> Result<(), Status>;
    async fn list_rooms(
        &self,
        home_id: impl Into<String>,
    ) -> Result<Vec<smart_home_contracts::Item>, Status>;

    async fn add_device(
        &self,
        home_id: impl Into<String>,
        room_id: impl Into<String>,
        device_type: smart_home_contracts::DeviceType,
        device_name: String,
        connection: Option<smart_home_contracts::ConnectionSettings>,
    ) -> Result<String, Status>;
    async fn delete_device(
        &self,
        home_id: impl Into<String>,
        room_id: impl Into<String>,
        device_id: impl Into<String>,
    ) -> Result<(), Status>;
    async fn list_devices(
        &self,
        home_id: impl Into<String>,
        room_id: impl Into<String>,
    ) -> Result<Vec<smart_home_contracts::Item>, Status>;
}
