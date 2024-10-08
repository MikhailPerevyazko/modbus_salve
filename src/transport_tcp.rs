// use rmodbus::ModbusProto;
use std::net::TcpStream;

pub trait TransportCommand {
    // fn modbus_proto(&self) -> ModbusProto;
    fn connect(&mut self) -> TcpStream;
    // fn disconnect(&mut self);
}

#[derive(Debug)]
pub struct TransportTCP {
    pub host_port: String,
    pub connected: bool,
    // pub stream: Option<TcpStream>,
}

impl TransportCommand for TransportTCP {
    // fn modbus_proto(&self) -> ModbusProto {
    //     ModbusProto::TcpUdp
    // }

    fn connect(&mut self) -> TcpStream {
        if self.connected {
            todo!()
        } else {
            // Устанавливаем соединение
            let stream = TcpStream::connect(&self.host_port).unwrap();
            println!("Соединение с устройством {} установлено\n", self.host_port);
            stream
        }
    }
    // fn disconnect(&mut self) {
    //     self.connected = false;
    //     println!("Соединение с устройством {} отключено", self.host_port);
    // }
}
