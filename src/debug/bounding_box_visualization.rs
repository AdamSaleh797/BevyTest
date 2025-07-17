use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    input::{ButtonInput, keyboard::KeyCode},
    log::info,
    math::{Rect, primitives::Rectangle},
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    rect::WorldRect,
    win_info::WinInfo,
    world_unit::{AspectRatio, WorldUnit, WorldVec2},
};

use crate::bounding_box::BoundingBox;

#[derive(Component)]
struct BoxSkeleton;

fn toggle(
    keys: Res<ButtonInput<KeyCode>>,
    bounding_box_query: Query<(&BoundingBox, Entity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    win_info: Res<WinInfo>,
    aspect_ratio: Res<AspectRatio>,
) {
    if !keys.just_pressed(KeyCode::KeyB) {
        return;
    }
    info!("pressed b");
    for (bounding_box, id) in &bounding_box_query {
        let rectangle = bounding_box
            .bounding_box()
            .to_rect(&win_info, &aspect_ratio);
        let mesh = meshes.add(Rectangle::new(rectangle.width(), rectangle.height()));
        let color = Color::srgb(0., 1., 0.);
        let material = materials.add(color);

        let position = Position::new(
            bounding_box.bounding_box().center(),
            bounding_box.bounding_box().width(),
            rectangle.width() as u32,
            -1.,
        );
        let mesh_component = Mesh2d(mesh);
        let color_component = MeshMaterial2d(material);

        let bb_id = commands
            .spawn((BoxSkeleton, position, mesh_component, color_component))
            .id();

        commands.entity(id).add_child(bb_id);
    }
}
#[derive(Default)]
pub struct BoundingBoxVisualizationPlugin;
impl Plugin for BoundingBoxVisualizationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, toggle);
    }
}
