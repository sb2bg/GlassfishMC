use crate::server::packet::Packet;

pub struct HandshakePacket {
    pub protocol_version: u32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: u8,
}

impl HandshakePacket {
    pub fn new(protocol_version: u32, server_address: String, server_port: u16, next_state: u8) -> Self {
        Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        }
    }
}

impl Packet for HandshakePacket {
    fn get_id(&self) -> u8 {
        0x00
    }

    fn get_size(&self) -> usize {
        self.server_address.len() + self.server_port.to_string().len() + 3
    }
}
