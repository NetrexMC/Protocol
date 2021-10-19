use binary_utils::*;

#[derive(BinaryStream, Debug)]
pub struct AddItemActor {
     pub entity_id: i64,
     pub entity_runtime_id: u64,
}
