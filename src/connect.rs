use rmodbus_client::ModBusClient;
use std::{thread::sleep, time::Duration};

pub fn conneting(client: &ModBusClient, vec_connection_config: Vec<String>) {
    for config in vec_connection_config {
        let _handle = match client.init(&config) {
            Ok(handle) => handle,
            Err(err) => {
                println!("Ошибка соединения с сервером: {err}.");
                return;
            }
        };

        while !client.is_connected() {
            println!("Соединение не установлено...");
            sleep(Duration::from_millis(100));
        }

        println!("Подключено!\n");
        println!("Конфигурация соединения: {:#?}\n", config);
    }
}
