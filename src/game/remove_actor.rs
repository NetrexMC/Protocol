use crate::util::ProtocolEncoder;
use binary_utils::{BinaryStream, IBufferWrite};

pub struct RemoveActor {
    pub network_id: i64,
}

impl ProtocolEncoder for RemoveActor {
    fn write(&self) -> BinaryStream {
        let mut stream = BinaryStream::new();
        stream.write_long(self.network_id);
        stream
    }
}
