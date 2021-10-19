use crate::interfaces::Slice;
use binary_utils::*;

#[derive(Debug, BinaryStream)]
pub struct Login {
     pub protocol: u8,
     pub connection_request: Slice,
}
