pub mod batch;
pub mod packet;
pub use packet::*;

use crate::interfaces::{Slice, VarString};

use binary_utils::*;

// This file contains all packet encoding for Netrex
// Please keep in mind not all of this implmentation is
// Final, a lot of it is just Wrapper typing.
macro_rules! packet_id {
    ($name: ident, $id: literal) => {
        impl PacketId for $name {
            fn id() -> u32 {
                $id
            }
        }
    };
}

/// Login Packet
#[derive(Debug, Clone, BinaryStream)]
pub struct Login {
    pub protocol: u32,
    pub request_data: Slice,
}
packet_id!(Login, 0x01);

#[derive(Debug, Clone, BinaryStream, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum PlayStatus {
    /// The client can successfully join the game
    Success = 0,
    /// The client has an outdated protocol version and may not join the server.
    FailedClient = 1,
    /// The client version is not supported and the server version is outdated when compared to the client.
    FailedServer = 2,
    /// The client can join and is being spawned into the world.
    PlayerSpawn = 3,
    /// Caused when the client does not have permission to join the server.
    /// Caused on EDU edition servers.
    InvalidTenant = 4,
    /// The client tried connect to the server with Education Edition, but the server is not an EDU edition server.
    NotEdu = 5,
    /// The client tried to connect to the server with a different edition than the server.
    EduVanilla = 6,
    /// The client tried to join the server while it was full.
    ServerFull = 7,
}
packet_id!(PlayStatus, 0x02);

#[derive(Debug, Clone, BinaryStream)]
pub struct ServerToClientHandshake {
    /// JSON Webtoken handshake, handled by the server
    pub jwt: Slice,
}
packet_id!(ServerToClientHandshake, 0x03);

#[derive(Debug, Clone, BinaryStream)]
pub struct ClientToServerHandshake {}
packet_id!(ClientToServerHandshake, 0x04);

#[derive(Debug, Clone, BinaryStream)]
pub struct Disconnect {
    pub hide_screen: bool,
    pub message: VarString,
}
packet_id!(Disconnect, 0x05);

// Resource packs {{
#[derive(Debug, Clone, BinaryStream)]
pub struct BehaviorPackInfo {
    pub uuid: VarString,
    pub version: VarString,
    pub size: u64,
    pub key: VarString,
    pub packname: VarString,
    pub content_id: VarString,
    pub has_scripts: bool,
}

#[derive(Debug, Clone, BinaryStream)]
pub struct TexturePackInfo {
    pub uuid: VarString,
    pub version: VarString,
    pub size: u64,
    pub key: VarString,
    pub packname: VarString,
    pub content_id: VarString,
    pub has_scripts: bool,
    pub rtx_enabled: bool,
}

#[derive(Debug, Clone, BinaryStream)]
pub struct ResourcePackInfo {
    pub pack_required: bool,
    pub has_scripts: bool,
    pub behavior_packs: Slice,
    pub texture_packs: Slice,
    pub force_packs: bool,
}
packet_id!(ResourcePackInfo, 0x06);
// }}
