use binary_utils::{BinaryStream, IBufferWrite};
use crate::interfaces::{Vector3, Vector2, Coordinates, Location};

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
     fn write_vector3(&mut self, vector: Vector3);

     /// Write a vector2 to the Binary Stream.
     fn write_vector2(&mut self, vector: Vector2);

     /// Writes a coordinate to the Binary Stream.
     fn write_coordinates(&mut self, coordinates: Coordinates);

     /// Writes a coordinate to the Binary Stream.
     fn write_location(&mut self, location: Location);
}

impl ProtocolWriter for BinaryStream {
     fn write_vector2(&mut self, vector: Vector2) {
          self.write_float(vector.x);
          self.write_float(vector.z);
     }

     fn write_vector3(&mut self, vector: Vector3) {
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
}