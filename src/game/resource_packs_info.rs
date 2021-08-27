use super::resource_pack::{BehaviorPack, TexturePack};
use crate::util::ProtocolEncoder;
use binary_utils::{BinaryStream, IBufferWrite};

pub struct ResourcePacksInfo {
     pub texture_pack_required: bool,
     pub has_scripts: bool,
     pub behavior_packs: Vec<BehaviorPack>,
     pub texture_packs: Vec<TexturePack>,
     pub force_pack: bool,
}

impl ProtocolEncoder for ResourcePacksInfo {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_bool(self.texture_pack_required);
          stream.write_bool(self.has_scripts);
          // write behavior packs
          stream.write_ushort(self.behavior_packs.len() as u16);
          for pack in self.behavior_packs.iter() {
               stream.write_slice(&pack.write().get_buffer()[..]);
          }
          stream.write_ushort(self.texture_packs.len() as u16);
          for pack in self.texture_packs.iter() {
               stream.write_slice(&pack.write().get_buffer()[..]);
          }
          stream.write_bool(self.force_pack);
          stream
     }
}
