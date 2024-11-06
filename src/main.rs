mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use transport_tcp::TransportCommand;

fn main() {
    do_modbus("Set coils".to_string());
    do_modbus("Get inputs".to_string());
    do_modbus("Set holdings".to_string());
    do_modbus("Get discrets".to_string());
}

pub fn do_modbus(param: String) {
    let stream = config::transport_tcp().connect();
    modbus::modbus_commands(param.to_string(), stream);
}
