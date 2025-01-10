mod config;
mod connect;

use config::{get_all_tasks, json_connection_config};
use connect::conneting;
use rmodbus_client::ModBusClient;
use serde_json::Value;
use std::error::Error;
use std::{thread::sleep, time::Duration};

fn main() {
    call_modbus();
}

pub fn call_modbus() {
    // Получить вектор настроек для соединения
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

                match reverse_data(answer) {
                    Ok(numbers) => {
                        // Variant 1
                        let convert_to_hex: Vec<String> = numbers
                            .iter()
                            .map(|&value| format!("{:X}", value))
                            .collect();

                        for v in convert_to_hex {
                            println!("{:?}", v)
                        }

                        // Variant 2
                        for num in numbers {
                            let bytes_num = num.to_be_bytes();
                            println!("{:?}", bytes_num);
                        }
                    }
                    Err(err) => println!("Error: {}", err),
                }
            }
            sleep(Duration::from_millis(200));
        }
        break;
    }
}

pub fn reverse_data(answer: String) -> Result<Vec<i32>, Box<dyn Error>> {
    let input = answer.as_str();
    let value: Value = serde_json::from_str(input).unwrap();

    // Проверяем, что в data находится массив
    if let Some(data_array) = value.get("data").and_then(|d| d.as_array()) {
        let mut numbers: Vec<i32> = data_array
            .iter()
            .filter_map(|v| v.as_i64())
            .map(|n| n as i32) // Преобразуем в i32
            .collect();
        numbers.reverse();
        Ok(numbers)
    } else {
        Err("Ошибка: 'data' не является массивом".into())
    }
}

// pub fn convert_to_hex(revers_data: Vec<i32>) -> Vec<String> {
//     let convert_string_vec = revers_data
//         .iter()
//         .map(|&value| format!("{:X}", value)) // Конвертируем в шестнадцатеричную строку
//         .collect();

//     return convert_string_vec;
// }
