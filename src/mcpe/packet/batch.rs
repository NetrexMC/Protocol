use std::io::Write;

use binary_utils::Streamable;
// use byteorder::WriteBytesExt;

use super::Packet;

/// A packet batch, (batch packet)
/// This is a wrapper for game packets, and is used to send multiple packets
/// in one packet (which is weird but ok)
///
/// By default the packet limit is set to 255, however you may change this.
/// The most optimal limit is 255, however if you want to eat more ram,
/// bite the dust.
#[derive(Debug, Clone)]
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

    /// Gets the amount of packets that can be sent in this batch
    pub fn get_limit(&self) -> usize {
        return self.limit.clone();
    }

    /// Gets the amount of packets that are currently in this batch
    pub fn get_size(&self) -> usize {
        return self.packets.len();
    }

    /// Gets the amount of packets that can be added to this batch
    pub fn get_remaining(&self) -> usize {
        return self.limit - self.packets.len();
    }

    pub fn add(&mut self, packet: Packet) -> bool {
        if self.packets.len() >= self.limit {
            return false;
        }
        self.packets.push(packet);
        return true;
    }

    pub fn get_packets(&self) -> Vec<Packet> {
        self.packets.clone()
    }
}

impl Streamable for Batch {
    fn compose(
        source: &[u8],
        position: &mut usize,
    ) -> Result<Self, binary_utils::error::BinaryError> {
        let mut packets: Vec<Packet> = Vec::new();
        loop {
            // let's read a unsigned var int
            if *position >= source.len() {
                break;
            }

            packets.push(Packet::compose(&source, position)?);
        }
        let length = packets.len();
        Ok(Self {
            packets,
            limit: length,
        })
    }

    fn parse(&self) -> Result<Vec<u8>, binary_utils::error::BinaryError> {
        let mut buf: Vec<u8> = Vec::new();
        for packet in &self.packets {
            buf.write_all(&packet.parse()?)?;
        }

        Ok(buf)
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
