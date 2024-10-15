use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn set_coils(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    // Создаем вектор запроса
    let mut request: Vec<u8> = Vec::new();
    // Установим значения для coil
    mreq.generate_set_coils_bulk(
        reg,
        &[false, true, true, true, true, true, true, true, true, false],
        &mut request,
    )
    .unwrap();

    println!("Запрос на запись койлов: {:?}", request);

    // Записываем запрос в потко
    stream.write_all(&request).unwrap();

    // Получаем заголовок ответа из потока
    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();

    let mut response = Vec::new();
    response.extend_from_slice(&buf);

    // Читаем хвост ответа
    let head = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if head > 6 {
        let mut tail = vec![0u8; (head - 6) as usize];
        stream.read_exact(&mut tail).unwrap();
        response.extend(tail);
        println!("Ответ на запись койлов: {:?}\n", response);
    }

    // Проверяем на наличие ошибок modbus coils
    mreq.parse_ok(&response).unwrap();
}

pub fn parse_status_coils(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    let mut request: Vec<u8> = Vec::new();
    // Получаем состояние койлов
    mreq.generate_get_coils(reg, 10, &mut request).unwrap();
    stream.write_all(&request).unwrap();
    println!("Запрос на запись состояния койлов: {:?}", request);

    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();

    let mut response = Vec::new();
    response.extend_from_slice(&buf);
    let head = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if head > 6 {
        let mut tail = vec![0u8; (head - 6) as usize];
        stream.read_exact(&mut tail).unwrap();
        response.extend(tail);
    }
    println!("Ответ на состояние койлов: {:?}\n", response);
    mreq.parse_ok(&response).unwrap();

    let mut data = Vec::new();

    // Парсим состояние койлов
    mreq.parse_bool(&response, &mut data).unwrap();
    for (i, c) in data.iter().enumerate() {
        println!("Coil #{} - {}", i, c);
    }
}
