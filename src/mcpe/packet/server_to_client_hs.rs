use binary_utils::*;
use super::packet::*;
use crate::{packet_id, interfaces::Slice};

#[derive(Debug, Clone, BinaryStream)]
pub struct ServerToClientHandshake {
    /// JSON Webtoken handshake, handled by the server
    pub jwt: Slice,
}
packet_id!(ServerToClientHandshake, 0x03);