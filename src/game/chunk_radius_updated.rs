use crate::util::ProtocolEncoder;
use binary_utils::{BinaryStream, IBufferWrite};

pub struct ChunkRadiusUpdate {
     pub radius: u8,
}

impl ProtocolEncoder for ChunkRadiusUpdate {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_uvar_int(self.radius);
     }
}
