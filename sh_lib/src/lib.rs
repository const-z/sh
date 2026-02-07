pub mod builder;
pub mod errors;
pub mod id;
pub mod reporter;
pub mod rich_console;
pub mod smart_device;
pub mod smart_home;
pub mod smart_room;
pub mod subscriber;

/// Макрос для создания комнат
#[macro_export]
macro_rules! create_room {
    ($name:expr) => {
        SmartRoom::new($name)
    };

    ($name:expr, $($device:expr),* $(,)?) => {
        {
            use sh_lib::smart_room::SmartRoom;
            let mut room = SmartRoom::new($name);
            $(
                room.add_device($device);
            )*
            room
        }
    };
}
