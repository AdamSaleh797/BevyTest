use bevy::ecs::component::Component;
use bevy_world_space::{position::Position, rect::WorldRect, world_unit::WorldVec2};

#[derive(Component, Clone)]
pub struct BoundingBox {
    bounding_box: WorldRect,
}
impl BoundingBox {
    pub fn new(bounding_box: WorldRect) -> Self {
        Self { bounding_box }
    }
    pub fn collides(&self, position: &Position, collision_point: WorldVec2) -> bool {
        self.bounding_box.contains(collision_point - position.pos)
    }
}
