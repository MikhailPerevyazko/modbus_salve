use registers_map::Coils;
use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::registers_map;

pub fn modbus_commands(find_param_name: String, mut stream: TcpStream) {
    let map_coils = registers_map::call_to_reg_map(find_param_name);
    //  Создание объекта запроса
    let mut mreq = ModbusRequest::new(map_coils.unit_id, ModbusProto::TcpUdp);
    // Команды Modbus в зависиомости от type storage
    let type_store = parse_type_storage(map_coils.clone());
    let param_type = parse_parameters_type(map_coils.clone());

    if type_store == "DO" {
        set_coils(&mut stream, &mut mreq, map_coils.start_address);
        parse_status_coils(&mut stream, &mut mreq, map_coils.start_address, param_type);
    } else if type_store == "DI" {
        todo!()
    } else if type_store == "AI" {
        todo!()
    } else if type_store == "AO" {
        set_hoildings(&mut stream, &mut mreq, map_coils.start_address);
        parse_status_holdings(&mut stream, &mut mreq, map_coils.start_address);
    } else {
        println!("This type storage is wrong!")
    }
}

pub fn parse_type_storage(map: Coils) -> String {
    map.type_storage
}

pub fn parse_parameters_type(map: Coils) -> String {
    map.parameters_type
}

pub fn set_coils(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    // Создаем вектор запроса
    let mut request: Vec<u8> = Vec::new();

    // Установим значения для coil
    mreq.generate_set_coils_bulk(
        reg,
        &[true, true, true, true, true, true, true, true, true, true],
        &mut request,
    )
    .unwrap();

    println!("Запрос на запись койлов: {:?}", request);

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
        println!("Ответ на запись койлов: {:?}\n", response);
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

    // В зависимости от parameters_type в yaml парсим состояние регистров
    let mut data = Vec::new();

    if param_type == "bool" {
        // Парсим состояние койлов
        mreq.parse_bool(&response, &mut data).unwrap();
        for (i, c) in data.iter().enumerate() {
            println!("Coil #{:?} - {:?}", i, c);
        }
    } else if param_type == "i32" {
        todo!()
    } else if param_type == "f32" {
        todo!()
    }
}

pub fn set_hoildings(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    let mut request: Vec<u8> = Vec::new();

    mreq.generate_set_holdings_bulk(reg, &[10, 10, 10, 10, 10, 10, 10, 10, 10, 10], &mut request)
        .unwrap();
    println!("Запрос на запись холдингов: {:?}", request);

    stream.write_all(&request).unwrap();

    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();

    let mut response: Vec<u8> = Vec::new();
    response.extend_from_slice(&buf);

    let head = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if head > 6 {
        let mut tail = vec![0u8; (head - 6) as usize];
        stream.read_exact(&mut tail).unwrap();
        response.extend(tail);
        println!("Ответ на запись холдингов: {:?}\n", response);
    }
}

pub fn parse_status_holdings(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    // Запрос на состояние холдингов
    let mut request: Vec<u8> = Vec::new();
    mreq.generate_get_holdings(reg, 10, &mut request).unwrap();
    stream.write_all(&request).unwrap();

    println!("Запрос на запись состояния холдингов: {:?}", request);

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

    println!("Ответ на состояние холдингов: {:?}\n", response);

    let mut result: Vec<u16> = Vec::new();
    mreq.parse_u16(&response, &mut result).unwrap();
    for (num, value) in result.iter().enumerate() {
        println!("Holding #{:?} - {:?}", num, value)
    }
}
