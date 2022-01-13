use std::io::Cursor;

use binary_utils::{Streamable, VarInt};

use super::Packet;

/// A packet batch, (batch packet)
/// This is a wrapper for game packets, and is used to send multiple packets
/// in one packet (which is weird but ok)
///
/// By default the packet limit is set to 255, however you may change this.
/// The most optimal limit is 255, however if you want to eat more ram,
/// bite the dust.
pub struct Batch {
    /// The packets in this batch that are decoded & ready to be sent
    /// This will cap at `limit` packets.
    pub packets: Vec<Packet>,
    /// The maximum amount of packets that can be sent in this batch
    /// This is set to 255 by default.
    limit: usize,
}

impl Batch {
    pub fn new(limit: usize) -> Self {
        Self {
            packets: Vec::new(),
            limit,
        }
    }

    pub fn add(&mut self, packet: Packet) {
        self.packets.push(packet);
    }
}

impl From<Vec<u8>> for Batch {
    fn from(buffer: Vec<u8>) -> Self {
        let mut packets: Vec<Packet> = Vec::new();
        let mut position: usize = 0;
        loop {
            // let's read a unsigned var int
            if position >= buffer.len() {
                break;
            }
            if let Ok(packet) = Packet::compose(&buffer, &mut position) {
                packets.push(packet);
            } else {
                break;
            }
        }

        let length = packets.len();
        Self {
            packets,
            limit: length,
        }
    }
}
