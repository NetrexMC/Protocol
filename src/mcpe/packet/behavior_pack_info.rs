use binary_utils::*;
use super::packet::*;
use crate::{interfaces::VarString, packet_id};

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
packet_id!(BehaviorPackInfo, 0x07);