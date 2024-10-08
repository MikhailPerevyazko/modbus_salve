mod cmd;
mod config;
mod transport_tcp;

use rmodbus::{client::ModbusRequest, ModbusProto};
use std::{
    io::{Read, Write},
    net::TcpStream,
};
use transport_tcp::TransportCommand;

fn main() {
    // Подключение по TCP
    let mut stream = config::transport_tcp().connect();
    // Создаем объект запроса
    let mut mreq = ModbusRequest::new(1, ModbusProto::TcpUdp);

    modbus(&mut stream, &mut mreq);
}

fn modbus(stream: &mut TcpStream, mreq: &mut ModbusRequest) {
    // Создаем вектор запроса
    let mut request: Vec<u8> = Vec::new();

    // Установим значения для coil
    mreq.generate_set_coils_bulk(
        0,
        &[true, true, true, true, true, true, true, true, true, true],
        &mut request,
    )
    .unwrap();

    // Записываем запрос в stream
    stream.write(&request).unwrap();
    println!("Request: {:?}", request);

    // Считываем 12 байтов ответа в буфер
    let mut buf = [0u8; 12];
    stream.read_exact(&mut buf).unwrap();

    let mut response = Vec::new();
    response.extend_from_slice(&buf);
    println!("Response: {:?}", response);

    // Проверим ошибки внутри модбаса
    mreq.parse_ok(&request).unwrap();
    mreq.parse_ok(&response).unwrap();
}
