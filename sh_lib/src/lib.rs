pub mod builder;
pub mod errors;
pub mod reporter;
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
            let mut room = SmartRoom::new(String::from($name), &[]);
            $(
                room.add_device($device);
            )*
            room
        }
    };
}
