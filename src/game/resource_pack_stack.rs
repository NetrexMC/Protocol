use super::resource_pack::StackInfo;

pub struct ResourcePackStack {
     pub texture_pack_required: bool,
     pub behavior_packs: Vec<StackInfo>,
     pub texture_packs: Vec<StackInfo>,
     pub game_version: String,
     pub experiments: Vec<u8>, // incomplete
     pub experiments_toggled: bool
}