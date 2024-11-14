use serde::{Deserialize, Serialize};
use std::fs;

fn main() {
    let x = parse_to_json_format();
    println!("{:#?}", x);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TcpConnection {
    pub host: String,
    pub port: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UdpConnection {
    pub host: String,
    pub port: String,
    pub listen_host: String,
    pub listen_port: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RtuConnection {
    pub name: String,
    pub baud_rate: i32,
    pub data_bits: i32,
    pub flow_control: Option<String>,
    pub parity: Option<String>,
    pub stop_bits: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Connection {
    TcpConnection,
    UdpConnection,
    RtuConnection,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Connection {
//     pub host: Option<String>,
//     pub port: Option<String>,
//     pub listen_host: Option<String>,
//     pub listen_port: Option<String>,
//     pub name: Option<String>,
//     pub baud_rate: Option<i32>,
//     pub data_bits: Option<i32>,
//     pub flow_control: Option<String>,
//     pub parity: Option<String>,
//     pub stop_bits: Option<i32>,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientModbusConfig {
    pub protocol_type: String,
    pub connection: Connection,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub unit_id: i32,
    pub ptype: String,
    pub start_address: i32,
    pub mstorage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientModbusConfigs {
    pub connection_configs: Vec<ClientModbusConfig>,
    pub parameters: Vec<FunctionParameter>,
}

pub fn load_configuration(path_to_config: &str) -> Vec<ClientModbusConfig> {
    let configuration = fs::read_to_string(path_to_config.to_string()).unwrap();
    let configs: ClientModbusConfigs = serde_yaml::from_str(&configuration.as_str()).unwrap();
    let vec_client_modbus_config = configs.connection_configs;
    vec_client_modbus_config
}

pub fn parse_to_json_format() -> Vec<String> {
    let configs =
        load_configuration("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_config.yaml");

    let mut vec_configs: Vec<String> = Vec::new();
    for config in configs {
        let parsed_config = serde_json::to_string(&config).unwrap();
        vec_configs.push(parsed_config.clone());
    }
    vec_configs
}
