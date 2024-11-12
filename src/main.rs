mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use modbus::modbus_commands;
use transport_tcp::TransportCommand;

fn main() {
    do_modbus("Write coils".to_string());
    do_modbus("Read inputs".to_string());
    do_modbus("Write holdings".to_string());
    do_modbus("Read registers".to_string());
}
pub fn do_modbus(param: String) {
    let stream = config::transport_tcp().connect();
    modbus_commands(param.to_string(), stream);
}
