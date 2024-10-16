mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use registers_map::Coils;
use rmodbus::{client::ModbusRequest, ModbusProto};
use transport_tcp::TransportCommand;

fn main() {
    //? Получаем карту регистров по имени "Temp"
    let map_coils = registers_map::call_to_reg_map("Voltage".to_string());

    //? Подключение по TCP и создание объекта запроса
    let mut stream = config::transport_tcp().connect();
    let mut mreq = ModbusRequest::new(map_coils.unit_id, ModbusProto::TcpUdp);

    let type_storage = parse_type_storage(map_coils.clone());

    //? Команды Modbus
    if type_storage == "DO" {
        modbus::set_coils(&mut stream, &mut mreq, map_coils.start_address);
        modbus::parse_status_coils(&mut stream, &mut mreq, map_coils.start_address);
    } else if map_coils.type_storage == "DI" {
        println!("nothing");
    } else if map_coils.type_storage == "AI" {
        println!("nothing");
    } else if map_coils.type_storage == "AO" {
    } else {
        println!("This coil's name doesn't exists!")
    }
}

pub fn parse_type_storage(map_coils: Coils) -> String {
    let type_storage = map_coils.type_storage;
    type_storage
}
