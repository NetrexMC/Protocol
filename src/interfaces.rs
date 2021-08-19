// use crate::util::ProtocolEncoder;
// use binary_utils::{BinaryStream, IBinaryStream, IBufferWrite};

/// A 12 byte value prefixed with 3 floats, x y and z.
pub struct Vector3 {
     pub x: f32,
     pub y: f32,
     pub z: f32
}

/// A 8 byte value prefixed with 2 floats, x, z.
pub struct Vector2 {
     pub x: f32,
     pub z: f32
}

/// A 3 - 15 byte struct
pub struct Coordinates {
     pub x: isize,
     pub y: usize,
     pub z: isize
}

/// 15 byte structure for location
pub struct Location {
     pub x: f32,
     pub y: f32,
     pub z: f32,
     pub yaw: f32,
     pub pitch: f32
}