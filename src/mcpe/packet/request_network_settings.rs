use binary_utils::*;
use super::packet::*;
use crate::packet_id;

#[derive(Debug, Clone, BinaryStream)]
pub struct RequestNetworkSettings {
    pub protocol: u32,
}
packet_id!(RequestNetworkSettings, 0xc1);