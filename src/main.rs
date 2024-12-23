mod config;
mod connect;

use config::get_all_tasks;
use config::json_connection_config;
use connect::conneting;
use rmodbus_client::ModBusClient;
use std::{thread::sleep, time::Duration};

fn main() {
    call_modbus();
}

pub fn call_modbus() {
    //? Получить вектор настроек для соединения
    let connection_config = json_connection_config();

    let mut vec_configs_tcp: Vec<String> = Vec::new();
    let mut vec_configs_udp: Vec<String> = Vec::new();
    let mut vec_configs_rtu: Vec<String> = Vec::new();

    let client_tcp = ModBusClient::new();
    let client_udp = ModBusClient::new();
    // let client_rtu = ModBusClient::new();

    for config in connection_config {
        if config.channel_id == 1 {
            let parsed_config = serde_json::to_string(&config).unwrap();
            vec_configs_tcp.push(parsed_config.clone());
        } else if config.channel_id == 2 {
            let parsed_config = serde_json::to_string(&config).unwrap();
            vec_configs_udp.push(parsed_config.clone());
        } else if config.channel_id == 3 {
            let parsed_config = serde_json::to_string(&config).unwrap();
            vec_configs_rtu.push(parsed_config.clone());
        } else {
            println!("unknown channel id")
        }
    }

    conneting(&client_tcp, vec_configs_tcp);
    conneting(&client_udp, vec_configs_udp);
    // conneting(&client_rtu, vec_configs_rtu);

    // Вывести в терминале все таски
    let vec_tasks = get_all_tasks();
    for tasks in vec_tasks.iter() {
        for task in tasks {
            println!("{:#?}", task)
        }
    }

    // Основной цикл
    loop {
        for vec in &vec_tasks {
            println!("\n");
            for task in vec {
                if let Err(err) = client_tcp.push_back_task_from_str(&task) {
                    println!("Ошибка отправки запроса: {:?}", err);
                    continue;
                }

                while !client_tcp.have_got_responses() {
                    sleep(Duration::from_millis(1));
                }

                let answer = client_tcp.last_response_str().unwrap();
                println!("Ответ: {:#?}", answer);
            }
            sleep(Duration::from_millis(200));
        }
        break;
    }
}
