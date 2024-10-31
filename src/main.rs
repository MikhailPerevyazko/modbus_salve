mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use transport_tcp::TransportCommand;

fn main() {
    let stream = config::transport_tcp().connect();
    let find_param_name_one = String::from("Set coils");
    modbus::modbus_commands(find_param_name_one, stream);

    println!("_________________________________________");

    let stream_two = config::transport_tcp().connect();
    let find_param_name_two = String::from("Set holdings");
    modbus::modbus_commands(find_param_name_two, stream_two);
}
