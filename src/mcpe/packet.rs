use super::*;
pub enum GamePacket {
     Login(Login),
     ServerToClientHandshake(ServerToClientHandshake),
     ClientToServerHandshake(ClientToServerHandshake),
     PlayStatus(PlayStatus),
     Disconnect(Disconnect),
     BehaviorPackInfo(BehaviorPackInfo),
     TexturePackInfo(TexturePackInfo),
     ResourcePackInfo(ResourcePackInfo),
     Unknown(Vec<u8>)
}

pub fn construct_packet(id: u8, buffer: &[u8]) -> GamePacket {
     match id {
          0x01 => GamePacket::Login(Login::fcompose(buffer, &mut 0)),
          0x02 => GamePacket::PlayStatus(PlayStatus::fcompose(buffer, &mut 0)),
          0x03 => GamePacket::ServerToClientHandshake(ServerToClientHandshake::fcompose(buffer, &mut 0)),
          0x04 => GamePacket::ClientToServerHandshake(ClientToServerHandshake::fcompose(buffer, &mut 0)),
          0x05 => GamePacket::Disconnect(Disconnect::fcompose(buffer, &mut 0)),
          0x06 => GamePacket::ResourcePackInfo(ResourcePackInfo::fcompose(buffer, &mut 0)),
          _ => GamePacket::Unknown(buffer.to_vec())
     }
}

