pub mod game;
/// Common interfaces, such as blocks, and related.
pub mod interfaces;
pub mod util;

pub use protocol_derive;

#[cfg(test)]
mod tests {
     use crate::util::ProtocolDecoder;
     use crate::*;
     use binary_utils::*;

     #[test]
     fn test_thing() {
          let thing = game::TestPacket { field_b: 2 };
          thing.into_stream();
     }
}
