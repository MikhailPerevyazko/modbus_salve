mod cmd;
mod config;
mod modbus;
mod registers_map;
mod transport_tcp;

use rmodbus::{client::ModbusRequest, ModbusProto};
use transport_tcp::TransportCommand;

fn main() {
    //? Получаем карту регистров по имени "Temp"
    let map_coils = registers_map::call_to_reg_map("Temp".to_string());
    println!("{:#?}", map_coils);

    //? Подключение по TCP и создание объекта запроса
    let mut stream = config::transport_tcp().connect();
    let mut mreq = ModbusRequest::new(map_coils.unit_id, ModbusProto::TcpUdp);

    //? Команды modbus
    if map_coils.type_storage == "DO" {
        modbus::set_coils(&mut stream, &mut mreq, map_coils.start_address);
        modbus::parse_status_coils(&mut stream, &mut mreq, map_coils.start_address);
    } else if map_coils.type_storage == "DI" {
        todo!()
    } else if map_coils.type_storage == "AI" {
        todo!()
    } else if map_coils.type_storage == "AO" {
        todo!()
    } else {
        todo!()
    }
}
