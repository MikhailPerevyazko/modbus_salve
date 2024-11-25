mod config;
mod connect;

use config::{get_parameters_from_config, json_connection_config};
use connect::conneting;
use rmodbus_client::ModBusClient;
use std::{thread::sleep, time::Duration};

fn main() {
    // Получить вектор настроек для соединения
    let connection_config = json_connection_config();
    let client_tcp = ModBusClient::new();
    // Подключение к modbus slave
    conneting(&client_tcp, connection_config);
    // Получить вектор задач
    let vec_tasks = get_parameters_from_config();

    // Основной цикл
    loop {
        println!("{:#?}", vec_tasks);
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
                let answer = client_tcp.last_response_str();
                println!("Ответ: {:?}", answer.unwrap());
            }
            sleep(Duration::from_millis(500));
        }
        break;
    }
}
