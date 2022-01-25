// use crate::util::ProtocolEncoder;
// use binary_utils::{BinaryStream, IBinaryStream, IBufferWrite};
use binary_utils::error::BinaryError;
use binary_utils::*;
use std::{convert::TryInto, io::Write};

/// A 3 - 15 byte struct
#[derive(Copy, Debug, Clone, BinaryStream)]
pub struct Coordinates {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

/// 15 byte structure for location
#[derive(Copy, Debug, Clone, BinaryStream)]
pub struct Location {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
}

/// A helper struct that allows easily reading of
/// bytes from a buffer, while keeping it's size
#[derive(Debug, Clone)]
pub struct Slice(pub Vec<u8>);

impl Streamable for Slice {
    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        Ok(self.0.clone())
    }

    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        // reads until EOF
        let pos = position.clone();
        *position = source.len();
        Ok(Self(source[pos..].to_vec()))
    }
}

/// A helper struct to read/write Strings with
/// varint encoded lengths.
///
/// ```rust
/// use mcpe_protocol::interfaces::VarString;
/// use binary_utils::*;
///
/// let my_string = "Hello World!".to_string();
/// let encoded = VarString(my_string).fparse();
/// assert_eq!(encoded, &[12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33])
/// ```
#[derive(Debug, Clone)]
pub struct VarString(pub String);

impl VarString {
    pub fn to_string(self) -> String {
        self.0
    }
}

impl Streamable for VarString {
    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        // read the length in var ints
        let length = VarInt::<u32>::from_be_bytes(&source[*position..])?;
        // increase the offset byte the length of bytes the varint is.
        *position += length.get_byte_length() as usize;
        let contents = &source[*position..(*position + length.0 as usize)];
        *position += contents.len();

        Ok(Self(String::from_utf8(contents.to_vec()).unwrap()))
    }

    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        let mut stream = Vec::new();
        let bytes = VarInt::<u32>(self.0.len().try_into().unwrap()).to_be_bytes();
        stream.write_all(&bytes[..])?;
        stream.write_all(self.0.as_bytes())?;
        Ok(stream)
    }
}

/// A helper struct to encode u32/i32 Length strings
/// It is advised to use String implementation when possible.
#[derive(Debug, Clone)]
pub struct String32(pub String);

impl Streamable for String32 {
    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        // get the length
        let mut buffer: Vec<u8> = Vec::new();
        buffer.write_all(&(self.0.len() as u32).parse()?[..])?;
        // now we write string buffer.
        buffer.write_all(&self.0.clone().into_bytes()[..])?;
        Ok(buffer)
    }

    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        // get the length.
        let length = u32::compose(&source, position)?;
        let bytes = &source[*position..(*position + length as usize)];
        *position = bytes.len();
        Ok(Self(String::from_utf8(bytes.to_vec()).unwrap()))
    }
}

/// A helper struct to encode u32/i32 LE Length strings
/// It is advised to use String implementation when possible.
#[derive(Debug, Clone)]
pub struct LString32(pub String);

impl Streamable for LString32 {
    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        // get the length
        let mut buffer: Vec<u8> = Vec::new();
        buffer.write_all(&LE::<u32>(self.0.len() as u32).parse()?[..])?;
        // now we write string buffer.
        buffer.write_all(&self.0.clone().into_bytes()[..])?;
        Ok(buffer)
    }

    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        let length = LE::<u32>::compose(source, position)?;
        let bytes = &source[*position..(*position + length.0 as usize)];

        *position += bytes.len();

        Ok(Self(unsafe { String::from_utf8_unchecked(bytes.to_vec()) }))
    }
}

#[derive(Debug, Clone)]
pub struct VarSlice(pub Vec<u8>);

impl VarSlice {
    pub fn to_vec(self) -> Vec<u8> {
        self.0
    }
}

impl Streamable for VarSlice {
    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        // read the length in var ints
        let length = VarInt::<u32>::from_be_bytes(&source[*position..])?;
        *position += length.get_byte_length() as usize;
        let from = *position;
        let to = length.0 as usize + *position;
        *position += length.0 as usize;
        // actual string is stored here.
        let contents = &source[from..to];

        Ok(Self(contents.to_vec()))
    }

    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        let mut stream = Vec::new();
        let bytes = VarInt::<u32>(self.0.len().try_into().unwrap()).to_be_bytes();
        stream.write_all(&bytes[..])?;
        stream.write_all(&self.0[..])?;
        Ok(stream)
    }
}

/// A leading byte array,
/// The length of the byte array is encoded as `L`
// pub struct LByteArray<T, const L: usize>(pub Vec<T>);

// impl<L, T> Streamable for LByteArray<L, T>
// where
//      L: AnyNumber,
//      T: Streamable {
//      fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
//          let length = L::compose(source, position)?;
//      }
//      fn parse(&self) -> Result<Vec<u8>, BinaryError> { todo!() }
// }

/// Byte arrays are read with varints.
/// The length of the bytearray sized by a varint.
#[derive(Debug, Clone)]
pub struct ByteArray<T>(pub Vec<T>);

impl<T> ByteArray<T>
where
    T: Streamable,
{
    pub fn to_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T> Streamable for ByteArray<T>
where
    T: Streamable,
{
    fn compose(source: &[u8], position: &mut usize) -> Result<Self, BinaryError> {
        // read the length in var ints
        let length = VarInt::<u32>::from_be_bytes(&source[*position..])?;
        // Update the position to consume the length of the varint.
        *position += length.get_byte_length() as usize;

        let mut ret = Vec::<T>::new();

        // we have the length now let's iterate until we dont.
        for _ in 0..length.0 {
            let data = T::compose(&source, position)?;
            ret.push(data);
        }

        Ok(Self(ret))
    }

    fn parse(&self) -> Result<Vec<u8>, BinaryError> {
        let mut stream = Vec::new();
        let length = VarInt::<u32>(self.0.len() as u32);
        // write the length.
        stream.write_all(&length.to_be_bytes()[..]).unwrap();
        for kind in &self.0 {
            // write each index now.
            stream
                .write_all(&kind.parse()?[..])
                .expect("Writing a ByteArray failed.");
        }
        Ok(stream)
    }
}
