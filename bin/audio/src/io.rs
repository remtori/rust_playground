use std::{
    io::{Read, Write},
    net::UdpSocket,
};

pub struct UdpReader {
    socket: UdpSocket,
}

impl UdpReader {
    pub fn new(socket: UdpSocket) -> Self {
        Self { socket }
    }
}

impl Read for UdpReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.socket.recv_from(buf).map(|(data, _addr)| data)
    }
}

pub struct UdpWriter {
    socket: UdpSocket,
}

impl UdpWriter {
    pub fn new(socket: UdpSocket) -> Self {
        Self { socket }
    }
}

impl Write for UdpWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.socket.send(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
