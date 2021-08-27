use crate::util::{ProtocolEncoder, ProtocolWriter};
use binary_utils::{BinaryStream, IBufferWrite};
use glm::Vector3;

pub struct AddActor {
     pub entity_id: i64,
     pub entity_runtime_id: i64,
     pub entity_type: String,
     pub position: Vector3<f32>,
     pub velocity: Vector3<f32>,
     pub pitch: f32,
     pub yaw: f32,
     pub head_yaw: f32,
     // attributes: Vec<ActorAttribute>,
     // entity_meta: Vec<u32>,
     // entity_links: Vec<EntityLink>
}

impl ProtocolEncoder for AddActor {
     fn write(&self) -> BinaryStream {
          let mut stream = BinaryStream::new();
          stream.write_long(self.entity_id);
          stream.write_long(self.entity_runtime_id);
          stream.write_lstring(&self.entity_type);
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
