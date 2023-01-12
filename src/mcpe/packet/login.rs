use binary_utils::*;
use super::packet::*;
use crate::{packet_id, interfaces::Slice};

/// Login Packet
#[derive(Debug, Clone, BinaryStream)]
pub struct Login {
    pub protocol: u32,
    pub request_data: Slice,
}
packet_id!(Login, 0x01);