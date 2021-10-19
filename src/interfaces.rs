// use crate::util::ProtocolEncoder;
// use binary_utils::{BinaryStream, IBinaryStream, IBufferWrite};
use binary_utils::*;
use std::io::{Write, Read};
use byteorder::{ReadBytesExt, WriteBytesExt};
// /// A 12 byte value prefixed with 3 floats, x y and z.
// #[derive(Copy, Clone)]
// pub struct Vector3 {
//      pub x: f32,
//      pub y: f32,
//      pub z: f32
// }

// /// A 8 byte value prefixed with 2 floats, x, z.
// #[derive(Copy, Clone)]
// pub struct Vector2 {
//      pub x: f32,
//      pub z: f32
// }

/// A 3 - 15 byte struct
#[derive(Copy, Clone, BinaryStream)]
pub struct Coordinates {
     pub x: i64,
     pub y: i64,
     pub z: i64,
}

/// 15 byte structure for location
#[derive(Copy, Clone, BinaryStream)]
pub struct Location {
     pub x: f32,
     pub y: f32,
     pub z: f32,
     pub yaw: f32,
     pub pitch: f32,
}


/// A helper struct that allows easily reading of
/// bytes from a buffer, while keeping it's size
pub struct Slice(pub Vec<u8>);

impl Streamable for Slice {
     fn parse(&self) -> Vec<u8> {
         self.0
     }

     fn compose(source: &[u8], position: &mut usize) -> Self {
          // reads until EOF
          let pos = position.clone();
          *position = source.len();
          Self(source[pos..].to_vec())
     }
}

pub struct VarString(pub String);

impl VarString {
     pub fn to_string(self) -> String {
          self.0
     }
}

impl Streamable for VarString {
     fn compose(source: &[u8], position: &mut usize) -> Self {
          // read the length in var ints
          let length = VarInt::<u32>::from_be_bytes(source);

          // actual string is stored here.
          let contents = source[*position..length.get_byte_length() + *position];
          *position += length.get_byte_length();

          Self(String::from_utf8(contents.to_vec()).unwrap())
     }

     fn parse(&self) -> Vec<u8> {
         let mut stream = Vec::new();
         let bytes = VarInt::<u32>(self.0.len()).to_be_bytes();
         stream.write_all(bytes).unwrap();
         stream
     }
}