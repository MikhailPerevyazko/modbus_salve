mod config;
mod connect;

use config::{json_connection_config, load_configs};
use connect::conneting;
use rmodbus_client::ModBusClient;
fn main() {
    let vec_json_config = json_connection_config();

    let client_tcp = ModBusClient::new();
    let client_udp = ModBusClient::new();
    let client_rtu = ModBusClient::new();
    let mut vec_client: Vec<ModBusClient> = Vec::new();

    vec_client.push(client_tcp);
    vec_client.push(client_udp);
    vec_client.push(client_rtu);
    conneting(vec_client, vec_json_config);

    let vec_parameters =
        load_configs("/home/Mikhail/projects/try_to_modbus/modbus_tcp/modbus_config.yaml");
    for parameters in vec_parameters {
        if parameters.mstorage == "DI" {
            println!("this is DI");
        } else if parameters.mstorage == "DO" {
            println!("this is DO");
        } else if parameters.mstorage == "AI" {
            println!("this is AI");
        } else if parameters.mstorage == "AO" {
            println!("this is AO");
        }
    }
}
