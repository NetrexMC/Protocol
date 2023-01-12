use binary_utils::*;
use super::packet::*;
use crate::{packet_id, interfaces::VarString};

#[derive(Debug, Clone, BinaryStream)]
pub struct Disconnect {
    pub hide_screen: bool,
    pub message: VarString,
}
packet_id!(Disconnect, 0x05);