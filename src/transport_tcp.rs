use std::net::TcpStream;

pub trait TransportCommand {
    fn connect(&mut self) -> TcpStream;
}

#[derive(Debug)]
pub struct TransportTCP {
    pub host_port: String,
    pub connected: bool,
}

impl TransportCommand for TransportTCP {
    fn connect(&mut self) -> TcpStream {
        if self.connected {
            todo!()
        } else {
            // Устанавливаем соединение
            let stream = TcpStream::connect(&self.host_port).unwrap();
            println!("Соединение с устройством {} установлено.\n", self.host_port);
            stream
        }
    }
}
