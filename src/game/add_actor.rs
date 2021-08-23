use crate::util::{ProtocolEncoder, ProtocolWriter};
use crate::interfaces::Vector3;
use binary_utils::{BinaryStream, IBufferWrite};

pub struct AddActor {
     entity_id: i64,
     entity_runtime_id: i64,
     entity_type: String,
     position: Vector3,
     velocity: Vector3,
     pitch: f32,
     yaw: f32,
     head_yaw: f32,
     // attributes: Vec<ActorAttribute>,
     // entity_meta: Vec<u32>,
     // entity_links: Vec<EntityLink>
}

impl ProtocolEncoder for AddActor {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_long(self.entity_id);
          stream.write_long(self.entity_runtime_id);
          stream.write_string(self.entity_type.clone());
          stream.write_vector3(self.position);
          stream.write_vector3(self.velocity);
          stream.write_float(self.pitch);
          stream.write_float(self.yaw);
          stream.write_float(self.head_yaw);
          stream.write_var_int(0);
          stream.write_var_int(0);
          stream.write_var_int(0);
          stream
     }
}