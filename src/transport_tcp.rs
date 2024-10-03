// use rmodbus::ModbusProto;
use std::{error::Error, net::TcpStream};

pub trait TransportCommand {
    // fn modbus_proto(&self) -> ModbusProto;
    fn connect(&mut self) -> Result<(), Box<dyn Error>>;
    // fn disconnect(&mut self);
}

#[derive(Debug)]
pub struct TransportTCP {
    pub host_port: String,
    pub connected: bool,
    pub stream: Option<Box<TcpStream>>,
}

impl TransportCommand for TransportTCP {
    // fn modbus_proto(&self) -> ModbusProto {
    //     ModbusProto::TcpUdp
    // }

    fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        if self.connected {
            return Ok(());
        } else {
            // Устанавливаем соединение
            let stream = TcpStream::connect(&self.host_port)?;

            self.stream = Some(Box::new(stream));
            self.connected = true;
            println!("Соединение с устройством {} установлено", self.host_port);
            Ok(())
        }
    }
    // fn disconnect(&mut self) {
    //     self.connected = false;
    //     self.stream = None;
    //     println!("Соединение с устройством {} отключено", self.host_port);
    // }
}
