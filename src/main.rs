mod config;
mod connect;

use config::{json_connection_config, print_parameters_json};
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

    print!("{:#?}", print_parameters_json());
}
