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

pub fn get_parameters_from_config() -> Vec<Vec<String>> {
    // Записываем вектор из структур с yaml
    let content =
        load_configs("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_config.yaml");

    let mut do_vec: Vec<FunctionParameter> = Vec::new();
    let mut di_vec: Vec<FunctionParameter> = Vec::new();
    let mut ao_vec: Vec<FunctionParameter> = Vec::new();
    let mut ai_vec: Vec<FunctionParameter> = Vec::new();

    // Разделить струкрутры по четырем векторам в зависимости от mstorage
    for v in content {
        if v.mstorage == "DO" {
            do_vec.push(v);
        } else if v.mstorage == "DI" {
            di_vec.push(v);
        } else if v.mstorage == "AO" {
            ao_vec.push(v);
        } else if v.mstorage == "AI" {
            ai_vec.push(v);
        } else {
            println!("Неверный mstorage!")
        }
    }

    // Сортировать векторы от min к max по start_address
    do_vec.sort_by(|a, b| a.start_address.cmp(&b.start_address));
    di_vec.sort_by(|a, b| a.start_address.cmp(&b.start_address));
    ao_vec.sort_by(|a, b| a.start_address.cmp(&b.start_address));
    ai_vec.sort_by(|a, b| a.start_address.cmp(&b.start_address));

    // Парсим векторы в json-формат
    let mut json_vec_do: Vec<String> = Vec::new();
    for params in &do_vec {
        let parsed_params = serde_json::to_string(&params).unwrap();
        json_vec_do.push(parsed_params.clone());
    }

    let mut json_vec_di: Vec<String> = Vec::new();
    for params in &di_vec {
        let parsed_params = serde_json::to_string(&params).unwrap();
        json_vec_di.push(parsed_params.clone());
    }

    let mut json_vec_ao: Vec<String> = Vec::new();
    for params in &ao_vec {
        let parsed_params = serde_json::to_string(&params).unwrap();
        json_vec_ao.push(parsed_params.clone());
    }

    let mut json_vec_ai: Vec<String> = Vec::new();
    for params in &ai_vec {
        let parsed_params = serde_json::to_string(&params).unwrap();
        json_vec_ai.push(parsed_params.clone());
    }

    let mut vec_fn_do: Vec<String> = Vec::new();
    let id: i32 = 1;
    for parameters in do_vec {
        let count = match &parameters.ptype.to_lowercase()[..] {
            "bool" => 1,
            "float" => 2,
            _ => 1,
        };

        let read_coil_status: String = format!(
            r#"{{"id":{},"unit":{},"operation":"ReadCoilStatus","address":{},"count":{}}}"#,
            id, parameters.unit_id, parameters.start_address, count
        );
        vec_fn_do.push(read_coil_status);
    }

    let mut vec_fn_di: Vec<String> = Vec::new();

    for parameters in di_vec {
        let count = match &parameters.ptype.to_lowercase()[..] {
            "bool" => 1,
            "float" => 2,
            _ => 1,
        };

        let read_input_status = format!(
            r#"{{"id":{},"unit":{},"operation":"ReadInputStatus","address":{},"count":{}}}"#,
            id, parameters.unit_id, parameters.start_address, count
        );
        vec_fn_di.push(read_input_status);
    }

    let mut vec_fn_ao: Vec<String> = Vec::new();
    for parameters in ao_vec {
        let count = match &parameters.ptype.to_lowercase()[..] {
            "bool" => 1,
            "float" => 2,
            _ => 1,
        };

        let read_holding_registers: String = format!(
            r#"{{"id":{},"unit":{},"operation":"ReadHoldingRegisters","address":{},"count":{}}}"#,
            id, parameters.unit_id, parameters.start_address, count
        );
        vec_fn_ao.push(read_holding_registers);
    }

    let mut vec_fn_ai: Vec<String> = Vec::new();
    for parameters in ai_vec {
        let count = match &parameters.ptype.to_lowercase()[..] {
            "bool" => 1,
            "float" => 2,
            _ => 1,
        };

        let read_holding_registers: String = format!(
            r#"{{"id":{},"unit":{},"operation":"ReadInputRegisters","address":{},"count":{}}}"#,
            id, parameters.unit_id, parameters.start_address, count
        );
        vec_fn_ai.push(read_holding_registers);
    }

    let mut vec_func_tasks = Vec::new();
    vec_func_tasks.push(vec_fn_do);
    vec_func_tasks.push(vec_fn_di);
    vec_func_tasks.push(vec_fn_ao);
    vec_func_tasks.push(vec_fn_ai);

    println!("{:#?}", vec_func_tasks);

    vec_func_tasks
}
