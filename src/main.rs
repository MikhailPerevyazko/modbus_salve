mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use transport_tcp::TransportCommand;

fn main() {
    // let param = String::from("Set coils");s
    // let stream = config::transport_tcp().connect();

    // if param == "Set coils" {
    //     modbus::modbus_commands(param, stream);
    // } else if param == "Set holdings" {
    //     modbus::modbus_commands(param, stream);
    // } else if param == "Get inputs" {
    //     modbus::modbus_commands(param, stream);
    // }

    let stream_one = config::transport_tcp().connect();
    let param_one = String::from("Set coils");
    modbus::modbus_commands(param_one, stream_one);

    let stream_two = config::transport_tcp().connect();
    let param_two = String::from("Get inputs");
    modbus::modbus_commands(param_two, stream_two);

    let stream_three = config::transport_tcp().connect();
    let param_three = String::from("Set holdings");
    modbus::modbus_commands(param_three, stream_three);

    let stream_four = config::transport_tcp().connect();
    let param_four = String::from("Get discrets");
    modbus::modbus_commands(param_four, stream_four);
}
