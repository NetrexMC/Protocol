use binary_utils::*;
use super::packet::*;
use crate::{packet_id, interfaces::VarString};

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
packet_id!(TexturePackInfo, 0x06);