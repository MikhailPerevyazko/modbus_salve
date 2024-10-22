use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn set_coils(stream: &mut TcpStream, mreq: &mut ModbusRequest, reg: u16) {
    // Создаем вектор запроса
    let mut request: Vec<u8> = Vec::new();

    //? Установим значения для coil
    mreq.generate_set_coils_bulk(
        reg,
        &[
            true, true, true, true, true, false, false, false, false, false,
        ],
        &mut request,
    )
    .unwrap();

    println!("Запрос на запись койлов: {:?}", request);

    //? Записываем запрос в поток
    stream.write_all(&request).unwrap();

    //? Получаем заголовок ответа из потока
    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();

    let mut response = Vec::new();
    response.extend_from_slice(&buf);

    //? Читаем хвост ответа
    let head = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if head > 6 {
        let mut tail = vec![0u8; (head - 6) as usize];
        stream.read_exact(&mut tail).unwrap();
        response.extend(tail);
        println!("Ответ на запись койлов: {:?}\n", response);
    }

    //? Проверяем на наличие ошибок modbus coils
    mreq.parse_ok(&response).unwrap();
}

pub fn parse_status_coils(
    stream: &mut TcpStream,
    mreq: &mut ModbusRequest,
    reg: u16,
    param_type: String,
) {
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

    //? В зависимости от parameters_type в yaml парсим состояние регистров
    if param_type == "bool" {
        // Парсим состояние койлов
        mreq.parse_bool(&response, &mut data).unwrap();
        for (i, c) in data.iter().enumerate() {
            println!("Coil #{} - {}", i, c);
        }
    } else if param_type == "i32" {
        todo!()
    } else if param_type == "f32" {
        todo!()
    }
}

pub fn some_modbus_command(stream: &mut TcpStream, mreq: &mut ModbusRequest) {
    let mut request: Vec<u8> = Vec::new();

    mreq.generate_set_holdings_bulk(0, &[1, 2, 3, 4], &mut request)
        .unwrap();

    println!("Запрос на запись холдингов: {:?}", request);

    //? Записываем запрос в поток
    stream.write_all(&request).unwrap();

    //? Получаем заголовок ответа из потока
    let mut buf = [0u8; 6];
    stream.read_exact(&mut buf).unwrap();

    let mut response = Vec::new();
    response.extend_from_slice(&buf);

    //? Читаем хвост ответа
    let head = guess_response_frame_len(&buf, ModbusProto::TcpUdp).unwrap();
    if head > 6 {
        let mut tail = vec![0u8; (head - 6) as usize];
        stream.read_exact(&mut tail).unwrap();
        response.extend(tail);
        println!("Ответ на запись холдингов: {:?}\n", response);
    }

    //? Проверяем на наличие ошибок modbus coils
    mreq.parse_ok(&response).unwrap();
}
