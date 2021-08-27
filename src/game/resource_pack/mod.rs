use crate::util::{ProtocolEncoder, ProtocolWriter};
use binary_utils::{BinaryStream, IBufferWrite};
pub struct TexturePack {
     pub uuid: String,
     pub version: String,
     pub size: u64,
     pub content_key: String,
     pub sub_pack_name: String,
     pub content_id: String,
     pub scripts: bool,
     pub rtx: bool,
}

impl ProtocolEncoder for TexturePack {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_lstring(&self.uuid);
          stream.write_lstring(&self.version);
          stream.write_ulong(self.size);
          stream.write_lstring(&self.content_key);
          stream.write_lstring(&self.sub_pack_name);
          stream.write_lstring(&self.content_key);
          stream.write_bool(self.scripts);
          stream.write_bool(self.rtx);
          stream
     }
}

pub struct BehaviorPack {
     pub uuid: String,
     pub version: String,
     pub size: u64,
     pub content_key: String,
     pub sub_pack_name: String,
     pub content_id: String,
     pub scripts: bool,
}

impl ProtocolEncoder for BehaviorPack {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_lstring(&self.uuid);
          stream.write_lstring(&self.version);
          stream.write_ulong(self.size);
          stream.write_lstring(&self.content_key);
          stream.write_lstring(&self.sub_pack_name);
          stream.write_lstring(&self.content_id);
          stream.write_bool(self.scripts);
          stream
     }
}

pub struct StackInfo {
     pub uuid: String,
     pub version: String,
     pub sub_pack_name: String,
}

impl ProtocolEncoder for StackInfo {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_lstring(&self.uuid);
          stream.write_lstring(&self.version);
          stream.write_lstring(&self.sub_pack_name);
          stream
     }
}
