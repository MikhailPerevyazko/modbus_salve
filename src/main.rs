mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use registers_map::Coils;
use rmodbus::{client::ModbusRequest, ModbusProto};
use transport_tcp::TransportCommand;

fn main() {
    modbus_commands();
}

pub fn parse_type_storage(map: Coils) -> String {
    map.type_storage
}

pub fn parse_parameters_type(map: Coils) -> String {
    map.parameters_type
}

pub fn modbus_commands() {
    // Получаем карту регистров по имени "Voltage" и парсим type storage, parameters_type
    let find_param_name = String::from("Power");

    let map_coils = registers_map::call_to_reg_map(find_param_name);

    let type_store = parse_type_storage(map_coils.clone());
    let param_type = parse_parameters_type(map_coils.clone());

    //  Подключение по TCP и создание объекта запроса
    let mut stream = config::transport_tcp().connect();
    let mut mreq = ModbusRequest::new(map_coils.unit_id, ModbusProto::TcpUdp);

    // Команды Modbus в зависиомости от type storage
    if type_store == "DO" {
        modbus::set_coils(&mut stream, &mut mreq, map_coils.start_address);
        modbus::parse_status_coils(&mut stream, &mut mreq, map_coils.start_address, param_type);
        modbus::set_coil();
    } else if type_store == "DI" {
        todo!()
    } else if type_store == "AI" {
        todo!()
    } else if type_store == "AO" {
        todo!()
    } else {
        println!("This type storage is wrong!")
    }
}
