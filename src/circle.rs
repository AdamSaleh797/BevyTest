use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        system::{Commands, ResMut},
    },
    math::primitives::Circle,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    rect::WorldRect,
    world_unit::{WorldUnit, WorldVec2},
};

use crate::{bounding_box::BoundingBox, mouse_drag::Draggable};

#[derive(Component)]
struct CircleId;

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    const PIXEL_RADIUS: f32 = 50.;
    const RADIUS: WorldUnit = WorldUnit::new(3.);
    let circle = Circle::new(PIXEL_RADIUS);
    let mesh = meshes.add(circle);
    let color = Color::srgb(1., 0., 0.);
    let material = materials.add(color);
    let bounding_box = BoundingBox::new(WorldRect::from_center_half_size(
        WorldVec2::ZERO,
        WorldVec2 {
            x: RADIUS,
            y: RADIUS,
        },
    ));

    let position = Position::new(WorldVec2::ZERO, RADIUS, (2. * PIXEL_RADIUS) as u32, 0.);
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
