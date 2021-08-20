use binary_utils::{BinaryStream, IBinaryStream, IBufferRead};
use crate::util::{ProtocolDecoder};

pub struct Login {
     pub protocol: u8,
     pub connection_request: BinaryStream
}

impl ProtocolDecoder for Login {
     fn read(stream: &mut BinaryStream) -> Self {
          Self {
               protocol: stream.read_byte(),
               connection_request: BinaryStream::init(&stream.read_slice_exact(None))
          }
     }
}