// use crate::util::ProtocolEncoder;
// use binary_utils::{BinaryStream, IBinaryStream, IBufferWrite};

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
#[derive(Copy, Clone)]
pub struct Coordinates {
     pub x: i64,
     pub y: i64,
     pub z: i64,
}

/// 15 byte structure for location
#[derive(Copy, Clone)]
pub struct Location {
     pub x: f32,
     pub y: f32,
     pub z: f32,
     pub yaw: f32,
     pub pitch: f32,
}
