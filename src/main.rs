mod config;
mod connect;

use config::{get_all_tasks, json_connection_config};
use connect::conneting;
use rmodbus_client::ModBusClient;
use serde_json::json;
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
    let mut num_task = 0;
    let vec_tasks = get_all_tasks();
    for tasks in vec_tasks.iter() {
        for task in tasks {
            num_task += 1;
            println!("Task #{}: {:#?}", num_task, task)
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
                print_response(answer);
            }
            sleep(Duration::from_millis(100));
        }
        break;
    }
}

pub fn reverse_data_in_response(answer: String) -> Result<Vec<i32>, Box<dyn Error>> {
    let input_answer = answer.as_str();
    let value_from_answer: Value = serde_json::from_str(input_answer).unwrap();

    // Проверяем, что в data находится массив
    if let Some(data_array) = value_from_answer.get("data").and_then(|d| d.as_array()) {
        let mut numbers: Vec<i32> = data_array
            .iter()
            .filter_map(|v| v.as_i64())
            .map(|n| n as i32) // Преобразуем в i32
            .collect();
        numbers.reverse();

        Ok(numbers)
    } else {
        Err("Ошибка: В response 'data' не является массивом".into())
    }
}

pub fn make_new_response_with_hex_data(bytes_data: Vec<[u8; 4]>, answer: &str) -> String {
    answer.to_string();
    let mut parsed: Value = serde_json::from_str(&answer).unwrap();

    let hex_data: Vec<String> = bytes_data
        .into_iter()
        .map(|byte_array| {
            byte_array
                .iter()
                .map(|&byte| format!("{}", byte))
                .collect::<String>()
        })
        .collect();

    if let Some(data_field) = parsed.as_object_mut().and_then(|obj| obj.get_mut("data")) {
        *data_field = json!(hex_data);
    }
    let new_answer = serde_json::to_string(&parsed).unwrap();

    return new_answer;
}

pub fn print_response(answer: String) {
    let double_answer = answer.clone();
    println!("Ответ: {}", answer);

    let reverse_data_vec = reverse_data_in_response(answer).unwrap();

    let mut bytes_data: Vec<[u8; 4]> = Vec::new();
    for v in reverse_data_vec {
        let bytes_val = v.to_be_bytes();
        bytes_data.push(bytes_val);
    }

    let new_answer = make_new_response_with_hex_data(bytes_data.clone(), &double_answer);
    println!("Новый ответ: {}", new_answer);
}
