use crate::util::ProtocolEncoder;
use binary_utils::*;

pub struct ServerClientHandshake {
     pub jwt_data: Vec<u8>,
}

impl ProtocolEncoder for ServerClientHandshake {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_slice(&self.jwt_data[..]);
          stream
     }
}
