use std::{thread::sleep, time::Duration};

use rmodbus_client::ModBusClient;

pub fn connect_tcp(vec_json_config: Vec<String>) {
    let config_tcp = vec_json_config.get(0).unwrap();
    let client = ModBusClient::new();

    let _handle = match client.init(&config_tcp) {
        Ok(handle) => handle,
        Err(err) => {
            println!("Ошибка соединения с сервером: {err}.");
            return;
        }
    };
    while !client.is_connected() {
        println!("Подключение не установлено");
        sleep(Duration::from_millis(200));
    }
    println!("Подключен")
}

pub fn connect_udp(vec_json_config: Vec<String>) {
    let config_udp = vec_json_config.get(1).unwrap();
    let client = ModBusClient::new();
    let _handle = match client.init(&config_udp) {
        Ok(handle) => handle,
        Err(err) => {
            println!("Ошибка соединения с сервером: {err}.");
            return;
        }
    };
    while !client.is_connected() {
        println!("Подключение не установлено");
        sleep(Duration::from_millis(200));
    }
    println!("Подключен")
}

pub fn connect_rtu(vec_json_config: Vec<String>) {
    todo!()
}
