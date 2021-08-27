pub mod game;
/// Common interfaces, such as blocks, and related.
pub mod interfaces;
pub mod util;

#[cfg(test)]
mod tests {
     use crate::util::ProtocolDecoder;
     use crate::*;
     use binary_utils::*;

     #[test]
     fn test_thing() {
          let mut stream = BinaryStream::new();
          let login = game::Login::read(&mut stream);
     }
}
