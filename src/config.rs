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
pub struct ConnectionModbusConfig {
    pub protocol_type: String,
    pub connection: Connection,
    pub channel_id: i32,
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
    pub connection_configs: Vec<ConnectionModbusConfig>,
    pub parameters: Vec<FunctionParameter>,
}

pub fn load_connections_config(path_to_config: &str) -> Vec<ConnectionModbusConfig> {
    let configuration = fs::read_to_string(path_to_config.to_string()).unwrap();
    let configs: ClientModbusConfigs = serde_yaml::from_str(&configuration.as_str()).unwrap();
    let vec_client_modbus_config = configs.connection_configs;
    return vec_client_modbus_config;
}

pub fn json_connection_config() -> Vec<ConnectionModbusConfig> {
    let path = String::from("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_config.yaml");
    let configs = load_connections_config(&path);
    return configs;
}

pub fn load_param_configs(path_to_config: &str) -> Vec<FunctionParameter> {
    let configuration = fs::read_to_string(path_to_config.to_string()).unwrap();
    let configs: ClientModbusConfigs = serde_yaml::from_str(&configuration.as_str()).unwrap();
    let parameters = configs.parameters;
    return parameters;
}

pub fn sort_fn_params() -> Vec<Vec<FunctionParameter>> {
    let vec_fn_params =
        load_param_configs("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_config.yaml");

    let mut vec_do_fn_params: Vec<FunctionParameter> = Vec::new();
    let mut vec_di_fn_params: Vec<FunctionParameter> = Vec::new();
    let mut vec_ao_fn_params: Vec<FunctionParameter> = Vec::new();
    let mut vec_ai_fn_params: Vec<FunctionParameter> = Vec::new();

    for fn_param in vec_fn_params {
        if fn_param.mstorage == "DO" {
            vec_do_fn_params.push(fn_param);
        } else if fn_param.mstorage == "DI" {
            vec_di_fn_params.push(fn_param);
        } else if fn_param.mstorage == "AO" {
            vec_ao_fn_params.push(fn_param);
        } else if fn_param.mstorage == "AI" {
            vec_ai_fn_params.push(fn_param);
        } else {
            println!("Error mstorage in config file.")
        }
    }
    vec_do_fn_params.sort_by(|a, b| a.start_address.cmp(&b.start_address));
    vec_di_fn_params.sort_by(|a, b| a.start_address.cmp(&b.start_address));
    vec_ao_fn_params.sort_by(|a, b| a.start_address.cmp(&b.start_address));
    vec_ai_fn_params.sort_by(|a, b| a.start_address.cmp(&b.start_address));

    let mut sorted_vec_fn_params: Vec<Vec<FunctionParameter>> = Vec::new();
    sorted_vec_fn_params.push(vec_do_fn_params);
    sorted_vec_fn_params.push(vec_di_fn_params);
    sorted_vec_fn_params.push(vec_ao_fn_params);
    sorted_vec_fn_params.push(vec_ai_fn_params);

    return sorted_vec_fn_params;
}

pub fn change_start_addr_count(params_store: &Vec<FunctionParameter>) -> Vec<(i32, i32)> {
    let mut start_addrs = Vec::new();
    for fn_param in params_store {
        start_addrs.push(fn_param.start_address);
    }
    let mut addrs_vec = Vec::new();
    for fn_param in params_store {
        addrs_vec.push(fn_param.start_address);
    }
    if addrs_vec.is_empty() {
        println!("Вектор пуст.");
    }

    let mut count = 1;
    let mut first_element = addrs_vec[0];
    let mut result = Vec::new();

    for i in 1..addrs_vec.len() {
        if addrs_vec[i] == addrs_vec[i - 1] + 1 {
            count += 1;
        } else {
            result.push((first_element, count));
            count = 1;
            first_element = addrs_vec[i];
        }
    }
    result.push((first_element, count));

    return result;
}

pub fn get_all_tasks() -> Vec<Vec<String>> {
    let params_store = sort_fn_params();
    // println!("{:?}", params_store);
    let do_params_store = params_store.get(0).unwrap();
    let di_params_store = params_store.get(1).unwrap();
    let ao_params_store = params_store.get(2).unwrap();
    let ai_params_store = params_store.get(3).unwrap();

    let new_do_param = change_start_addr_count(do_params_store);
    let new_di_param = change_start_addr_count(di_params_store);
    let new_ao_param = change_start_addr_count(ao_params_store);
    let new_ai_param = change_start_addr_count(ai_params_store);

    let mut id = 0;

    let do_unit_id = do_params_store[0].unit_id;
    let mut do_tasks_vec: Vec<String> = Vec::new();
    for params in &new_do_param {
        let (start_addr, count) = params;
        id += 1;
        let read_coil_status = format!(
            r#"{{"id":{},"unit":{},"operation":"ReadCoilStatus","address":{},"count":{}}}"#,
            id, do_unit_id, start_addr, count
        );
        do_tasks_vec.push(read_coil_status);
    }

    let di_unit_id = di_params_store[0].unit_id;
    let mut di_tasks_vec: Vec<String> = Vec::new();
    for params in &new_di_param {
        let (start_addr, count) = params;
        id += 1;
        let read_input_status = format!(
            r#"{{"id":{},"unit":{},"operation":"ReadInputStatus","address":{},"count":{}}}"#,
            id, di_unit_id, start_addr, count
        );
        di_tasks_vec.push(read_input_status);
    }

    let ao_unit_id = ao_params_store[0].unit_id;
    let mut ao_tasks_vec: Vec<String> = Vec::new();
    for params in &new_ao_param {
        let (start_addr, count) = params;
        id += 1;
        let read_holding_registers = format!(
            r#"{{"id":{},"unit":{},"operation":"ReadHoldingRegisters","address":{},"count":{}}}"#,
            id, ao_unit_id, start_addr, count
        );
        ao_tasks_vec.push(read_holding_registers);
    }

    let ai_unit_id = ai_params_store[0].unit_id;
    let mut ai_tasks_vec: Vec<String> = Vec::new();
    for params in &new_ai_param {
        let (start_addr, count) = params;
        id += 1;
        let read_input_registers = format!(
            r#"{{"id":{},"unit":{},"operation":"ReadInputRegisters","address":{},"count":{}}}"#,
            id, ai_unit_id, start_addr, count
        );
        ai_tasks_vec.push(read_input_registers);
    }

    let mut all_tasks_json = Vec::new();
    all_tasks_json.push(do_tasks_vec);
    all_tasks_json.push(di_tasks_vec);
    all_tasks_json.push(ao_tasks_vec);
    all_tasks_json.push(ai_tasks_vec);

    return all_tasks_json;
}
