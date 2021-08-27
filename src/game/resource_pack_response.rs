use crate::util::{ProtocolDecoder, ProtocolEncoder, ProtocolReader, ProtocolWriter};
use binary_utils::{BinaryStream, IBufferRead, IBufferWrite};
#[derive(Clone, Copy, PartialEq)]
pub enum ResourcePackResponseType {
     None,
     /// Sent if the player denies the downloading of the resource packs
     /// in order to join the server.
     Refused,
     /// Sent when the client is missing some texture packs and needs them to be sent.
     /// If this is present, an array of packets will be present in the `ResourcePackResponse` packet.
     Send,
     /// Sent when the client matches one of the following:
     /// - All of the packs from the server have been downloaded.
     /// - All of the packs are already present on the clients computer.
     PacksPresent,
     /// Sent when the client is ready to proceed with the login process.
     Completed,
}

pub struct ResourcePackResponse {
     /// The response from the client, in response to: `ResourcePacksInfo`
     pub status: ResourcePackResponseType,
     /// A vector of UUID's containing the packs that need downloading.
     /// This field is only present if `ResourcePackResponseType::Send` is the status.
     pub packs: Vec<String>,
}

impl ProtocolDecoder for ResourcePackResponse {
     fn read(stream: &mut BinaryStream) -> Self {
          let status = match stream.read_byte() {
               0 => ResourcePackResponseType::None,
               1 => ResourcePackResponseType::Refused,
               2 => ResourcePackResponseType::Send,
               3 => ResourcePackResponseType::PacksPresent,
               4 => ResourcePackResponseType::Completed,
               _ => ResourcePackResponseType::None,
          };
          let mut packs = Vec::<String>::new();

          for _ in 0..stream.read_ushort() {
               packs.push(stream.read_lstring());
          }

          Self { status, packs }
     }
}

impl ProtocolEncoder for ResourcePackResponse {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          match self.status {
               ResourcePackResponseType::None => stream.write_byte(0),
               ResourcePackResponseType::Refused => stream.write_byte(1),
               ResourcePackResponseType::Send => stream.write_byte(2),
               ResourcePackResponseType::PacksPresent => stream.write_byte(3),
               ResourcePackResponseType::Completed => stream.write_byte(4),
          };

          stream.write_ushort(self.packs.len() as u16);

          for pack in self.packs.iter() {
               stream.write_lstring(pack);
          }
          stream
     }
}
