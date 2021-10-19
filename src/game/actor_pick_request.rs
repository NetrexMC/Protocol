use binary_utils::*;

#[derive(Debug, BinaryStream)]
pub struct ActorPickRequest {
     pub entity_id: u64,
     pub slot: u8,
}