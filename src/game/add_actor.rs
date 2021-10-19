use binary_utils::*;
use glm::Vector3;

#[derive(Debug, BinaryStream)]
pub struct AddActor {
     pub entity_id: i64,
     pub entity_runtime_id: i64,
     pub entity_type: LE::<String>,
     pub position: Vector3::<f32>,
     pub velocity: Vector3::<f32>,
     pub pitch: f32,
     pub yaw: f32,
     pub head_yaw: f32,
     pub l0: i32,
     pub l1: i32,
     pub l2: i32,
     // attributes: Vec<ActorAttribute>,
     // entity_meta: Vec<u32>,
     // entity_links: Vec<EntityLink>
}