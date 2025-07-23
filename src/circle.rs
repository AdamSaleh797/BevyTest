use bevy::{
    app::{Plugin, Startup},
    color::Color,
    ecs::{component::Component, system::Commands},
    math::primitives::Circle,
    render::mesh::Mesh2d,
    sprite::MeshMaterial2d,
};
use bevy_world_space::{
    position::Position,
    rect::WorldRect,
    world_unit::{WorldUnit, WorldVec2},
};

use crate::{bounding_box::BoundingBox, mouse_drag::Draggable, shapes::RenderingParams};

#[derive(Component)]
struct CircleId;

fn setup(mut rendering_params: RenderingParams, mut commands: Commands) {
    const PIXEL_RADIUS: f32 = 50.;
    const RADIUS: WorldUnit = WorldUnit::new(1.5);
    let circle = Circle::new(PIXEL_RADIUS);
    let mesh = rendering_params.meshes.add(circle);
    let color = Color::srgb(1., 0., 0.);
    let material = rendering_params.materials.add(color);
    let bounding_box = BoundingBox::new(WorldRect::from_center_half_size(
        WorldVec2::ZERO,
        WorldVec2 {
            x: RADIUS,
            y: RADIUS,
        },
    ));

    let position = Position::new(WorldVec2::ZERO, 2. * RADIUS, (2. * PIXEL_RADIUS) as u32, 0.);
    let mesh_component = Mesh2d(mesh);
    let color_component = MeshMaterial2d(material);

    commands.spawn((
        CircleId,
        Draggable,
        bounding_box.clone(),
        position.clone(),
        mesh_component.clone(),
        color_component.clone(),
    ));

    commands.spawn((
        CircleId,
        Draggable,
        bounding_box,
        position,
        mesh_component,
        color_component,
    ));
}

pub struct CirclePlugin;
impl Plugin for CirclePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
    }
}
