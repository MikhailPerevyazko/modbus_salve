mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use std::net::TcpStream;

use rmodbus::{client::ModbusRequest, ModbusProto};
use transport_tcp::TransportCommand;

fn main() {
    // Подключение по TCP
    let stream = config::transport_tcp().connect();
    // Получаем карту регистров по имени "Voltage" и парсим type storage, parameters_type
    let find_param_name_one = String::from("Set coils");
    modbus_commands(find_param_name_one, stream);

    let stream_two = config::transport_tcp().connect();
    let find_param_name_two = String::from("Set holdings");
    modbus_commands(find_param_name_two, stream_two);
}

pub fn modbus_commands(find_param_name: String, mut stream: TcpStream) {
    let map_coils = registers_map::call_to_reg_map(find_param_name);

    let type_store = modbus::parse_type_storage(map_coils.clone());
    let param_type = modbus::parse_parameters_type(map_coils.clone());

    //  Создание объекта запроса
    let mut mreq = ModbusRequest::new(map_coils.unit_id, ModbusProto::TcpUdp);

    // Команды Modbus в зависиомости от type storage
    if type_store == "DO" {
        modbus::set_coils(&mut stream, &mut mreq, map_coils.start_address);
        modbus::parse_status_coils(&mut stream, &mut mreq, map_coils.start_address, param_type);
    } else if type_store == "DI" {
        todo!()
    } else if type_store == "AI" {
        todo!()
    } else if type_store == "AO" {
        modbus::set_hoildings(&mut stream, &mut mreq, map_coils.start_address);
    } else {
        println!("This type storage is wrong!")
    }
}
