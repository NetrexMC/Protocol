use super::*;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use binary_utils::*;

macro_rules! write_packet {
     ($id: literal, $packet: expr) => {
         {
             let mut buffer = Vec::new();
             buffer.write_u8($id)?;
             let mut pk = $packet.parse()?;
             buffer.append(&mut pk);
             Ok(buffer)
         }
     };
     ($id: literal) => {
         {
             let mut buffer = Vec::new();
             buffer.write_u8($id)?;
             Ok(buffer)
         }
     };
 }

#[derive(Debug, Clone)]
pub enum GamePacket {
    Login(Login),
    ServerToClientHandshake(ServerToClientHandshake),
    ClientToServerHandshake(ClientToServerHandshake),
    PlayStatus(PlayStatus),
    Disconnect(Disconnect),
    BehaviorPackInfo(BehaviorPackInfo),
    TexturePackInfo(TexturePackInfo),
    ResourcePackInfo(ResourcePackInfo),
    Unknown(Vec<u8>),
}

impl GamePacket {
     pub fn write(&self) -> Result<Vec<u8>, binary_utils::error::BinaryError> {
          match self.clone() {
               GamePacket::Login(p) => write_packet!(0x01, p),
               GamePacket::PlayStatus(p) => write_packet!(0x02, p),
               GamePacket::ServerToClientHandshake(p) => write_packet!(0x03, p),
               GamePacket::ClientToServerHandshake(p) => write_packet!(0x04, p),
               GamePacket::Disconnect(p) => write_packet!(0x05, p),
               GamePacket::ResourcePackInfo(p) => write_packet!(0x06, p),
               GamePacket::BehaviorPackInfo(p) => write_packet!(0x07, p),
               GamePacket::TexturePackInfo(p) => write_packet!(0x08, p),
               GamePacket::Unknown(p) => Ok(p.clone()),
          }
     }
}

pub fn construct_packet(id: u8, buffer: &[u8]) -> GamePacket {
    match id {
        0x01 => GamePacket::Login(Login::fcompose(buffer, &mut 0)),
        0x02 => GamePacket::PlayStatus(PlayStatus::fcompose(buffer, &mut 0)),
        0x03 => {
            GamePacket::ServerToClientHandshake(ServerToClientHandshake::fcompose(buffer, &mut 0))
        }
        0x04 => {
            GamePacket::ClientToServerHandshake(ClientToServerHandshake::fcompose(buffer, &mut 0))
        }
        0x05 => GamePacket::Disconnect(Disconnect::fcompose(buffer, &mut 0)),
        0x06 => GamePacket::ResourcePackInfo(ResourcePackInfo::fcompose(buffer, &mut 0)),
        _ => GamePacket::Unknown(buffer.to_vec()),
    }
}
