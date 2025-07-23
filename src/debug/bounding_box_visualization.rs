use bevy::{
    app::{Plugin, Update},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::Without,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    log::info,
    math::primitives::Rectangle,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
};
use bevy_world_space::{
    position::Position,
    win_info::WinInfo,
    world_unit::{AspectRatio, WorldVec2},
};

use crate::bounding_box::BoundingBox;

#[derive(Component)]
struct BoxSkeleton {
    parent_id: Entity,
    offset: WorldVec2,
}

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

        commands.spawn((
            BoxSkeleton {
                parent_id: id,
                offset: bounding_box.bounding_box().center(),
            },
            position,
            mesh_component,
            color_component,
        ));
    }
}

fn update_position(
    mut box_skeleton_query: Query<(&BoxSkeleton, &mut Position)>,
    parent_query: Query<&Position, Without<BoxSkeleton>>,
) {
    for (box_skeleton, mut position) in &mut box_skeleton_query {
        if let Ok(pos) = parent_query.get(box_skeleton.parent_id) {
            position.pos = pos.pos + box_skeleton.offset;
        }
    }
}
#[derive(Default)]
pub struct BoundingBoxVisualizationPlugin;
impl Plugin for BoundingBoxVisualizationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, toggle)
            .add_systems(Update, update_position);
    }
}
