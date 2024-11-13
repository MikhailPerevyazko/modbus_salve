use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::registers_map;

pub fn modbus_commands(find_param_name: String, mut stream: TcpStream) {
    let map = registers_map::call_to_reg_map(find_param_name);
    let mut mreq = ModbusRequest::new(map.unit_id, ModbusProto::TcpUdp);

    let type_storage = map.type_storage;

    if type_storage == "DO" {
        write_coils(&mut stream, &mut mreq, map.start_address);
        read_coils(
            &mut stream,
            &mut mreq,
            map.start_address,
            map.parameters_type,
        );
    } else if type_storage == "DI" {
        read_input_status(&mut stream, &mut mreq, map.start_address);
    } else if type_storage == "AO" {
        write_hoildings(&mut stream, &mut mreq, map.start_address);
        read_holdings(&mut stream, &mut mreq, map.start_address);
    } else if type_storage == "AI" {
        read_input_registers(&mut stream, &mut mreq, map.start_address);
    } else {
        println!("Error!")
    }
}

pub fn write_coils(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
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

pub fn read_coils(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16, param_type: String) {
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
    let mut result = Vec::new();
    if param_type == "bool" {
        // Парсим состояние койлов
        mreq.parse_bool(&response, &mut result).unwrap();
        for (i, c) in result.iter().enumerate() {
            println!("Coil #{:?} - {:?}", i, c);
        }
    } else if param_type == "i32" {
        todo!()
    } else if param_type == "f32" {
        todo!()
    }
}

pub fn read_input_status(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    let mut request: Vec<u8> = Vec::new();
    mreq.generate_get_discretes(reg, 10, &mut request).unwrap();
    stream.write_all(&request).unwrap();
    println!("Запрос на чтение статуса входных инпутов: {:?}", request);

    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();

    let mut response = Vec::new();
    response.extend_from_slice(&buf);

    let head = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if head > 6 {
        let mut tail = vec![0u8; (head - 6) as usize];
        stream.read_exact(&mut tail).unwrap();
        response.extend(tail)
    }

    println!("Ответ на состояние входных инпутов: {:?}\n", response);

    let mut result = Vec::new();
    mreq.parse_bool(&response, &mut result).unwrap();
    for (num, value) in result.iter().enumerate() {
        println!("Input status #{:?} - {:?}", num, value)
    }
}

pub fn write_hoildings(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
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

pub fn read_holdings(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
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

pub fn read_input_registers(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    let mut request = Vec::new();
    mreq.generate_get_inputs(reg, 10, &mut request).unwrap();
    stream.write_all(&request).unwrap();
    println!("Запрос на чтение регистров входа: {:?}", request);

    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();

    let mut response = Vec::new();
    response.extend_from_slice(&buf);

    let head = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if head > 6 {
        let mut tail = vec![0u8; (head - 6) as usize];
        stream.read_exact(&mut tail).unwrap();
        response.extend(tail)
    }
    println!("Ответ на состояние регистров входа: {:?}\n", response);

    let mut result = Vec::new();
    mreq.parse_u16(&response, &mut result).unwrap();
    for (num, value) in result.iter().enumerate() {
        println!("Input register #{:?} - {:?}", num, value)
    }
}
