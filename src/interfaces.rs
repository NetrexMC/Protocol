// use crate::util::ProtocolEncoder;
// use binary_utils::{BinaryStream, IBinaryStream, IBufferWrite};
use binary_utils::*;
use std::{convert::TryInto, io::Write};

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
#[derive(Debug)]
pub struct Slice(pub Vec<u8>);

impl Streamable for Slice {
     fn parse(&self) -> Vec<u8> {
         self.0.clone()
     }

     fn compose(source: &[u8], position: &mut usize) -> Self {
          // reads until EOF
          let pos = position.clone();
          *position = source.len();
          Self(source[pos..].to_vec())
     }
}

/// A helper struct to read/write Strings with
/// varint encoded lengths.
///
/// **Example:**
/// ```rust notest
/// let my_string = "Hello World!".to_string();
/// let encoded = VarString(my_string).parse();
/// [12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]
/// ```
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
          let contents = &source[*position..(length.get_byte_length() as usize) + *position];
          *position += length.get_byte_length() as usize;

          Self(String::from_utf8(contents.to_vec()).unwrap())
     }

     fn parse(&self) -> Vec<u8> {
         let mut stream = Vec::new();
         let bytes = VarInt::<u32>(self.0.len().try_into().unwrap()).to_be_bytes();
         stream.write_all(&bytes[..]).unwrap();
         stream.write_all(self.0.as_bytes()).unwrap();
         stream
     }
}

/// A helper struct to encode u32/i32 Length strings
/// It is advised to use String implementation when possible.
pub struct String32(pub String);

/// A helper struct to encode u32/i32 LE Length strings
/// It is advised to use String implementation when possible.
pub struct LString32(pub String);

impl Streamable for LString32 {
     fn parse(&self) -> Vec<u8> {
         // get the length
         let mut buffer: Vec<u8> = Vec::new();
         buffer.write_all(&LE::<u32>(self.0.len() as u32).parse()[..]).unwrap();
         // now we write string buffer.
         buffer.write_all(&self.0.clone().into_bytes()[..]).unwrap();
         buffer
     }

     fn compose(source: &[u8], position: &mut usize) -> Self {
         // get the length.
         let length = LE::<u32>::compose(&source, position);
         let bytes = &source[*position..(length.0 as usize)];
         Self(String::from_utf8(bytes.to_vec()).unwrap())
     }
}

impl Streamable for String32 {
     fn parse(&self) -> Vec<u8> {
         // get the length
         let mut buffer: Vec<u8> = Vec::new();
         buffer.write_all(&(self.0.len() as u32).parse()[..]).unwrap();
         // now we write string buffer.
         buffer.write_all(&self.0.clone().into_bytes()[..]).unwrap();
         buffer
     }

     fn compose(source: &[u8], position: &mut usize) -> Self {
         // get the length.
         let length = u32::compose(&source, position);
         let bytes = &source[*position..(length as usize)];
         Self(String::from_utf8(bytes.to_vec()).unwrap())
     }
}

pub struct VarSlice(pub Vec<u8>);

impl VarSlice {
     pub fn to_vec(self) -> Vec<u8> {
          self.0
     }
}

impl Streamable for VarSlice {
     fn compose(source: &[u8], position: &mut usize) -> Self {
          // read the length in var ints
          let length = VarInt::<u32>::from_be_bytes(source);
          *position += length.get_byte_length() as usize;
          let from = *position;
          let to = length.0 as usize + *position;
          *position += length.0 as usize;
          // actual string is stored here.
          let contents = &source[from..to];

          Self(contents.to_vec())
     }

     fn parse(&self) -> Vec<u8> {
         let mut stream = Vec::new();
         let bytes = VarInt::<u32>(self.0.len().try_into().unwrap()).to_be_bytes();
         stream.write_all(&bytes[..]).unwrap();
         stream.write_all(&self.0[..]).unwrap();
         stream
     }
}

/// Byte arrays are read with varints.
/// The length of the bytearray sized by a varint.
pub struct ByteArray<T>(pub Vec<T>);

impl<T> ByteArray<T>
where
     T: Streamable {
     pub fn to_vec(self) -> Vec<T> {
          self.0
     }
}

impl<T> Streamable for ByteArray<T>
where
     T: Streamable {
     fn compose(source: &[u8], position: &mut usize) -> Self {
          // read the length in var ints
          let length = VarInt::<u32>::from_be_bytes(source);
          // Update the position to consume the length of the varint.
          *position += length.get_byte_length() as usize;
          *position += length.0 as usize;

          let mut ret = Vec::<T>::new();

          // we have the length now let's iterate until we dont.
          for _ in 0..length.0 {
               let data = T::compose(source, position);
               ret.push(data);
          }

          Self(ret)
     }

     fn parse(&self) -> Vec<u8> {
         let mut stream = Vec::new();
         let length = VarInt::<u32>(self.0.len() as u32);
         // write the length.
         stream.write_all(&length.to_be_bytes()[..]).unwrap();
         for kind in &self.0 {
              // write each index now.
              stream.write_all(&kind.parse()[..]).expect("Writing a ByteArray failed.");
         }
         stream
     }
}
