use super::*;
use binary_utils::*;

pub trait PacketId {
    fn id() -> u8;

    fn get_id(&self) -> u8 {
        Self::id()
    }
}

#[derive(Debug, Clone, BinaryStream)]
pub struct Packet {
    pub id: u8,
    pub kind: PacketKind,
}

#[derive(Debug, Clone)]
pub enum PacketKind {
    Login(Login),
    ServerToClientHandshake(ServerToClientHandshake),
    ClientToServerHandshake(ClientToServerHandshake),
    PlayStatus(PlayStatus),
    Disconnect(Disconnect),
    ResourcePackInfo(ResourcePackInfo),
    Unknown(Vec<u8>),
}

impl PacketKind {
    pub fn get_id(&self) -> u8 {
        // get the inner value
        match self {
            PacketKind::Login(x) => x.get_id(),
            PacketKind::ServerToClientHandshake(x) => x.get_id(),
            PacketKind::ClientToServerHandshake(x) => x.get_id(),
            PacketKind::PlayStatus(x) => x.get_id(),
            PacketKind::Disconnect(x) => x.get_id(),
            PacketKind::ResourcePackInfo(x) => x.get_id(),
            PacketKind::Unknown(x) => 0,
        }
    }
}

impl Into<Packet> for PacketKind {
    fn into(self) -> Packet {
        Packet {
            id: self.get_id(),
            kind: self,
        }
    }
}

impl Streamable for PacketKind {
    fn parse(&self) -> Result<Vec<u8>, binary_utils::error::BinaryError> {
        match self.clone() {
            PacketKind::Login(p) => p.parse(),
            PacketKind::PlayStatus(p) => p.parse(),
            PacketKind::ServerToClientHandshake(p) => p.parse(),
            PacketKind::ClientToServerHandshake(p) => p.parse(),
            PacketKind::Disconnect(p) => p.parse(),
            PacketKind::ResourcePackInfo(p) => p.parse(),
            PacketKind::Unknown(p) => Ok(p.clone()),
        }
    }

    /// Compose is picky here, it expects the packet id to be the first byte
    /// So when composing, we need to prepend the packet id
    fn compose(source: &[u8], position: &mut usize) -> Result<Self, error::BinaryError>
    where
        Self: Sized,
    {
        let id = source[*position];
        let local = *position + 1;
        if let Ok(res) = construct_packet(id, &source[local..]) {
            *position += res.parse()?.len();
            return Ok(res);
        } else {
            return Err(binary_utils::error::BinaryError::RecoverableKnown(
                "Packet is not a gamepacket".into(),
            ));
        }
    }
}

pub fn construct_packet(id: u8, buffer: &[u8]) -> Result<PacketKind, error::BinaryError> {
    match id {
        x if x == Login::id() => Ok(PacketKind::Login(Login::compose(buffer, &mut 0)?)),
        x if x == ServerToClientHandshake::id() => Ok(PacketKind::ServerToClientHandshake(
            ServerToClientHandshake::compose(buffer, &mut 0)?,
        )),
        x if x == ClientToServerHandshake::id() => Ok(PacketKind::ClientToServerHandshake(
            ClientToServerHandshake::compose(buffer, &mut 0)?,
        )),
        x if x == PlayStatus::id() => {
            Ok(PacketKind::PlayStatus(PlayStatus::compose(buffer, &mut 0)?))
        }
        x if x == Disconnect::id() => {
            Ok(PacketKind::Disconnect(Disconnect::compose(buffer, &mut 0)?))
        }
        x if x == ResourcePackInfo::id() => Ok(PacketKind::ResourcePackInfo(
            ResourcePackInfo::compose(buffer, &mut 0)?,
        )),
        _ => Err(error::BinaryError::RecoverableKnown(
            "Packet is not a gamepacket".into(),
        )),
    }
}
