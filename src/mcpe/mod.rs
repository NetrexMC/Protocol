pub mod packet_util;
pub use packet_util::*;

use crate::interfaces::{Slice, VarString};
use binary_utils::*;

// This file contains all packet encoding for Netrex
// Please keep in mind not all of this implmentation is
// Final, a lot of it is just Wrapper typing.
macro_rules! packet_id {
    ($name: ident, $id: literal) => {
        impl PacketId for $name {
            fn id() -> u8 {
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
    // Failed login
    Success = 0,
    FailedClient = 1,
    FailedServer = 2,
    PlayerSpawn = 3,
    InvalidTenant = 4,
    NotEdu = 5,
    EduVanilla = 6,
    ServerFull = 7,
}
packet_id!(PlayStatus, 0x02);

#[derive(Debug, Clone, BinaryStream)]
pub struct ServerToClientHandshake {
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
