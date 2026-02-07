mod wasm_grpc_client;

use std::rc::Rc;

use chrono::TimeZone;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use web_sys::console;

use wasm_grpc_client::smart_home_contracts;

slint::include_modules!();

impl From<i32> for DeviceType {
    fn from(item_type: i32) -> DeviceType {
        match item_type {
            3 => DeviceType::Socket,
            4 => DeviceType::Thermo,
            _ => panic!("Unknown device type"),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();

    ui.global::<HomeService>().on_request_home_list(move || {
        let ui = ui_handle.unwrap();

        ui.set_app_error("".to_string().into());

        // Получить список домов
        slint::spawn_local(async move {
            match wasm_grpc_client::get_homes().await {
                Ok(homes) => {
                    let main_list = VecModel::<ModelRc<SharedString>>::default();

                    for home in homes {
                        let row_values =
                            vec![SharedString::from(home.0), SharedString::from(home.1)];
                        let row_model = VecModel::from(row_values);
                        main_list.push(ModelRc::new(row_model));
                    }

                    ui.set_homes_list(ModelRc::new(main_list));
                    ui.set_need_refresh(ui.get_need_refresh() + 1);
                }
                Err(e) => {
                    console::error_1(&format!("Error: {}", e).into());
                    ui.set_app_error(e.to_string().into());
                    ui.set_need_refresh(ui.get_need_refresh() + 1);
                }
            }
        })
        .unwrap();
    });

    let ui_handle = ui.as_weak();
    ui.global::<HomeService>()
        .on_request_rooms_list(move |home_id: SharedString| {
            let ui = ui_handle.unwrap();

            ui.set_app_error("".to_string().into());

            // Получить список комнат
            slint::spawn_local(async move {
                match wasm_grpc_client::get_rooms(home_id.to_string()).await {
                    Ok(rooms) => {
                        let main_list = VecModel::<ModelRc<SharedString>>::default();

                        for room in rooms {
                            let row_values =
                                vec![SharedString::from(room.0), SharedString::from(room.1)];
                            let row_model = VecModel::from(row_values);
                            main_list.push(ModelRc::new(row_model));
                        }

                        ui.set_rooms_list(ModelRc::new(main_list));
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                    Err(e) => {
                        console::error_1(&format!("Error: {}", e).into());
                        ui.set_app_error(e.to_string().into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                }
            })
            .unwrap();
        });

    let ui_handle = ui.as_weak();
    ui.global::<HomeService>().on_request_devices_list(
        move |home_id: SharedString, room_id: SharedString| {
            let ui = ui_handle.unwrap();

            ui.set_app_error("".to_string().into());

            // Получить список устройств
            slint::spawn_local(async move {
                match wasm_grpc_client::get_devices(home_id.to_string(), room_id.to_string()).await
                {
                    Ok(devices) => {
                        let mut main_list = vec![];

                        for device in devices {
                            let slint_device = to_slint_device(device);

                            main_list.push(slint_device);
                        }

                        let devices_model = Rc::new(slint::VecModel::from(main_list));
                        ui.set_devices_list(devices_model.into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                    Err(e) => {
                        console::error_1(&format!("Error: {}", e).into());
                        ui.set_app_error(e.to_string().into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                }
            })
            .unwrap();
        },
    );

    let ui_handle = ui.as_weak();
    ui.global::<HomeService>()
        .on_add_home(move |name: SharedString| {
            let ui = ui_handle.unwrap();
            ui.set_app_error("".to_string().into());

            slint::spawn_local(async move {
                match wasm_grpc_client::add_home(name.to_string()).await {
                    Ok(_) => {
                        ui.global::<HomeService>().invoke_request_home_list();
                    }
                    Err(e) => {
                        console::error_1(&format!("Error: {}", e).into());
                        ui.set_app_error(e.to_string().into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                }
            })
            .unwrap();
        });

    let ui_handle = ui.as_weak();
    ui.global::<HomeService>()
        .on_add_room(move |home_id: SharedString, name: SharedString| {
            let ui = ui_handle.unwrap();
            ui.set_app_error("".to_string().into());

            slint::spawn_local(async move {
                match wasm_grpc_client::add_room(home_id.to_string(), name.to_string()).await {
                    Ok(_) => {
                        ui.global::<HomeService>()
                            .invoke_request_rooms_list(home_id);
                    }
                    Err(e) => {
                        console::error_1(&format!("Error: {}", e).into());
                        ui.set_app_error(e.to_string().into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                }
            })
            .unwrap();
        });

    let ui_handle = ui.as_weak();
    ui.global::<HomeService>().on_add_device(
        move |home_id: SharedString,
              room_id: SharedString,
              device_type: i32,
              name: SharedString,
              ip_addr: SharedString,
              port: SharedString| {
            let ui = ui_handle.unwrap();
            ui.set_app_error("".to_string().into());

            slint::spawn_local(async move {
                match wasm_grpc_client::add_device(
                    home_id.to_string(),
                    room_id.to_string(),
                    device_type,
                    name.to_string(),
                    ip_addr.to_string(),
                    port.to_string(),
                )
                .await
                {
                    Ok(_) => {
                        ui.global::<HomeService>()
                            .invoke_request_devices_list(home_id, room_id);
                    }
                    Err(e) => {
                        console::error_1(&format!("Error: {}", e).into());
                        ui.set_app_error(e.to_string().into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                }
            })
            .unwrap();
        },
    );

    let ui_handle = ui.as_weak();
    ui.global::<HomeService>()
        .on_delete_home(move |home_id: SharedString| {
            let ui = ui_handle.unwrap();
            ui.set_app_error("".to_string().into());

            slint::spawn_local(async move {
                match wasm_grpc_client::delete_home(home_id.to_string()).await {
                    Ok(_) => {
                        ui.global::<HomeService>().invoke_request_home_list();
                    }
                    Err(e) => {
                        console::error_1(&format!("Error: {}", e).into());
                        ui.set_app_error(e.to_string().into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                }
            })
            .unwrap();
        });

    let ui_handle = ui.as_weak();
    ui.global::<HomeService>().on_delete_room(
        move |home_id: SharedString, room_id: SharedString| {
            let ui = ui_handle.unwrap();
            ui.set_app_error("".to_string().into());

            slint::spawn_local(async move {
                match wasm_grpc_client::delete_room(
                    home_id.clone().to_string(),
                    room_id.to_string(),
                )
                .await
                {
                    Ok(_) => {
                        ui.global::<HomeService>()
                            .invoke_request_rooms_list(home_id);
                    }
                    Err(e) => {
                        console::error_1(&format!("Error: {}", e).into());
                        ui.set_app_error(e.to_string().into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                }
            })
            .unwrap();
        },
    );

    let ui_handle = ui.as_weak();
    ui.global::<HomeService>().on_delete_device(
        move |home_id: SharedString, room_id: SharedString, device_id: SharedString| {
            let ui = ui_handle.unwrap();
            ui.set_app_error("".to_string().into());

            slint::spawn_local(async move {
                match wasm_grpc_client::delete_device(
                    home_id.to_string(),
                    room_id.to_string(),
                    device_id.to_string(),
                )
                .await
                {
                    Ok(_) => {
                        ui.global::<HomeService>()
                            .invoke_request_devices_list(home_id, room_id);
                    }
                    Err(e) => {
                        console::error_1(&format!("Error: {}", e).into());
                        ui.set_app_error(e.to_string().into());
                        ui.set_need_refresh(ui.get_need_refresh() + 1);
                    }
                }
            })
            .unwrap();
        },
    );

    let ui_handle = ui.as_weak();
    slint::Timer::single_shot(std::time::Duration::from_millis(1000), move || {
        let ui = ui_handle.unwrap();
        ui.global::<HomeService>().invoke_request_home_list();
    });

    ui.run().unwrap();
}

fn to_slint_device(device: smart_home_contracts::Item) -> Device {
    let socket_data = if let Some(smart_home_contracts::item::Value::SocketValue(sv)) = device.value
    {
        SocketData {
            is_on: sv.is_on,
            power: sv.power.to_string().into(),
            timestamp: chrono::Utc
                .timestamp_millis_opt(sv.timestamp as i64)
                .single()
                .unwrap()
                .to_string()
                .into(),
        }
    } else {
        SocketData {
            is_on: false,
            power: "0.0".into(),
            timestamp: "".into(),
        }
    };
    let thermo_data = if let Some(smart_home_contracts::item::Value::ThermoValue(tv)) = device.value
    {
        ThermoData {
            temp: tv.temp.to_string().into(),
            timestamp: chrono::Utc
                .timestamp_millis_opt(tv.timestamp as i64)
                .single()
                .unwrap()
                .to_string()
                .into(),
        }
    } else {
        ThermoData {
            temp: "0.0".into(),
            timestamp: "".into(),
        }
    };
    let is_online = if let Some(smart_home_contracts::item::Value::SocketValue(sv)) = device.value {
        sv.is_online
    } else if let Some(smart_home_contracts::item::Value::ThermoValue(tv)) = device.value {
        tv.is_online
    } else {
        false
    };

    Device {
        id: SharedString::from(device.id),
        name: SharedString::from(device.name),
        device_type: DeviceType::from(device.item_type),
        connection: if let Some(c) = device.device_connection {
            SharedString::from(format!("{} {}:{}", c.service, c.ip, c.port))
        } else {
            "".to_string().into()
        },
        socket_data,
        thermo_data,
        is_online,
    }
}
