use std::{error::Error, fmt::Display};

const ERR_PREFIX: &str = "ERR";
const ROOM_NOT_FOUND_ERR_CODE: &str = "1001";
const DEVICE_NOT_FOUND_ERR_CODE: &str = "1002";
const DECODE_MESSAGE_ERROR: &str = "1003";
const GETTING_STATUS_ERROR: &str = "1004";
const SOME_EMULATOR_ERROR: &str = "1005";

pub struct ErrorInfo {
    pub code: String,
    pub message: String,
}

pub enum SmartHomeErrors {
    RoomNotFound(ErrorInfo),
    DeviceNotFound(ErrorInfo),
    DecodeMessageError(ErrorInfo),
    GettingStatusError(ErrorInfo),
    EmulatorError(ErrorInfo),
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

    pub fn decode_message_error(e: String) -> Self {
        Self::DecodeMessageError(ErrorInfo {
            code: String::from(DECODE_MESSAGE_ERROR),
            message: format!(r#"Не удалось декодировать сообщение: {}"#, e),
        })
    }

    pub fn getting_status_error(e: String) -> Self {
        Self::DecodeMessageError(ErrorInfo {
            code: String::from(GETTING_STATUS_ERROR),
            message: format!(r#"Ошибка при получении статуса устройства: {}"#, e),
        })
    }

    pub fn emulator_error(e: String) -> Self {
        Self::EmulatorError(ErrorInfo {
            code: String::from(SOME_EMULATOR_ERROR),
            message: format!(r#"Ошибка в удаленном устройстве: {}"#, e),
        })
    }
}

impl Display for SmartHomeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeErrors::RoomNotFound(err)
            | SmartHomeErrors::DeviceNotFound(err)
            | SmartHomeErrors::DecodeMessageError(err)
            | SmartHomeErrors::GettingStatusError(err)
            | SmartHomeErrors::EmulatorError(err) => {
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
