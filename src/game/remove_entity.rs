use crate::util::ProtocolEncoder;
use binary_utils::{BinaryStream, IBufferWrite};

pub struct RemoveEntity {
    pub entity_id: i64,
}

impl ProtocolEncoder for RemoveEntity {
    fn write(&self) -> BinaryStream {
        let mut stream = BinaryStream::new();
        stream.write_long(self.entity_id);
        stream
    }
}
