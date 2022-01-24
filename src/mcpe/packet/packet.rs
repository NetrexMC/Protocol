use std::{fmt, io::Write};

use super::*;
use binary_utils::*;

pub trait PacketId {
    fn id() -> u32;

    fn get_id(&self) -> u32 {
        Self::id()
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    /// The actual ID of the packet we're decoding/sending
    pub id: u32,
    /// The kind of packet, this is a wrapper around the actual packet.
    pub kind: PacketKind,
    /// The sender subclient is the co-op player that sent the packet, for instance:
    /// If we have "Steve" on the left side and "Alex" on the right,
    /// This would be "Steve" if Steve sent the packet.
    ///
    /// It is important to note, that netrex ignores this field.
    pub send_sub_client: u8,
    /// The receiver subclient is the co-op player that should receive the packet, for instance:
    /// Using the same example above, this would be "Steve" again, if Steve sent the packet.
    ///
    /// It is important to note, that netrex ignores this field.
    pub target_sub_client: u8,
}

impl Streamable for Packet {
    fn compose(source: &[u8], position: &mut usize) -> Result<Self, error::BinaryError> {
        // we can silently read the id, but we do NOT want to read it
        let kind = PacketKind::compose(&source, position)?;
        Ok(Packet {
            id: kind.get_id(),
            kind,
            send_sub_client: 0,
            target_sub_client: 0,
        })
    }

    fn parse(&self) -> Result<Vec<u8>, error::BinaryError> {
        let mut buf = Vec::new();
        buf.write_all(&VarInt::<u32>(self.id | (0 << 10) | (0 << 12)).parse()?)?;
        buf.write_all(&self.kind.parse()?)?;
        Ok(buf)
    }
}

/// A wrapper for all existing packets.
/// This is used as a "type union".
///
/// You should not instance this struct directly, but rather, instance it with the `into` method.
/// For Example:
/// ```rust
/// use mcpe_protocol::mcpe::packet::*;
/// let packet: Packet = PlayStatus::NotEdu.into(); // Packet
/// ```
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

impl fmt::Display for PacketKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PacketKind::Login(_) => write!(f, "Login"),
            PacketKind::ServerToClientHandshake(_) => write!(f, "ServerToClientHandshake"),
            PacketKind::ClientToServerHandshake(_) => write!(f, "ClientToServerHandshake"),
            PacketKind::PlayStatus(_) => write!(f, "PlayStatus"),
            PacketKind::Disconnect(_) => write!(f, "Disconnect"),
            PacketKind::ResourcePackInfo(_) => write!(f, "ResourcePackInfo"),
            PacketKind::Unknown(_) => write!(f, "Unknown"),
        }
    }
}

impl From<PacketKind> for String {
    fn from(kind: PacketKind) -> Self {
        format!("{}", kind)
    }
}

macro_rules! impl_from_pkind {
    ($($kind: ident),*) => {
        $(
            impl From<$kind> for Packet {
                fn from(kind: $kind) -> Self {
                    Packet {
                        id: kind.get_id(),
                        kind: PacketKind::$kind(kind),
                        send_sub_client: 0,
                        target_sub_client: 0
                    }
                }
            }

            impl From<$kind> for PacketKind {
                fn from(kind: $kind) -> Self {
                    PacketKind::$kind(kind)
                }
            }

            impl From<PacketKind> for $kind {
                fn from(kind: PacketKind) -> Self {
                    match kind {
                        PacketKind::$kind(kind) => kind,
                        _ => panic!("Invalid packet kind"),
                    }
                }
            }
        )*
    };
}
impl_from_pkind! {
    Login,
    ServerToClientHandshake,
    ClientToServerHandshake,
    PlayStatus,
    Disconnect,
    ResourcePackInfo
}

impl PacketKind {
    pub fn get_id(&self) -> u32 {
        // get the inner value
        match self {
            PacketKind::Login(x) => x.get_id(),
            PacketKind::ServerToClientHandshake(x) => x.get_id(),
            PacketKind::ClientToServerHandshake(x) => x.get_id(),
            PacketKind::PlayStatus(x) => x.get_id(),
            PacketKind::Disconnect(x) => x.get_id(),
            PacketKind::ResourcePackInfo(x) => x.get_id(),
            PacketKind::Unknown(_) => 0,
        }
    }
}

impl Into<Packet> for PacketKind {
    fn into(self) -> Packet {
        Packet {
            id: self.get_id(),
            kind: self,
            send_sub_client: 0,
            target_sub_client: 0,
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
        let flags = VarInt::<u32>::compose(source, position)?;
        // todo: This is going to cause problems in the future, but the subclient and subtarget need to
        // todo: be handled
        let id = flags.0 & 0x3ff;
        if let Ok(res) = construct_packet(id, &source[*position..]) {
            *position += res.parse()?.len();
            return Ok(res);
        } else {
            return Err(binary_utils::error::BinaryError::RecoverableKnown(format!(
                "Packet {} is not a gamepacket",
                id
            )));
        }
    }
}

pub fn construct_packet(id: u32, buffer: &[u8]) -> Result<PacketKind, error::BinaryError> {
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
