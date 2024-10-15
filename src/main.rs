mod cmd;
mod config;
mod registers_map;
mod transport_tcp;

use rmodbus::{client::ModbusRequest, ModbusProto};
use transport_tcp::TransportCommand;
mod modbus;

fn main() {
    // Получаем карту регистров
    let map_coils = registers_map::call_to_reg_map();

    // Подключение по TCP и создание объекта запроса
    let mut stream = config::transport_tcp().connect();
    let mut mreq = ModbusRequest::new(map_coils.unit_id, ModbusProto::TcpUdp);

    // Команды modbus
    modbus::set_coils(&mut stream, &mut mreq, map_coils.start_address);
    modbus::parse_status_coils(&mut stream, &mut mreq, map_coils.start_address);
}
