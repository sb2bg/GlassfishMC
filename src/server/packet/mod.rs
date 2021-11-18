pub mod handshake_packet;

pub trait Packet {
    fn get_id(&self) -> u8;
    fn get_size(&self) -> usize;
}