pub mod packet;
use binary_utils::*;
use crate::interfaces::{VarString, Slice};
// This file contains all packet encoding for Netrex
// Please keep in mind not all of this implmentation is
// Final, a lot of it is just Wrapper typing.

/// Login Packet
#[derive(Debug, BinaryStream)]
pub struct Login {
     pub protocol: u8,
     pub request_data: Slice
}

#[derive(Debug, BinaryStream)]
pub struct ServerToClientHandshake {
     pub jwt: Slice
}

#[derive(Debug, BinaryStream)]
pub struct ClientToServerHandshake {}

#[derive(Debug, BinaryStream)]
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
     ServerFull = 7
}

#[derive(BinaryStream)]
pub struct Disconnect {
     pub hide_screen: bool,
     pub message: VarString
}

// Resource packs {{
#[derive(BinaryStream)]
pub struct BehaviorPackInfo {
     pub uuid: VarString,
     pub version: VarString,
     pub size: u64,
     pub key: VarString,
     pub packname: VarString,
     pub content_id: VarString,
     pub has_scripts: bool
}

#[derive(BinaryStream)]
pub struct TexturePackInfo {
     pub uuid: VarString,
     pub version: VarString,
     pub size: u64,
     pub key: VarString,
     pub packname: VarString,
     pub content_id: VarString,
     pub has_scripts: bool,
     pub rtx_enabled: bool
}

#[derive(BinaryStream)]
pub struct ResourcePackInfo {
     pub pack_required: bool,
     pub has_scripts: bool,
     pub behavior_packs: Slice,
     pub texture_packs: Slice,
     pub force_packs: bool
}
// }}