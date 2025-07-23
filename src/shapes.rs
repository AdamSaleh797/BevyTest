use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(SystemParam)]
pub struct RenderingParams<'w> {
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
}
