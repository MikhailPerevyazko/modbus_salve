mod config;
mod connect;

use std::{thread::sleep, time::Duration};

use config::{get_parameters_from_config, json_connection_config};
use connect::conneting;
use rmodbus_client::ModBusClient;

fn main() {
    let connection_config = json_connection_config();
    let client_tcp = ModBusClient::new();
    conneting(&client_tcp, connection_config);

    let vec_tasks = get_parameters_from_config();
    let vec_do = vec_tasks.get(0).unwrap();
    let vec_di = vec_tasks.get(1).unwrap();
    let vec_ao = vec_tasks.get(2).unwrap();
    let vec_ai = vec_tasks.get(3).unwrap();

    loop {
        for task in vec_do {
            if let Err(err) = client_tcp.push_back_task_from_str(&task) {
                println!("Ошибка отправки запроса: {:?}", err);
                continue;
            }
            while !client_tcp.have_got_responses() {
                sleep(Duration::from_millis(1));
            }
            let answer = client_tcp.last_responses_str();
            println!("Answer coils status: {:?}", answer.unwrap());
        }
        sleep(Duration::from_secs(1));

        for task in vec_di {
            if let Err(err) = client_tcp.push_back_task_from_str(&task) {
                println!("Ошибка отправки запроса: {:?}", err);
                continue;
            }
            while !client_tcp.have_got_responses() {
                sleep(Duration::from_millis(1));
            }
            let answer = client_tcp.last_responses_str();
            println!("Answer input status: {:?}", answer.unwrap());
        }
        sleep(Duration::from_secs(1));

        for task in vec_ao {
            if let Err(err) = client_tcp.push_back_task_from_str(&task) {
                println!("Ошибка отправки запроса: {:?}", err);
                continue;
            }
            while !client_tcp.have_got_responses() {
                sleep(Duration::from_millis(1));
            }
            let answer = client_tcp.last_responses_str();
            println!("Answer holding registers status: {:?}", answer.unwrap());
        }
        sleep(Duration::from_secs(1));

        for task in vec_ai {
            if let Err(err) = client_tcp.push_back_task_from_str(&task) {
                println!("Ошибка отправки запроса: {:?}", err);
                continue;
            }
            while !client_tcp.have_got_responses() {
                sleep(Duration::from_millis(1));
            }
            let answer = client_tcp.last_responses_str();
            println!("Answer input register status: {:?}", answer.unwrap());
        }
        sleep(Duration::from_secs(1));

        break;
    }
}
