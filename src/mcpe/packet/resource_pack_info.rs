use binary_utils::*;
use super::packet::*;
use crate::{packet_id, interfaces::{Slice, Coordinates}};

#[derive(Debug, Clone, BinaryStream)]
pub struct ResourcePackInfo {
    pub pack_required: bool,
    pub has_scripts: bool,
    pub behavior_packs: Slice,
    pub texture_packs: Slice,
    pub force_packs: bool,
}
packet_id!(ResourcePackInfo, 0x06);
    
    // We're not actually going to be using this packet
    // it's here simply so we can decode batch packets.
    #[derive(Debug, Clone, BinaryStream)]
    pub struct LabTable {
        pub action: u64,
        pub block_pos: Coordinates,
        pub reaction: u64,
    }
    packet_id!(LabTable, 0x6D);
    // }}
    