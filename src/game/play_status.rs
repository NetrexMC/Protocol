use binary_utils::{BinaryStream, IBufferWrite};
use crate::util::ProtocolEncoder;

#[derive(Clone, Copy)]
pub enum PlayStatus {
     Success,
     OutdatedClient,
     OutdatedServer,
     Spawn,
     InvalidTenant,
     EditionIsNotEducation,
     EditionIsNotMatching,
     ServerFull
}

impl ProtocolEncoder for PlayStatus {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_uint(*self as u32);
          stream
     }
}