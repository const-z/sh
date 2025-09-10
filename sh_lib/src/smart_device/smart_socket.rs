use super::{OnOff, SmartDevice};

pub struct SmartSocket {
    name: String,
    power: f32,
    is_on: OnOff,
}

impl SmartSocket {
    pub fn new(name: String, power: f32, is_on: OnOff) -> Self {
        Self { name, power, is_on }
    }

    /// Включить розетку
    pub fn turn_on(&mut self) {
        self.is_on = OnOff::On;
    }

    /// Выключить розетку
    pub fn turn_off(&mut self) {
        self.is_on = OnOff::Off;
    }

    /// Проверить, включена ли розетка
    pub fn is_on(&self) -> bool {
        match self.is_on {
            OnOff::On => true,
            OnOff::Off => false,
        }
    }

    /// Возвращает потребляемую мощность
    pub fn get_power(&self) -> f32 {
        match self.is_on {
            OnOff::On => self.power,
            OnOff::Off => 0.0,
        }
    }
}

impl SmartDevice for SmartSocket {
    /// Получить статус розетки
    fn get_status(&self) -> String {
        format!(
            "{}: {}",
            self.name,
            match self.is_on {
                OnOff::On => format!("Вкл, {} Вт", self.power),
                OnOff::Off => "Выкл".to_string(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_power_zero_if_off() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::Off);
        assert_eq!(socket.get_power(), 0.0);
    }

    #[test]
    fn socket_power() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On);
        assert_eq!(socket.get_power(), 1000.0);
    }

    #[test]
    fn socket_status() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On);
        assert_eq!(socket.get_status(), "Розетка: Вкл, 1000 Вт");
    }

    #[test]
    fn socket_status_off() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::Off);
        assert_eq!(socket.get_status(), "Розетка: Выкл");
    }

    #[test]
    fn socket_turn_on() {
        let mut socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::Off);
        socket.turn_on();
        assert_eq!(socket.get_status(), "Розетка: Вкл, 1000 Вт");
    }

    #[test]
    fn socket_turn_off() {
        let mut socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On);
        socket.turn_off();
        assert_eq!(socket.get_status(), "Розетка: Выкл");
    }

    #[test]
    fn socket_is_on_true() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::On);
        assert_eq!(socket.is_on(), true);
    }

    #[test]
    fn socket_is_on_false() {
        let socket = SmartSocket::new(String::from("Розетка"), 1000.0, OnOff::Off);
        assert_eq!(socket.is_on(), false);
    }
}
