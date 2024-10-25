use registers_map::Coils;
use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::registers_map;

pub fn parse_type_storage(map: Coils) -> String {
    map.type_storage
}

pub fn parse_parameters_type(map: Coils) -> String {
    map.parameters_type
}

pub fn set_coils(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    // Создаем вектор запроса
    let mut request: Vec<u8> = Vec::new();

    //? Установим значения для coil
    mreq.generate_set_coils_bulk(reg, &[true, true, true, true, true], &mut request)
        .unwrap();

    println!("Запрос на запись койлов: {request:?}");

    // Записываем запрос в поток
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
        println!("Ответ на запись койлов: {response:?}\n");
    }

    // Проверяем на наличие ошибок modbus coils
    mreq.parse_ok(&response).unwrap();
}

pub fn parse_status_coils(
    stream: &mut TcpStream,
    mreq: &mut ModbusRequest,
    reg: u16,
    param_type: String,
) {
    // Получаем (читаем) состояние койлов
    let mut request: Vec<u8> = Vec::new();

    mreq.generate_get_coils(reg, 10, &mut request).unwrap();
    stream.write_all(&request).unwrap();
    println!("Запрос на запись состояния койлов: {request:?}");

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

    println!("Ответ на состояние койлов: {response:?}\n");
    mreq.parse_ok(&response).unwrap();

    // В зависимости от parameters_type в yaml парсим состояние регистров
    let mut data = Vec::new();

    if param_type == "bool" {
        // Парсим состояние койлов
        mreq.parse_bool(&response, &mut data).unwrap();
        for (i, c) in data.iter().enumerate() {
            println!("Coil #{i:?} - {c:?}");
        }
    } else if param_type == "i32" {
        todo!()
    } else if param_type == "f32" {
        todo!()
    }
}

pub fn set_coil() {
    let mut stream = TcpStream::connect("127.0.0.1:5500").unwrap();
    let mut mreq = ModbusRequest::new(2, ModbusProto::TcpUdp);
    let mut request: Vec<u8> = Vec::new();

    mreq.generate_set_coil(6, true, &mut request).unwrap();
    stream.write_all(&request).unwrap();

    println!("Запрос на запись одного койла: {:?}", request);
}
