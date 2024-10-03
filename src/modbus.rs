use rmodbus::{client::ModbusRequest, ErrorKind, ModbusProto};

pub fn make_req_to_mb() -> Result<(), ErrorKind> {
    let unit_id: u8 = 1;
    let proto = ModbusProto::TcpUdp;

    let mut client = ModbusRequest::new(unit_id, proto);

    let mut request: Vec<u8> = Vec::new();
    let count_coils: u16 = 9;
    let first_read_register: u16 = 2;

    let read_coils = client.generate_get_coils(first_read_register, count_coils, &mut request);

    // match try_to_read_coils {
    //     Ok(data) => println!("{:?}", data),
    //     Err(e) => println!("{}", e),
    // }

    return read_coils;
}
