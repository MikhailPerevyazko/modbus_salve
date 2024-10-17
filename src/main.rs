mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use registers_map::Coils;
use rmodbus::{client::ModbusRequest, ModbusProto};
use transport_tcp::TransportCommand;

pub fn parse_type_storage(map: Coils) -> String {
    let type_storage = map.type_storage;
    type_storage
}

fn main() {
    //? Получаем карту регистров по имени "Temp" и парсим type storage
    let map_coils = registers_map::call_to_reg_map("Voltage".to_string());
    let type_store = parse_type_storage(map_coils.clone());

    //? Подключение по TCP и создание объекта запроса
    let mut stream = config::transport_tcp().connect();
    let mut mreq = ModbusRequest::new(map_coils.unit_id, ModbusProto::TcpUdp);

    //? Команды Modbus
    if type_store == "DO" {
        modbus::set_coils(&mut stream, &mut mreq, map_coils.start_address);
        modbus::parse_status_coils(&mut stream, &mut mreq, map_coils.start_address);
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
