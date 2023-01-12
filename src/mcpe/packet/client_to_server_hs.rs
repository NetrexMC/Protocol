use binary_utils::*;
use super::packet::*;

use crate::packet_id;

#[derive(Debug, Clone, BinaryStream)]
pub struct ClientToServerHandshake {}
packet_id!(ClientToServerHandshake, 0x04);