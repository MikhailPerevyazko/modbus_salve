use rmodbus_client::ModBusClient;
use std::{thread::sleep, time::Duration};

pub fn conneting(vec_client: Vec<ModBusClient>, vec_json_config: Vec<String>) {
    let sum = vec_client.len();
    let sum_two = vec_json_config.len();
    if sum == sum_two {
        for i in 0..sum {
            let client = vec_client.get(i).unwrap();
            let _handle = match client.init(vec_json_config.get(i).unwrap()) {
                Ok(handle) => handle,
                Err(err) => {
                    println!("Ошибка соединения с сервером: {err}.");
                    return;
                }
            };
            while !client.is_connected() {
                println!("Соединение не установлено. ");
                sleep(Duration::from_millis(100));
            }
            println!("Подключено.");
            println!(
                "Конфигурация соединения: {:?}\n",
                vec_json_config.get(i).unwrap()
            );
        }
    }
}
