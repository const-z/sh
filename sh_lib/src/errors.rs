use std::{error::Error, fmt::Display};

const ERR_PREFIX: &str = "ERR";
const ROOM_NOT_FOUND_ERR_CODE: &str = "1001";
const DEVICE_NOT_FOUND_ERR_CODE: &str = "1002";

pub struct ErrorInfo {
    pub code: String,
    pub message: String,
}

pub enum SmartHomeErrors {
    RoomNotFound(ErrorInfo),
    DeviceNotFound(ErrorInfo),
}

impl SmartHomeErrors {
    pub fn room_not_found(name: &str) -> Self {
        Self::RoomNotFound(ErrorInfo {
            code: String::from(ROOM_NOT_FOUND_ERR_CODE),
            message: format!(r#"Комната "{}" не найдена"#, name),
        })
    }

    pub fn device_not_found(name: &str) -> Self {
        Self::DeviceNotFound(ErrorInfo {
            code: String::from(DEVICE_NOT_FOUND_ERR_CODE),
            message: format!(r#"Устройство '{}' не найдено"#, name),
        })
    }
}

impl Display for SmartHomeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeErrors::RoomNotFound(err) => {
                write!(f, "{ERR_PREFIX}[{}]: {}", err.code, err.message)
            }
            SmartHomeErrors::DeviceNotFound(err) => {
                write!(f, "{ERR_PREFIX}[{}]: {}", err.code, err.message)
            }
        }
    }
}

impl std::fmt::Debug for SmartHomeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for SmartHomeErrors {}
