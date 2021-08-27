use crate::interfaces::{Coordinates, Location};
use binary_utils::{BinaryStream, IBufferRead, IBufferWrite};
use glm::{Vector2, Vector3};

/// A trait to simplify the process
/// of parsing a specific interface
pub trait ProtocolEncoder {
     /// Parses a given interface returning
     /// its byte stream as a BinaryStream.
     fn write(&self) -> BinaryStream;
}

pub trait ProtocolDecoder {
     /// Reads a given stream, and creates an instance from itself.
     fn read(stream: &mut BinaryStream) -> Self;
}

/// A packet utility to simplify
/// writing interfaces specifically related to minecraft.
pub trait ProtocolWriter {
     /// Write a vector3 to the Binary Stream.
     fn write_vector3(&mut self, vector: Vector3<f32>);

     /// Write a vector2 to the Binary Stream.
     fn write_vector2(&mut self, vector: Vector2<f32>);

     /// Writes a coordinate to the Binary Stream.
     fn write_coordinates(&mut self, coordinates: Coordinates);

     /// Writes a coordinate to the Binary Stream.
     fn write_location(&mut self, location: Location);

     fn write_lstring(&mut self, string: &String);
}

impl ProtocolWriter for BinaryStream {
     fn write_vector2(&mut self, vector: Vector2<f32>) {
          self.write_float(vector.x);
          self.write_float(vector.y);
     }

     fn write_vector3(&mut self, vector: Vector3<f32>) {
          self.write_float(vector.x);
          self.write_float(vector.y);
          self.write_float(vector.z);
     }

     fn write_coordinates(&mut self, coordinates: Coordinates) {
          self.write_var_long(coordinates.x);
          self.write_var_long(coordinates.y);
          self.write_var_long(coordinates.z);
     }

     fn write_location(&mut self, location: Location) {
          self.write_float(location.x);
          self.write_float(location.y);
          self.write_float(location.z);
          self.write_float(location.yaw);
          self.write_float(location.pitch);
     }

     fn write_lstring(&mut self, string: &String) {
          let bytes = string.as_bytes();
          self.write_uvar_int(bytes.len() as u32);
          self.write_slice(&bytes);
     }
}

pub trait ProtocolReader {
     /// Read a vector3 from a Binary Stream.
     fn read_vector3(&mut self) -> Vector3<f32>;

     /// Read a vector2 from a Binary Stream.
     fn read_vector2(&mut self) -> Vector2<f32>;

     /// Reads a coordinate from a Binary Stream.
     fn read_coordinates(&mut self) -> Coordinates;

     /// Reads a coordinate from a Binary Stream.
     fn read_location(&mut self) -> Location;

     /// Reads a string from a Binary Stream.
     fn read_lstring(&mut self) -> String;
}

impl ProtocolReader for BinaryStream {
     /// Read a vector3 from a Binary Stream.
     fn read_vector3(&mut self) -> Vector3<f32> {
          Vector3 {
               x: self.read_float(),
               y: self.read_float(),
               z: self.read_float(),
          }
     }

     /// Read a vector2 from a Binary Stream.
     fn read_vector2(&mut self) -> Vector2<f32> {
          Vector2 {
               x: self.read_float(),
               y: self.read_float(),
          }
     }

     /// Reads a coordinate from a Binary Stream.
     fn read_coordinates(&mut self) -> Coordinates {
          Coordinates {
               x: self.read_var_long(),
               y: self.read_var_long(),
               z: self.read_var_long(),
          }
     }

     /// Reads a coordinate from a Binary Stream.
     fn read_location(&mut self) -> Location {
          Location {
               x: self.read_float(),
               y: self.read_float(),
               z: self.read_float(),
               yaw: self.read_float(),
               pitch: self.read_float(),
          }
     }

     /// Reads a string from a Binary Stream.
     fn read_lstring(&mut self) -> String {
          let len = self.read_var_int();
          let string = String::from_utf8(self.read_slice_exact(Some(len as usize)).to_vec());
          self.increase_offset(Some(len as usize));
          match string {
               Ok(v) => v,
               Err(v) => panic!(v),
          }
     }
}
