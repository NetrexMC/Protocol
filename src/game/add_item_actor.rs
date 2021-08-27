use crate::interfaces::Vector3;
use crate::util::{ProtocolEncoder, ProtocolWriter};
use binary_utils::{BinaryStream, IBufferWrite};

pub struct AddItemActor {
     pub entity_id: i64,
     pub entity_runtime_id: u64,
}
