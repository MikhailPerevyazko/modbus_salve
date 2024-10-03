use crate::{cmd::ConfigArgs, transport_tcp::TransportTCP};

use serde::Deserialize;
use std::{error::Error, fs, path::PathBuf};

// Структура содержимого файла конфигурации
#[derive(Debug, Deserialize)]
pub struct TransportConfig {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub transport: Vec<TransportConfig>,
}

//? Функция читает и парсит файл конфигурации yaml
pub fn load_config(path: PathBuf) -> Result<Configuration, Box<dyn Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: Configuration = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

//? Функция собирает host:port из конфиг файла
pub fn to_tcp_address(config_content: Configuration) -> String {
    let mut tcp_address = String::new();
    for transport in config_content.transport {
        let host = transport.host;
        let port = transport.port;
        tcp_address = format!("{}:{}", host, port);
    }
    return tcp_address;
}

//? Чтение и запись конфиг файла
pub fn read_config_file() -> Configuration {
    // Получаем путь к файлу конфигурации
    let config_args = ConfigArgs { config_path: None };
    let path_to_config_file = config_args.get_path_to_config();

    // Проверка в терминале наличия пути к конфигурационному файлу
    config_args.check_path_to_config();

    // Получаем содержимое конфигурационного файла
    let config_content = load_config(path_to_config_file).unwrap();
    return config_content;
}

//? Получаем ip адрес
pub fn get_id_addr() -> String {
    let config = read_config_file();
    let id_address = to_tcp_address(config);
    id_address
}

pub fn transport_tcp() -> TransportTCP {
    let ip_addr = get_id_addr();
    let transport_tcp = TransportTCP {
        host_port: ip_addr,
        connected: false,
        stream: None,
    };
    transport_tcp
}
