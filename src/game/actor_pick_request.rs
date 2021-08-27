use crate::util::{ProtocolDecoder, ProtocolEncoder};
use binary_utils::{BinaryStream, IBufferRead, IBufferWrite};

pub struct ActorPickRequest {
     pub entity_id: u64,
     pub slot: u8,
}

impl ProtocolEncoder for ActorPickRequest {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_ulong(self.entity_id);
          stream.write_byte(self.slot);
          stream
     }
}

impl ProtocolDecoder for ActorPickRequest {
     fn read(stream: &mut BinaryStream) -> Self {
          Self {
               entity_id: stream.read_ulong(),
               slot: stream.read_byte(),
          }
     }
}
