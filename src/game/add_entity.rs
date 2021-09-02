use crate::util::ProtocolEncoder;
use binary_utils::{BinaryStream, IBufferWrite};

pub struct AddEntity {
     pub network_id: u64,
}