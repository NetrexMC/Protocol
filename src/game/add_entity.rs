use crate::util::ProtocolEncoder;
use binary_utils::{BinaryStream, IBufferWrite};

pub struct AddEntity {
     pub network_id: u64,
}

impl ProtocolEncoder for AddEntity {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_uvar_int(self.network_id);
     }
}
