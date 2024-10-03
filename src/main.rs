mod cmd;
mod config;
mod modbus;
mod transport_tcp;
use transport_tcp::TransportCommand;

fn main() {
    //? Подключение по TCP
    let mut tcp = config::transport_tcp();
    tcp.connect().unwrap();

    //? Пробуем поулчить данные в slave-устройства
    let req = modbus::make_req_to_mb().unwrap();
    println!("{:?}", req);
}
