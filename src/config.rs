use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub host: Option<String>,
    pub port: Option<String>,
    pub listen_host: Option<String>,
    pub listen_port: Option<String>,
    pub name: Option<String>,
    pub baud_rate: Option<i32>,
    pub data_bits: Option<i32>,
    pub flow_control: Option<String>,
    pub parity: Option<String>,
    pub stop_bits: Option<i32>,
}

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

pub fn load_connections_config(path_to_config: &str) -> Vec<ClientModbusConfig> {
    let configuration = fs::read_to_string(path_to_config.to_string()).unwrap();
    let configs: ClientModbusConfigs = serde_yaml::from_str(&configuration.as_str()).unwrap();
    let vec_client_modbus_config = configs.connection_configs;
    vec_client_modbus_config
}

pub fn json_connection_config() -> Vec<String> {
    let path = String::from("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_config.yaml");
    let configs = load_connections_config(&path);

    let mut vec_configs: Vec<String> = Vec::new();
    for config in configs {
        let parsed_config = serde_json::to_string(&config).unwrap();
        vec_configs.push(parsed_config.clone());
    }
    vec_configs
}

pub fn load_configs(path_to_config: &str) -> Vec<FunctionParameter> {
    let configuration = fs::read_to_string(path_to_config.to_string()).unwrap();
    let configs: ClientModbusConfigs = serde_yaml::from_str(&configuration.as_str()).unwrap();
    let parameters = configs.parameters;
    parameters
}

pub fn print_parameters_json() -> Vec<String> {
    let vec_parameters_configs =
        load_configs("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_config.yaml");

    let holding = 0;
    let holding1 = 0;
    // let coil = true;
    // let coil1 = true;
    let mut vec_fn: Vec<String> = Vec::new();
    // let count_float = 2;
    // let count_bool = 1;

    for parameters in vec_parameters_configs {
        let mstore = parameters.mstorage;
        let count = match &parameters.ptype.to_lowercase()[..] {
            "bool" => 1,
            "float" => 2,
            _ => 1,
        };
        match &mstore.to_uppercase()[..] {
            "DI" => {
                let read_input_status = format!(
                    r#"{{"id":2,"unit":{},"operation":"ReadInputStatus","address":{},"count":{}}}"#,
                    parameters.unit_id, parameters.start_address, count
                );
                vec_fn.push(read_input_status);
            }

            "DO" => {
                let read_coil_status: String = format!(
                    r#"{{"id":1,"unit":{},"operation":"ReadInputStatus","address":{},"count":{}}}"#,
                    parameters.unit_id, parameters.start_address, count
                );
                // let force_single_coil: String = format!("{{\"id\":7,\"unit\":1,\"operation\":\"ForceSingleCoil\",\"address\":7,\"data\":[{}]}}",if coil {1}else{0});
                // let force_miltiple_coils: String = format!("{{\"id\":8,\"unit\":1,\"operation\":\"ForceMultipleCoils\",\"address\":8,\"data\":[{},{}]}}",if coil {1}else{0},if coil1 {1}else{0});
                vec_fn.push(read_coil_status);
                // vec_fn.push(force_single_coil);
                // vec_fn.push(force_miltiple_coils);
            }

            "AI" => {
                let read_input_registers = format!(
                    r#"{{"id":3,"unit":{},"operation":"ReadInputRegisters","address":{},"count":{}}}"#,
                    parameters.unit_id, parameters.start_address, count
                );
                vec_fn.push(read_input_registers);
            }

            "AO" => {
                let read_holding_registers: String = format!(
                    r#"{{"id":4,"unit":{},"operation":"ReadHoldingRegisters","address":{},"count":{}}}"#,
                    parameters.unit_id, parameters.start_address, count
                );
                let preset_single_register: String = format!("{{\"id\":5,\"unit\":{},\"operation\":\"PresetSingleRegister\",\"address\":{},\"data\":[{holding}]}}", parameters.unit_id, parameters.start_address,);
                let preset_multiple_register: String = format!("{{\"id\":6,\"unit\":{},\"operation\":\"PresetMultipleRegisters\",\"address\":{},\"data\":[{holding},{holding1}]}}", parameters.unit_id, parameters.start_address,);
                vec_fn.push(read_holding_registers);
                vec_fn.push(preset_single_register);
                vec_fn.push(preset_multiple_register);
            }
            _ => println!("something else"),
        }
    }
    vec_fn
}
